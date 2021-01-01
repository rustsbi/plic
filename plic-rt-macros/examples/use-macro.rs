mod pac {
    #![allow(unused)]
    pub use plic_rt_macros::interrupt;

    #[doc = r"Enumeration of all the interrupts"]
    #[derive(Copy, Clone, Debug)]
    #[repr(u16)]
    pub enum Interrupt {
        GPIO = 1,
        SERIAL = 2,
    }

    impl plic::Nr for Interrupt {
        fn number(self) -> u16 {
            self as u16
        }
    }

    extern {
        fn GPIO();
        fn SERIAL();
    }
    
    #[doc(hidden)]
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    
    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 3] = [
        Vector { _reserved: 0 },
        Vector { _handler: GPIO },
        Vector { _handler: SERIAL },
    ];
}

use pac::{interrupt, Interrupt};

// if you modify function's name, it would become compile error

#[interrupt]
fn GPIO() {
    // interrupt handler
}

#[interrupt]
fn SERIAL() { 
    // interrupt handler
}

fn main() {

}
