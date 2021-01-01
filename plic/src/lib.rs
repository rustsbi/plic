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

/// Platform-Level Interrupt Controller
pub struct Plic<const B: usize>(());

impl<const B: usize> Plic<B> {
    const PTR: *const plic::RegisterBlock = B as *const _;
}

impl<const B: usize> Plic<B> {
    /// Check if interrupt is enabled for context
    pub fn is_enabled<I: Nr>(context: usize, interrupt: I) -> bool {
        let irq_number = interrupt.number() as usize;
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
    pub unsafe fn unmask<I: Nr>(context: usize, interrupt: I) {
        let irq_number = interrupt.number() as usize;
        (*Self::PTR).enables[context].enable[irq_number / 32]
            .modify(|v| v | 1 << (irq_number % 32));
    }

    /// Disable interrupt for context
    pub fn mask<I: Nr>(context: usize, interrupt: I) { 
        let irq_number = interrupt.number() as usize;
        unsafe {
            (*Self::PTR).enables[context].enable[irq_number / 32]
                .modify(|v| v & !(1 << (irq_number % 32)));
        }
    }

    /// Get interrupt priority
    pub fn get_priority<I: Nr>(interrupt: I) -> Priority { 
        let irq_number = interrupt.number() as usize;
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
    pub unsafe fn set_priority<I: Nr>(interrupt: I, prio: Priority) { 
        let irq_number = interrupt.number() as usize;
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
    pub fn claim(context: usize) -> Option<u16> {
        let bits = unsafe {
            (*Self::PTR).contexts[context].claim.read()
        };
        use core::convert::TryInto;
        bits.try_into().ok()
    }

    /// Complete interrupt (used by interrupt runtime)
    pub fn complete<I: Nr>(context: usize, interrupt: I) {
        let irq_number = interrupt.number() as u32;
        unsafe {
            (*Self::PTR).contexts[context].claim.write(irq_number);
        }
    }

    /// Checks if interrupt is pending
    pub fn is_pending<I: Nr>(interrupt: I) -> bool {
        let irq_number = interrupt.number() as usize;
        unsafe {
            (*Self::PTR).pending[irq_number / 32]
                .read() & 1 << (irq_number % 32) != 0
        }
    }
}

/// Interrupt number
pub trait Nr {
    /// Valid values are within 0..=1023
    fn number(self) -> u16;
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
