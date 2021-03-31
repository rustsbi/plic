//! Platform-Level Interrupt Controller
//!
//! Ref: [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)

use volatile_register::RW;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x000000 - Interrupt source priority
    ///
    /// base + 0x000000: Reserved (interrupt source 0 does not exist)
    /// base + 0x000004: Interrupt source 1 priority
    /// base + 0x000008: Interrupt source 2 priority
    /// ...
    /// base + 0x000FFC: Interrupt source 1023 priority
    pub priority: [RW<u32>; 1024],
    /// 0x001000 - Interrupt pending
    ///
    /// base + 0x001000: Interrupt Pending bit 0-31
    /// base + 0x00107C: Interrupt Pending bit 992-1023
    pub pending: [RW<u32>; 128],
    _padding1: [u32; 896],
    /// 0x002000 - Enable bits for sources on contexts
    ///
    /// base + 0x002000: Enable bits for sources 0-31 on context 0
    /// base + 0x002004: Enable bits for sources 32-63 on context 0
    /// ...
    /// base + 0x00207F: Enable bits for sources 992-1023 on context 0
    /// base + 0x002080: Enable bits for sources 0-31 on context 1
    /// base + 0x002084: Enable bits for sources 32-63 on context 1
    /// ...
    /// base + 0x0020FF: Enable bits for sources 992-1023 on context 1
    /// base + 0x002100: Enable bits for sources 0-31 on context 2
    /// base + 0x002104: Enable bits for sources 32-63 on context 2
    /// ...
    /// base + 0x00217F: Enable bits for sources 992-1023 on context 2
    /// ...
    /// base + 0x1F1F80: Enable bits for sources 0-31 on context 15871
    /// base + 0x1F1F84: Enable bits for sources 32-63 on context 15871
    /// base + 0x1F1FFF: Enable bits for sources 992-1023 on context 15871
    /// ...
    pub enables: [Enables; 15872],
    _padding2: [u32; 1792],
    /// 0x200000 - Context configurations
    /// 
    /// base + 0x200000: Priority threshold for context 0
    /// base + 0x200004: Claim/complete for context 0
    /// base + 0x200008: Reserved
    /// ...
    /// base + 0x200FFC: Reserved
    /// base + 0x201000: Priority threshold for context 1
    /// base + 0x201004: Claim/complete for context 1
    /// ...
    /// base + 0x3FFE000: Priority threshold for context 15871
    /// base + 0x3FFE004: Claim/complete for context 15871
    /// base + 0x3FFE008: Reserved
    pub contexts: [Contexts; 15872],
}

/// 0x002000 - Enable bits for sources
#[repr(C)]
pub struct Enables {
    /// 0x000: Enable bits for sources
    pub enable: [RW<u32>; 32],
}

/// 0x200000 - Context configurations
#[repr(C)]
pub struct Contexts {
    /// 0x000: Priority threshold for context
    pub threshold: RW<u32>,
    /// 0x004: Claim/complete for context
    pub claim: RW<u32>,
    _reserved: [u32; 1022],
}
