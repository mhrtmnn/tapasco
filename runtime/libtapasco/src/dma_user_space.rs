/*
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo
 * (see https://github.com/esa-tu-darmstadt/tapasco).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::device::DeviceAddress;
use crate::device::DeviceSize;
use crate::dma::DMABufferAllocate;
use crate::dma::DMAControl;
use crate::dma::Error;
use crate::dma::ErrorInterrupt;
use crate::dma::FailedMMapDMA;
use crate::interrupt::Interrupt;
use crate::tlkm::tlkm_dma_buffer_allocate;
use crate::tlkm::tlkm_ioctl_dma_buffer_allocate;
use core::fmt::Debug;
use crossbeam::deque::{Injector, Steal};
use memmap::MmapMut;
use memmap::MmapOptions;
use snafu::ResultExt;
use std::fs::File;
use std::os::unix::prelude::*;
use std::sync::Arc;
use volatile::Volatile;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
struct DMABuffer {
    id: usize,
    size: usize,
    addr: u64,
    mapped: MmapMut,
}

#[derive(Debug, Getters)]
pub struct UserSpaceDMA {
    tlkm_file: Arc<File>,
    memory: Arc<MmapMut>,
    engine_offset: usize,
    to_dev_buffer: Injector<DMABuffer>,
    from_dev_buffer: Injector<DMABuffer>,
    read_int: Interrupt,
    write_int: Interrupt,
}

impl UserSpaceDMA {
    pub fn new(
        tlkm_file: &Arc<File>,
        offset: usize,
        read_interrupt: usize,
        write_interrupt: usize,
        memory: &Arc<MmapMut>,
    ) -> Result<UserSpaceDMA> {
        let buf_size = 256 * 1024;
        let mut to_dev_buf = tlkm_dma_buffer_allocate {
            size: buf_size,
            from_device: false,
            buffer_id: 42,
            addr: 42,
        };
        unsafe {
            tlkm_ioctl_dma_buffer_allocate(tlkm_file.as_raw_fd(), &mut to_dev_buf)
                .context(DMABufferAllocate)?;
        };

        let mut from_dev_buf = tlkm_dma_buffer_allocate {
            size: buf_size,
            from_device: true,
            buffer_id: 42,
            addr: 42,
        };
        unsafe {
            tlkm_ioctl_dma_buffer_allocate(tlkm_file.as_raw_fd(), &mut from_dev_buf)
                .context(DMABufferAllocate)?;
        };

        let read_map = Injector::new();
        read_map.push(DMABuffer {
            id: from_dev_buf.buffer_id,
            addr: from_dev_buf.addr,
            size: buf_size,
            mapped: unsafe {
                MmapOptions::new()
                    .len(buf_size)
                    .offset(((4 + from_dev_buf.buffer_id) * 4096) as u64)
                    .map_mut(&tlkm_file)
                    .context(FailedMMapDMA)?
            },
        });

        let write_map = Injector::new();
        write_map.push(DMABuffer {
            id: to_dev_buf.buffer_id,
            addr: to_dev_buf.addr,
            size: buf_size,
            mapped: unsafe {
                MmapOptions::new()
                    .len(buf_size)
                    .offset(((4 + to_dev_buf.buffer_id) * 4096) as u64)
                    .map_mut(&tlkm_file)
                    .context(FailedMMapDMA)?
            },
        });

        Ok(UserSpaceDMA {
            tlkm_file: tlkm_file.clone(),
            memory: memory.clone(),
            engine_offset: offset,
            to_dev_buffer: write_map,
            from_dev_buffer: read_map,
            read_int: Interrupt::new(tlkm_file, read_interrupt, false, false)
                .context(ErrorInterrupt)?,
            write_int: Interrupt::new(tlkm_file, write_interrupt, false, false)
                .context(ErrorInterrupt)?,
        })
    }

    fn schedule_dma_transfer(
        &self,
        addr_host: u64,
        addr_device: DeviceAddress,
        size: DeviceSize,
        from_device: bool,
    ) -> Result<()> {
        let mut offset = (self.engine_offset as usize + 0x00) as isize;
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u64>;
            (*volatile_ptr).write(addr_host);
        };

        offset = (self.engine_offset as usize + 0x08) as isize;
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u64>;
            (*volatile_ptr).write(addr_device);
        };

        offset = (self.engine_offset as usize + 0x10) as isize;
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u64>;
            (*volatile_ptr).write(size);
        };

        offset = (self.engine_offset as usize + 0x20) as isize;
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u64>;
            (*volatile_ptr).write(if from_device { 0x10001000 } else { 0x10000001 });
        };

        Ok(())
    }
}

impl DMAControl for UserSpaceDMA {
    fn copy_to(&self, data: &[u8], ptr: DeviceAddress) -> Result<()> {
        trace!(
            "Copy Host({:?}) -> Device(0x{:x}) ({} Bytes)",
            data.as_ptr(),
            ptr,
            data.len()
        );

        let mut ptr_buffer = 0;
        let mut ptr_device = ptr;
        let mut btt = data.len();

        let mut buffer = loop {
            match self.to_dev_buffer.steal() {
                Steal::Success(buffer) => break buffer,
                Steal::Empty => (),
                Steal::Retry => (),
            }
        };

        while btt > 0 {
            let btt_this = if btt < buffer.size { btt } else { buffer.size };

            buffer.mapped[0..btt_this].copy_from_slice(&data[ptr_buffer..ptr_buffer + btt_this]);

            self.schedule_dma_transfer(buffer.addr, ptr_device, btt_this as u64, false)?;

            btt -= btt_this;
            ptr_buffer += btt_this;
            ptr_device += btt_this as u64;

            self.write_int
                .wait_for_interrupt()
                .context(ErrorInterrupt)?;
        }

        self.to_dev_buffer.push(buffer);
        Ok(())
    }

    fn copy_from(&self, ptr: DeviceAddress, data: &mut [u8]) -> Result<()> {
        trace!(
            "Copy Device(0x{:x}) -> Host({:?}) ({} Bytes)",
            ptr,
            data.as_mut_ptr(),
            data.len()
        );

        let mut ptr_buffer = 0;
        let mut ptr_device = ptr;
        let mut btt = data.len();

        let buffer = loop {
            match self.from_dev_buffer.steal() {
                Steal::Success(buffer) => break buffer,
                Steal::Empty => (),
                Steal::Retry => (),
            }
        };

        while btt > 0 {
            let btt_this = if btt < buffer.size { btt } else { buffer.size };

            self.schedule_dma_transfer(buffer.addr, ptr_device, btt_this as u64, true)?;

            self.read_int.wait_for_interrupt().context(ErrorInterrupt)?;

            data[ptr_buffer..ptr_buffer + btt_this].copy_from_slice(&buffer.mapped[0..btt_this]);

            btt -= btt_this;
            ptr_buffer += btt_this;
            ptr_device += btt_this as u64;
        }

        self.from_dev_buffer.push(buffer);

        Ok(())
    }
}