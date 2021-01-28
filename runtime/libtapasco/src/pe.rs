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

use crate::debug::DebugControl;
use crate::device::DataTransferPrealloc;
use crate::device::DeviceAddress;
use crate::device::DeviceSize;
use crate::device::OffchipMemory;
use crate::device::PEParameter;
use crate::interrupt::Interrupt;
use memmap::MmapMut;
use snafu::ResultExt;
use std::fs::File;
use std::sync::Arc;
use volatile::Volatile;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "Param {:?} is unsupported. Only 32 and 64 Bit value can interact with device registers.",
        param
    ))]
    UnsupportedParameter { param: PEParameter },

    #[snafu(display(
        "Transfer width {} Bit is unsupported. Only 32 and 64 Bit value can interact with device registers.",
        param * 8
    ))]
    UnsupportedRegisterSize { param: usize },

    #[snafu(display("PE {} is already running.", id))]
    PEAlreadyActive { id: usize },

    #[snafu(display("Could not read completion file: {}", source))]
    ReadCompletionError { source: std::io::Error },

    #[snafu(display("Could not insert PE {} into active PE set.", pe_id))]
    CouldNotInsertPE { pe_id: usize },

    #[snafu(display("Error during interrupt handling: {}", source))]
    ErrorInterrupt { source: crate::interrupt::Error },

    #[snafu(display("Error creating interrupt eventfd: {}", source))]
    ErrorEventFD { source: nix::Error },

    #[snafu(display("Error reading interrupt eventfd: {}", source))]
    ErrorEventFDRead { source: nix::Error },

    #[snafu(display("Could not register eventfd with driver: {}", source))]
    ErrorEventFDRegister { source: nix::Error },

    #[snafu(display("Failed to enable debug for PE {}: {}", id, source))]
    DebugError {
        source: crate::debug::Error,
        id: usize,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum CopyBack {
    Transfer(DataTransferPrealloc),
    Free(DeviceAddress, Arc<OffchipMemory>),
}

pub type PEId = usize;

/// Representation of a TaPaSCo PE
///
/// Supports starting and releasing a PE as well as
/// interacting with its registers.
/// Stores information of attached memory for copy back
/// operations after PE execution.
#[derive(Debug, Getters, Setters)]
pub struct PE {
    #[get = "pub"]
    id: usize,
    #[get = "pub"]
    type_id: PEId,
    pub offset: DeviceAddress, // HACK to enable FFI iface to write arbitrary addresses
    size: DeviceSize,
    name: String,
    #[get = "pub"]
    active: bool,
    copy_back: Option<Vec<CopyBack>>,
    pub memory: Arc<MmapMut>, // HACK to enable FFI iface to write arbitrary addresses

    #[set = "pub"]
    #[get = "pub"]
    local_memory: Option<Arc<OffchipMemory>>,

    interrupt: Interrupt,

    debug: Box<dyn DebugControl + Sync + Send>,
}

impl PE {
    pub fn new(
        id: usize,
        type_id: PEId,
        offset: DeviceAddress,
        size: DeviceSize,
        name: String,
        memory: Arc<MmapMut>,
        completion: &File,
        interrupt_id: usize,
        debug: Box<dyn DebugControl + Sync + Send>,
    ) -> Result<PE> {
        Ok(PE {
            id: id,
            type_id: type_id,
            offset: offset,
            size: size,
            name: name,
            active: false,
            copy_back: None,
            memory: memory,
            local_memory: None,
            interrupt: Interrupt::new(completion, interrupt_id, false).context(ErrorInterrupt)?,
            debug: debug,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        ensure!(!self.active, PEAlreadyActive { id: self.id });
        trace!("Starting PE {}.", self.id);
        let offset = self.offset as isize;
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).write(1);
        }
        self.active = true;
        Ok(())
    }

    pub fn release(&mut self, return_value: bool) -> Result<(u64, Option<Vec<CopyBack>>)> {
        trace!(
            "Waiting for PE {} to complete processing (interrupt signal).",
            self.id
        );
        self.wait_for_completion()?;
        trace!("PE {} done.", self.id);
        let rv = if return_value { self.return_value() } else { 0 };
        Ok((rv, self.get_copyback()))
    }

    /// Waits for a PE interrupt and deactivates the PE afterwards
    fn wait_for_completion(&mut self) -> Result<()> {
        if self.active {
            self.interrupt
                .wait_for_interrupt()
                .context(ErrorInterrupt)?;
            trace!("Cleaning up PE {} after release.", self.id);
            self.active = false;
            self.reset_interrupt(true)?;
        } else {
            trace!("Wait requested but {:?} is already idle.", self.id);
        }
        Ok(())
    }

    pub fn interrupt_set(&self) -> Result<bool> {
        let offset = (self.offset as usize + 0x0c) as isize;
        let r = unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).read()
        };
        let s = (r & 1) == 1;
        trace!("Reading interrupt status from 0x{:x} -> {}", offset, s);
        Ok(s)
    }

    pub fn reset_interrupt(&self, v: bool) -> Result<()> {
        let offset = (self.offset as usize + 0x0c) as isize;
        trace!("Resetting interrupts: 0x{:x} -> {}", offset, v);
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).write(if v { 1 } else { 0 });
        }
        Ok(())
    }

    pub fn interrupt_status(&self) -> Result<(bool, bool)> {
        let mut offset = (self.offset as usize + 0x04) as isize;
        let g = unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).read()
        } & 1
            == 1;
        offset = (self.offset as usize + 0x08) as isize;
        let l = unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).read()
        } & 1
            == 1;
        trace!("Interrupt status is {}, {}", g, l);
        Ok((g, l))
    }

    pub fn enable_interrupt(&self) -> Result<()> {
        ensure!(!self.active, PEAlreadyActive { id: self.id });
        let mut offset = (self.offset as usize + 0x04) as isize;
        trace!("Enabling interrupts: 0x{:x} -> 1", offset);
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).write(1);
        }
        offset = (self.offset as usize + 0x08) as isize;
        trace!("Enabling global interrupts: 0x{:x} -> 1", offset);
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            let volatile_ptr = ptr as *mut Volatile<u32>;
            (*volatile_ptr).write(1);
        }
        Ok(())
    }

    pub fn set_arg(&self, argn: usize, arg: PEParameter) -> Result<()> {
        let offset = (self.offset as usize + 0x20 + argn * 0x10) as isize;
        trace!("Writing argument: 0x{:x} ({}) -> {:?}", offset, argn, arg);
        unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            match arg {
                PEParameter::Single32(x) => (*(ptr as *mut Volatile<u32>)).write(x),
                PEParameter::Single64(x) => (*(ptr as *mut Volatile<u64>)).write(x),
                _ => return Err(Error::UnsupportedParameter { param: arg }),
            };
        }
        Ok(())
    }

    pub fn read_arg(&self, argn: usize, bytes: usize) -> Result<PEParameter> {
        let offset = (self.offset as usize + 0x20 + argn * 0x10) as isize;
        let r = unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            match bytes {
                4 => Ok(PEParameter::Single32(
                    (*(ptr as *const Volatile<u32>)).read(),
                )),
                8 => Ok(PEParameter::Single64(
                    (*(ptr as *const Volatile<u64>)).read(),
                )),
                _ => Err(Error::UnsupportedRegisterSize { param: bytes }),
            }
        };
        trace!(
            "Reading argument: 0x{:x} ({} x {}B) -> {:?}",
            offset,
            argn,
            bytes,
            r
        );
        r
    }

    pub fn return_value(&self) -> u64 {
        let offset = (self.offset as usize + 0x10) as isize;
        let r = unsafe {
            let ptr = self.memory.as_ptr().offset(offset);
            (*(ptr as *const Volatile<u64>)).read()
        };
        trace!("Reading return value: {}", r);
        r
    }

    pub fn add_copyback(&mut self, param: CopyBack) {
        self.copy_back.get_or_insert(Vec::new()).push(param);
    }

    fn get_copyback(&mut self) -> Option<Vec<CopyBack>> {
        self.copy_back.take()
    }

    pub fn enable_debug(&mut self) -> Result<()> {
        self.debug
            .enable_debug()
            .context(DebugError { id: self.id })
    }
}
