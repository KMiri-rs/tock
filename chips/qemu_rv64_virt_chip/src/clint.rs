// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Machine Timer instantiation.

use kernel::utilities::StaticRef;
use sifive::clint::ClintRegisters;

pub type StaticRefClintRegisters = StaticRef<ClintRegisters>;
pub fn clint_base() -> StaticRefClintRegisters {
    unsafe { StaticRef::new(0x0200_0000 as *const ClintRegisters) }
}
