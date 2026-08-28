// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! QEMU VirtIO MMIO instantiation

use kernel::utilities::StaticRef;
use virtio::transports::mmio::VirtIOMMIODeviceRegisters;

pub fn virtio_mmio_0_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_1000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_1_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_2000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_2_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_3000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_3_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_4000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_4_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_5000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_5_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_6000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_6_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_7000 as *const VirtIOMMIODeviceRegisters) }
}
pub fn virtio_mmio_7_base() -> StaticRef<VirtIOMMIODeviceRegisters> {
    unsafe { StaticRef::new(0x1000_8000 as *const VirtIOMMIODeviceRegisters) }
}
