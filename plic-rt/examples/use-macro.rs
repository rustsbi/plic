mod pac {
    #![allow(unused)]
    pub use plic_rt::interrupt;

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
        pub handler: unsafe extern "C" fn(),
        reserved: usize,
    }
    
    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 3] = [
        Vector { reserved: 0 },
        Vector { handler: GPIO },
        Vector { handler: SERIAL },
    ];

    pub type PLIC = plic::Plic<0x4000_0000>;
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
