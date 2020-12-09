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
use crate::tlkm::tlkm_copy_cmd_from;
use crate::tlkm::tlkm_copy_cmd_to;
use crate::tlkm::tlkm_ioctl_copy_from;
use crate::tlkm::tlkm_ioctl_copy_to;
use crate::vfio::*;
use core::fmt::Debug;
use memmap::MmapMut;
use snafu::ResultExt;
use std::fs::File;
use std::os::unix::prelude::*;
use std::sync::Arc;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not transfer to device {}", source))]
    DMAToDevice { source: nix::Error },

    #[snafu(display("Could not transfer from device {}", source))]
    DMAFromDevice { source: nix::Error },

    #[snafu(display("Could not allocate DMA buffer {}", source))]
    DMABufferAllocate { source: nix::Error },

    #[snafu(display(
        "Transfer 0x{:x} - 0x{:x} outside of memory region 0x{:x}.",
        ptr,
        end,
        size
    ))]
    OutOfRange {
        ptr: DeviceAddress,
        end: DeviceAddress,
        size: DeviceSize,
    },

    #[snafu(display("Mutex has been poisoned"))]
    MutexError {},

    #[snafu(display("Failed flushing the memory for DirectDMA: {}", source))]
    FailedFlush { source: std::io::Error },

    #[snafu(display("Failed to mmap DMA buffer: {}", source))]
    FailedMMapDMA { source: std::io::Error },

    #[snafu(display("Error during interrupt handling: {}", source))]
    ErrorInterrupt { source: crate::interrupt::Error },

    #[snafu(display(
        "Got interrupt but outstanding buffers are empty. This should never happen."
    ))]
    TooManyInterrupts {},
}
type Result<T, E = Error> = std::result::Result<T, E>;

/// Specifies a method to interact with DMA methods
///
/// The methods will block and the transfer is assumed complete when they return.
pub trait DMAControl: Debug {
    fn copy_to(&self, data: &[u8], ptr: DeviceAddress) -> Result<()>;
    fn copy_from(&self, ptr: DeviceAddress, data: &mut [u8]) -> Result<()>;
}

#[derive(Debug, Getters)]
pub struct DriverDMA {
    tlkm_file: Arc<File>,
}

impl DriverDMA {
    pub fn new(tlkm_file: &Arc<File>) -> DriverDMA {
        DriverDMA {
            tlkm_file: tlkm_file.clone(),
        }
    }
}

/// Use TLKM IOCTLs to transfer data
///
/// Is currently used for Zynq based devices.
impl DMAControl for DriverDMA {
    fn copy_to(&self, data: &[u8], ptr: DeviceAddress) -> Result<()> {
        trace!(
            "Copy Host({:?}) -> Device(0x{:x}) ({} Bytes)",
            data.as_ptr(),
            ptr,
            data.len()
        );
        unsafe {
            tlkm_ioctl_copy_to(
                self.tlkm_file.as_raw_fd(),
                &mut tlkm_copy_cmd_to {
                    dev_addr: ptr,
                    length: data.len(),
                    user_addr: data.as_ptr(),
                },
            )
            .context(DMAToDevice)?;
        };
        Ok(())
    }

    fn copy_from(&self, ptr: DeviceAddress, data: &mut [u8]) -> Result<()> {
        trace!(
            "Copy Device(0x{:x}) -> Host({:?}) ({} Bytes)",
            ptr,
            data.as_mut_ptr(),
            data.len()
        );
        unsafe {
            tlkm_ioctl_copy_from(
                self.tlkm_file.as_raw_fd(),
                &mut tlkm_copy_cmd_from {
                    dev_addr: ptr,
                    length: data.len(),
                    user_addr: data.as_mut_ptr(),
                },
            )
            .context(DMAFromDevice)?;
        };
        Ok(())
    }
}

#[derive(Debug, Getters)]
pub struct VfioDMA {
    tlkm_file: Arc<File>,
    vfio_dev: Arc<VfioDev>,
}

impl VfioDMA {
    pub fn new(tlkm_file: &Arc<File>, vfio_dev: &Arc<VfioDev>) -> VfioDMA {
        VfioDMA {
            tlkm_file: tlkm_file.clone(),
            vfio_dev: vfio_dev.clone(),
        }
    }
}

/// Use VFIO to give PE access to virtual addresses using the SMMU
///
/// Is currently used for ZynqMP based devices.
impl DMAControl for VfioDMA {
    fn copy_to(&self, data: &[u8], ptr: DeviceAddress) -> Result<()> {
        error!(
            "WHOOOP!  Copy Host({:?}) -> Device(0x{:x}) ({} Bytes)",
            data.as_ptr(),
            ptr,
            data.len()
        );
    
        let pagesize = 4096;
        let pages = data.len() / pagesize + 1; // round to next highest page boundary
        let map_len = pages * pagesize;

        // obviously this needs to be fixed, cannot copy buffer every time
        let mut src_buf = MmapMut::map_anon(map_len).unwrap();
        src_buf[0..data.len()].copy_from_slice(data);

        // for i in 0..20 {
        //     error!("Content: orig {} cpy {}", data[i], src_buf[i]);
        // }

        error!("Allocating {} bytes for va 0x{:x} iova 0x{:x}", map_len, src_buf.as_ptr() as u64, ptr);
        vfio_dma_map(&self.vfio_dev, map_len as u64, ptr, src_buf.as_ptr() as u64);

        Ok(())
    }

    fn copy_from(&self, ptr: DeviceAddress, data: &mut [u8]) -> Result<()> {
        error!(
            "WHOOP!   Copy Device(0x{:x}) -> Host({:?}) ({} Bytes)",
            ptr,
            data.as_mut_ptr(),
            data.len()
        );
        unsafe {
            tlkm_ioctl_copy_from(
                self.tlkm_file.as_raw_fd(),
                &mut tlkm_copy_cmd_from {
                    dev_addr: ptr,
                    length: data.len(),
                    user_addr: data.as_mut_ptr(),
                },
            )
            .context(DMAFromDevice)?;
        };
        Ok(())
    }
}



/// Use the CPU to transfer data
///
/// Can be used for all memory that is directly accessible by the host.
/// This might be BAR mapped device off-chip-memory or PE local memory.
/// The latter is the main use case for this kind of transfer.
#[derive(Debug, Getters)]
pub struct DirectDMA {
    offset: DeviceAddress,
    size: DeviceSize,
    memory: Arc<MmapMut>,
}

impl DirectDMA {
    pub fn new(offset: DeviceAddress, size: DeviceSize, memory: Arc<MmapMut>) -> DirectDMA {
        DirectDMA {
            offset: offset,
            size: size,
            memory: memory,
        }
    }
}

impl DMAControl for DirectDMA {
    fn copy_to(&self, data: &[u8], ptr: DeviceAddress) -> Result<()> {
        let end = ptr + data.len() as u64;
        if end > self.size {
            return Err(Error::OutOfRange {
                ptr: ptr,
                end: end,
                size: self.size,
            });
        }

        trace!(
            "Copy Host -> Device(0x{:x} + 0x{:x}) ({} Bytes)",
            self.offset,
            ptr,
            data.len()
        );

        // This is necessary as MMapMut is Protected by an Arc but access without
        // an explicit lock is necessary
        // Locking the mmap would slow down PE start etc too much
        unsafe {
            let p = self.memory.as_ptr().offset((self.offset + ptr) as isize) as *mut u8;
            let s = std::ptr::slice_from_raw_parts_mut(p, data.len());
            (*s).clone_from_slice(&data[..]);
        }

        Ok(())
    }

    fn copy_from(&self, ptr: DeviceAddress, data: &mut [u8]) -> Result<()> {
        let end = ptr + data.len() as u64;
        if end > self.size {
            return Err(Error::OutOfRange {
                ptr: ptr,
                end: end,
                size: self.size,
            });
        }

        trace!(
            "Copy Device(0x{:x} + 0x{:x}) -> Host ({} Bytes)",
            self.offset,
            ptr,
            data.len()
        );

        data[..].clone_from_slice(
            &self.memory[(self.offset + ptr) as usize..(self.offset + end) as usize],
        );

        Ok(())
    }
}
