//! RISC-V Platform-Level Interrupt Controller

#![no_std]
pub mod plic;

/*

in PAC crates: 

pub struct Peripherals {
    uart0: UART0,
    plic: PLIC,
}

pub type PLIC = plic::Plic<0x4000_0000>;
*/

use core::{convert::{TryFrom, TryInto}, num::NonZeroU16};

/// Platform-Level Interrupt Controller
pub struct Plic<const B: usize>(());

impl<const B: usize> Plic<B> {
    const PTR: *const plic::RegisterBlock = B as *const _;
}

impl<const B: usize> Plic<B> {
    /// Check if interrupt is enabled for context
    pub fn is_enabled(context: usize, interrupt: Nr) -> bool {
        let irq_number = interrupt.index() as usize;
        unsafe {
            (*Self::PTR).enables[context].enable[irq_number / 32]
                .read() & 1 << (irq_number % 32) != 0
        }
    }

    /// Enable interrupt for context
    ///
    /// # Unsafety
    ///
    /// This function is unsafe because it can break mask-based critical sections
    pub unsafe fn unmask(context: usize, interrupt: Nr) {
        let irq_number = interrupt.index() as usize;
        (*Self::PTR).enables[context].enable[irq_number / 32]
            .modify(|v| v | 1 << (irq_number % 32));
    }

    /// Disable interrupt for context
    pub fn mask(context: usize, interrupt: Nr) { 
        let irq_number = interrupt.index() as usize;
        unsafe {
            (*Self::PTR).enables[context].enable[irq_number / 32]
                .modify(|v| v & !(1 << (irq_number % 32)));
        }
    }

    /// Get interrupt priority
    pub fn get_priority(interrupt: Nr) -> Priority { 
        let irq_number = interrupt.index() as usize;
        let bits = unsafe {
            (*Self::PTR).priority[irq_number].read() 
        };
        Priority::from_bits(bits)
    }

    /// Set interrupt priority
    ///
    /// # Unsafety 
    /// 
    /// Changing priority levels can break priority-based critical sections 
    /// and compromise memory safety.
    pub unsafe fn set_priority(interrupt: Nr, prio: Priority) { 
        let irq_number = interrupt.index() as usize;
        (*Self::PTR).priority[irq_number].write(prio.into_bits());
    }

    /// Get threshold for context
    pub fn get_threshold(context: usize) -> Priority {
        let bits = unsafe {
            (*Self::PTR).contexts[context].threshold.read()
        };
        Priority::from_bits(bits)
    }

    /// Set threshold for context
    pub unsafe fn set_threshold(context: usize, threshold: Priority) {
        (*Self::PTR).contexts[context].threshold.write(threshold.into_bits());
    }

    /// Claim interrupt (used by interrupt runtime)
    pub fn claim(context: usize) -> Option<Nr> {
        let bits = unsafe {
            (*Self::PTR).contexts[context].claim.read()
        };
        <Nr as TryFrom<u32>>::try_from(bits).ok()
    }

    /// Complete interrupt (used by interrupt runtime)
    pub fn complete(context: usize, interrupt: Nr) {
        let irq_number = interrupt.index() as u32;
        unsafe {
            (*Self::PTR).contexts[context].claim.write(irq_number);
        }
    }

    /// Checks if interrupt is pending
    pub fn is_pending(interrupt: Nr) -> bool {
        let irq_number = interrupt.index() as usize;
        unsafe {
            (*Self::PTR).pending[irq_number / 32]
                .read() & 1 << (irq_number % 32) != 0
        }
    }
}

/// Interrupt number
///
/// Valid number range are from 1..=1023; interrupt source 0 does not exist.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Nr(NonZeroU16);

impl Nr {
    #[inline] fn index(&self) -> u16 {
        self.0.into()
    }
}

impl TryFrom<u16> for Nr {
    type Error = core::num::TryFromIntError;
    fn try_from(src: u16) -> Result<Nr, Self::Error> {
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl TryFrom<u32> for Nr {
    type Error = core::num::TryFromIntError;
    fn try_from(src: u32) -> Result<Nr, Self::Error> {
        let src: u16 = src.try_into()?;
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl TryFrom<usize> for Nr {
    type Error = core::num::TryFromIntError;
    fn try_from(src: usize) -> Result<Nr, Self::Error> {
        let src: u16 = src.try_into()?;
        let non_zero = src.try_into()?;
        Ok(Nr(non_zero))
    }
}

impl From<Nr> for u16 {
    fn from(src: Nr) -> u16 {
        src.0.into()
    }
}

impl From<Nr> for u32 {
    fn from(src: Nr) -> u32 {
        <u16 as From<Nr>>::from(src).into()
    }
}

impl From<Nr> for usize {
    fn from(src: Nr) -> usize {
        <u16 as From<Nr>>::from(src).into()
    }
}

// todo: highest priority is vendor defined

/// Priority of an interrupt
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Priority {
    /// Priority 0: Never interrupt
    P0,
    /// Priority 1: Lowest active priority
    P1,
    /// Priority 2
    P2,
    /// Priority 3
    P3,
    /// Priority 4
    P4,
    /// Priority 5
    P5,
    /// Priority 6
    P6,
    /// Priority 7: Highest priority
    P7,
}

impl Priority {
    fn into_bits(self) -> u32 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
            Priority::P5 => 5,
            Priority::P6 => 6,
            Priority::P7 => 7,
        }
    }     
    fn from_bits(prio: u32) -> Priority {
        match prio {
            0 => Priority::P0,
            1 => Priority::P1,
            2 => Priority::P2,
            3 => Priority::P3,
            4 => Priority::P4,
            5 => Priority::P5,
            6 => Priority::P6,
            7 => Priority::P7,
            _ => panic!("Invalid priority"),
        }
    }
}
