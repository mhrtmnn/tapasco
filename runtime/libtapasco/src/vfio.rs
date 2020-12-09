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

use vfio_bindings::bindings::vfio::*;
use std::ffi::CString;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::Read;
use memmap::MmapMut;
use snafu::ResultExt;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::fs::File;
use std::sync::Mutex;



// VFIO ioctl definition
ioctl_none_bad!(
    vfio_get_api_version,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 0)
);

ioctl_write_int_bad!(
    vfio_check_extension,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 1)
);

ioctl_read_bad!(
    vfio_group_get_status,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 3),
    vfio_group_status
);

ioctl_write_int_bad!(
    vfio_set_iommu,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 2)
);

ioctl_write_ptr_bad!(
    vfio_group_set_container,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 4),
    i32
);

ioctl_write_ptr_bad!(
    vfio_group_get_device_fd,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 6),
    u8
);

ioctl_read_bad!(
    vfio_device_get_info,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 7),
    vfio_device_info
);

ioctl_readwrite_bad!(
    vfio_device_get_region_info,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 8),
    vfio_region_info
);

ioctl_read_bad!(
    vfio_iommu_get_info,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 12),
    vfio_iommu_type1_info
);

ioctl_write_ptr_bad!(
    vfio_iommu_map_dma,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 13),
    vfio_iommu_type1_dma_map
);

ioctl_readwrite_bad!(
    vfio_iommu_unmap_dma,
    request_code_none!(VFIO_TYPE, VFIO_BASE + 14),
    vfio_iommu_type1_dma_unmap
);


// ioctl_none!(
//     vfio_get_api_version,
//     VFIO_TYPE,
//     VFIO_BASE + 0
// );

// ioctl_none!(
//     vfio_check_extension,
//     VFIO_TYPE,
//     VFIO_BASE + 1
// );

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not open {}: {}", file, source))]
    VfioOpen {
        source: std::io::Error,
        file: String
    },

}

#[derive(Debug)]
pub struct VfioMapping {
    pub iova: u64,
    pub size: u64
}

#[derive(Debug)]
pub struct VfioDev {
    container: File,
    group: File,
    device: File,
    pub mappings: Mutex<Vec<VfioMapping>>
}

pub fn init_vfio(vfio_group: u32) -> Result<VfioDev, Error> {
    error!("Initializing VFIO!");
    let container_path = "/dev/vfio/vfio";

    let container = OpenOptions::new()
        .read(true)
        .write(true)
        .open(container_path)
        .context(VfioOpen { file: container_path })?;

    let mut ret = unsafe { vfio_get_api_version(container.as_raw_fd()) }.unwrap();
    if ret != VFIO_API_VERSION as i32 {
        error!("VFIO version is {} should be {}", ret, VFIO_API_VERSION);
    } else {
        error!("VFIO version is {}, okay!", ret);
    }

    ret = unsafe { vfio_check_extension(container.as_raw_fd(), VFIO_TYPE1_IOMMU as i32) }.unwrap();
    if ret == 0 {
        error!("VFIO_TYPE1_IOMMU not supported");
    } else {
        error!("VFIO_TYPE1_IOMMU okay!");
    }

    // this will fail if vfio driver not yet loaded (or group number changed)
    let group_path = format!("/dev/vfio/{}", vfio_group);
    let group = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&group_path)
        .context(VfioOpen { file: &group_path })?;

    let mut group_status = vfio_group_status {
        argsz: std::mem::size_of::<vfio_group_status>() as u32,
        flags: 0,
    };
    ret = unsafe { vfio_group_get_status(group.as_raw_fd(), &mut group_status) }.unwrap();
    if ret < 0 {
        error!("vfio_group_get_status failed!\n");
    } else if (group_status.flags & VFIO_GROUP_FLAGS_VIABLE) != VFIO_GROUP_FLAGS_VIABLE {
        error!("Group is not viable (not all devices bound for vfio)\n");
    } else {
        error!("Group is okay!\n");
    }

    ret = unsafe { vfio_group_set_container(group.as_raw_fd(), &container.as_raw_fd()) }.unwrap();
    if ret < 0 {
        error!("Set container failed!\n");
    } else {
        error!("Set container okay!\n");
    }

    ret = unsafe { vfio_set_iommu(container.as_raw_fd(), VFIO_TYPE1_IOMMU as i32) }.unwrap();
    if ret < 0 {
        error!("vfio_set_iommu failed!\n");
    } else {
        error!("vfio_set_iommu okay!\n");
    }

    let dma_device = CString::new("tapasco").unwrap();
    let dev_fd = unsafe { vfio_group_get_device_fd(group.as_raw_fd(), dma_device.as_ptr()) }.unwrap();
    if dev_fd < 0 {
        error!("vfio_group_get_device_fd dst failed!\n");
    } else {
        error!("vfio_group_get_device_fd dst okay! fd={}\n", dev_fd);
    }

    let dev = VfioDev{
        container,
        group,
        device: unsafe { File::from_raw_fd(dev_fd) },
        mappings: Mutex::new((Vec::new()))
    };
    Ok(dev)
}


pub fn vfio_get_info(dev: &VfioDev) {
    let mut iommu_info = vfio_iommu_type1_info {
        argsz: std::mem::size_of::<vfio_iommu_type1_info>() as u32,
        flags: 0,
        iova_pgsizes: 0,
    };
    let ret = unsafe { vfio_iommu_get_info(dev.container.as_raw_fd(), &mut iommu_info) }.unwrap();
    if ret < 0 {
        error!("vfio_iommu_get_info failed!\n");
    } else {
        error!("vfio_iommu_get_info okay!\n");
        error!("flags={}, PS={}!\n", iommu_info.flags, iommu_info.iova_pgsizes);
    }
}


pub fn vfio_test(dev: &VfioDev) {
    let size_to_map = 4096;

    /* map src buffer */
    let mut src_buf = MmapMut::map_anon(size_to_map as usize).unwrap();
    vfio_dma_map(&dev, size_to_map, 0, src_buf.as_ptr() as u64);

    /* map dst buffer */
    let dst_buf = MmapMut::map_anon(size_to_map as usize).unwrap();
    vfio_dma_map(&dev, size_to_map, 3*size_to_map, dst_buf.as_ptr() as u64);


    // array comp
    let mut small_rng = StdRng::from_entropy();

    let mut result: Vec<u8> = Vec::new();
    for i in 0..4096 {
        let x = small_rng.gen();
        result.push(x);
        src_buf[i] = x;
    }

    error!("Checking mmap src ...");
    let mut errors = 0;
    for i in 0..4096 {
        if result[i] != src_buf[i] {
            errors+=1;
        }
    }

    errors = 0;
    for i in 0..4096 {
        if dst_buf[i] != src_buf[i] {
            errors+=1;
        }
    }
    error!("There were {} errors (expect=4096)!", errors);

    let _input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);


    errors = 0;
    for i in 0..4096 {
        if dst_buf[i] != src_buf[i] {
            errors+=1;
        }
    }
    error!("There were {} errors (expect=0)!", errors);

    vfio_dma_unmap(&dev, 0, size_to_map);
    vfio_dma_unmap(&dev, 3*size_to_map, size_to_map);
}


pub fn vfio_dma_map(dev: &VfioDev, size: u64, iova: u64, vaddr: u64) {
    // is missing MAP_PRIVATE flag. Needed?
    let dma_map_src = vfio_iommu_type1_dma_map {
        argsz: std::mem::size_of::<vfio_iommu_type1_dma_map>() as u32,
        flags: VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE,
        vaddr,
        iova,
        size,
    };

    let ret = unsafe { vfio_iommu_map_dma(dev.container.as_raw_fd(), &dma_map_src) }.unwrap();
    if ret < 0 {
        error!("vfio_iommu_map_dma src failed!\n");
    } else {
        error!("vfio_iommu_map_dma src okay!\n");
    }
}

pub fn vfio_dma_unmap(dev: &VfioDev, iova: u64, size: u64) {
    let mut dma_unmap = vfio_iommu_type1_dma_unmap {
        argsz: std::mem::size_of::<vfio_iommu_type1_dma_unmap>() as u32,
        flags: 0,
        iova,
        size,
    };

    let ret = unsafe { vfio_iommu_unmap_dma(dev.container.as_raw_fd(), &mut dma_unmap) }.unwrap();
    if ret < 0 {
        error!("vfio_iommu_unmap_dma src failed!\n");
    } else {
        error!("vfio_iommu_unmap_dma src okay! Unmapped regio of size 0x{:x}\n", dma_unmap.size);
    }
}


pub fn vfio_get_region_info(dev: &VfioDev) {
    let mut dev_info = vfio_device_info {
        argsz: std::mem::size_of::<vfio_device_info>() as u32,
        flags: 0,
        num_regions: 0,
        num_irqs: 0,
    };

    let ret = unsafe { vfio_device_get_info(dev.device.as_raw_fd(), &mut dev_info) }.unwrap();
    if ret < 0 {
        error!("vfio_device_get_info dst failed!\n");
    } else {
        error!("vfio_device_get_info dst okay!\n");
        error!("Dev has {} regions\n", dev_info.num_regions);
    }

    // get info of all regions
    for r in 0..dev_info.num_regions {
        let mut reg_info = vfio_region_info {
            argsz: std::mem::size_of::<vfio_region_info>() as u32,
            flags: 0,
            index: r,
            cap_offset: 0,
            size: 0,
            offset: 0,
        };

        let ret = unsafe { vfio_device_get_region_info(dev.device.as_raw_fd(), &mut reg_info) }.unwrap();
        if ret < 0 {
            error!("vfio_device_get_region_info failed!\n");
        } else {
            error!("vfio_device_get_region_info okay!\n");
            error!("sz=0x{:x}, offs=0x{:x}\n", reg_info.size, reg_info.offset);
        }
    }

    // open device regs
    // let df = unsafe { File::from_raw_fd(dev_fd) };
    // let mmap = unsafe { MmapOptions::new().map(&df).unwrap().make_mut() };
    // error!("Dev: MM2S_STATUS_REGISTER = 0x{:x}", mmap.unwrap()[4]);
}
