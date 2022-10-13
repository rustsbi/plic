//! RISC-V Platform-Level Interrupt Controller

#![no_std]
pub mod plic;

/*

in PAC crates:

pub struct Peripherals {
    uart0: UART0,
    plic: PLIC,
}

// Base address = 0x4000_0000
// Maximum level priority bits = 3 (Highest = P7)
pub type PLIC = plic::Plic<0x4000_0000, 3>;
*/

use core::{
    convert::{TryFrom, TryInto},
    num::NonZeroU16,
};

/// Platform-Level Interrupt Controller
pub struct Plic<const P: usize, const B: usize>(pub(crate) ());

impl<const P: usize, const B: usize> Plic<P, B> {
    const PTR: *const plic::RegisterBlock = P as *const _;
}

impl<const P: usize, const B: usize> Plic<P, B> {
    /// Check if interrupt is enabled for context
    #[inline]
    pub fn is_enabled(context: usize, interrupt: impl Into<Nr>) -> bool {
        let irq_number = interrupt.into().index() as usize;
        unsafe {
            (*Self::PTR).enables[context].enable[irq_number / 32].read() & 1 << (irq_number % 32)
                != 0
        }
    }

    /// Enable interrupt for context
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because it can break mask-based critical sections
    #[inline]
    pub unsafe fn unmask(context: usize, interrupt: impl Into<Nr>) {
        let irq_number = interrupt.into().index() as usize;
        (*Self::PTR).enables[context].enable[irq_number / 32]
            .modify(|v| v | 1 << (irq_number % 32));
    }

    /// Disable interrupt for context
    #[inline]
    pub fn mask(context: usize, interrupt: impl Into<Nr>) {
        let irq_number = interrupt.into().index() as usize;
        unsafe {
            (*Self::PTR).enables[context].enable[irq_number / 32]
                .modify(|v| v & !(1 << (irq_number % 32)));
        }
    }

    /// Get interrupt priority
    #[inline]
    pub fn get_priority(interrupt: impl Into<Nr>) -> Priority<B> {
        let irq_number = interrupt.into().index() as usize;
        let bits = unsafe { (*Self::PTR).priority[irq_number].read() };
        Priority::from_bits(bits)
    }

    /// Set interrupt priority
    ///
    /// # Unsafety
    ///
    /// Changing priority levels can break priority-based critical sections
    /// and compromise memory safety.
    #[inline]
    pub unsafe fn set_priority(interrupt: impl Into<Nr>, prio: Priority<B>) {
        let irq_number = interrupt.into().index() as usize;
        (*Self::PTR).priority[irq_number].write(prio.into_bits());
    }

    /// Get threshold for context
    #[inline]
    pub fn get_threshold(context: usize) -> Priority<B> {
        let bits = unsafe { (*Self::PTR).contexts[context].threshold.read() };
        Priority::from_bits(bits)
    }

    /// Set threshold for context
    #[inline]
    pub unsafe fn set_threshold(context: usize, threshold: Priority<B>) {
        (*Self::PTR).contexts[context]
            .threshold
            .write(threshold.into_bits());
    }

    /// Claim interrupt (used by interrupt runtime)
    #[inline]
    pub fn claim(context: usize) -> Option<Nr> {
        let bits = unsafe { (*Self::PTR).contexts[context].claim.read() };
        <Nr as TryFrom<u32>>::try_from(bits).ok()
    }

    /// Complete interrupt (used by interrupt runtime)
    #[inline]
    pub fn complete(context: usize, interrupt: impl Into<Nr>) {
        let irq_number = interrupt.into().index() as u32;
        unsafe {
            (*Self::PTR).contexts[context].claim.write(irq_number);
        }
    }

    /// Checks if interrupt is pending
    #[inline]
    pub fn is_pending(interrupt: impl Into<Nr>) -> bool {
        let irq_number = interrupt.into().index() as usize;
        unsafe { (*Self::PTR).pending[irq_number / 32].read() & 1 << (irq_number % 32) != 0 }
    }
}

/// Interrupt number
///
/// Valid number range are from 1..=1023; interrupt source 0 does not exist.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Nr(NonZeroU16);

impl Nr {
    #[inline]
    fn index(&self) -> u16 {
        self.0.into()
    }
}

impl TryFrom<u16> for Nr {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(src: u16) -> Result<Nr, Self::Error> {
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl TryFrom<u32> for Nr {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(src: u32) -> Result<Nr, Self::Error> {
        let src: u16 = src.try_into()?;
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl TryFrom<usize> for Nr {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(src: usize) -> Result<Nr, Self::Error> {
        let src: u16 = src.try_into()?;
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl From<Nr> for u16 {
    #[inline]
    fn from(src: Nr) -> u16 {
        src.0.into()
    }
}

impl From<Nr> for u32 {
    #[inline]
    fn from(src: Nr) -> u32 {
        <u16 as From<Nr>>::from(src).into()
    }
}

impl From<Nr> for usize {
    #[inline]
    fn from(src: Nr) -> usize {
        <u16 as From<Nr>>::from(src).into()
    }
}

// todo: highest priority is vendor defined

/// Priority of an interrupt
///
/// Type parameter B means how many bits are supported in target implementation.
/// For example if B = 3, highest priority would be 7 or 2^3 - 1, lowest would be 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Priority<const B: usize>(pub(crate) u32);

impl<const B: usize> Priority<B> {
    /// Priority 0 means never interrupt
    #[inline]
    pub const fn never() -> Priority<B> {
        Priority(0)
    }
    /// Returns the lowest active priority, or priority 1.
    #[inline]
    pub const fn lowest() -> Priority<B> {
        Priority(1)
    }
    /// Returns the highest active priority, or priority (2 << B) - 1.
    #[inline]
    pub const fn highest() -> Priority<B> {
        if B == 32 {
            Priority(u32::MAX)
        } else {
            Priority((2 << B) - 1)
        }
    }
}

impl<const B: usize> Priority<B> {
    #[inline]
    fn into_bits(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn from_bits(prio: u32) -> Priority<B> {
        if B == 32 {
            return Priority(prio); // always legal for B == 32
        }
        if prio < (2 << B) {
            Priority(prio)
        } else {
            panic!("invalid priority")
        }
    }
}
