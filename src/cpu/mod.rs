/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2022 Advanced Micro Devices, Inc.
 *
 * Authors: Carlos Bilbao <carlos.bilbao@amd.com> and
 *          Tom Lendacky <thomas.lendacky@amd.com>
 */

/// Handle CpuidPages and their entries.
pub mod cpuid;
/// Create IDT and handle exceptions
pub mod idt;
/// Handle per-vCPU information (Vmsa and Caa)
pub mod percpu;
/// Initialize and start SMP
pub mod smp;
/// Auxiliary assembly functions
pub mod sys;
/// VC functions
pub mod vc;
/// Vmsa (Virtual Machine Saving Area) support
pub mod vmsa;

pub use crate::cpu::idt::*;
pub use crate::cpu::percpu::*;
pub use crate::cpu::smp::*;
pub use crate::cpu::sys::*;
pub use crate::cpu::vc::*;
