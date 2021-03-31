mod pac {
    #![allow(unused)]
    pub use plic_rt::interrupt;
    use core::convert::TryFrom;

    #[doc = r"Enumeration of all the interrupts"]
    #[derive(Copy, Clone, Debug)]
    #[repr(u16)]
    pub enum Interrupt {
        GPIO = 1,
        SERIAL = 2,
    }

    impl From<Interrupt> for plic::Nr {
        fn from(src: Interrupt) -> plic::Nr {
            // note(unwrap): always success for non zero numbers
            plic::Nr::try_from(src as u16).unwrap()
        }
    }

    extern {
        fn GPIO();
        fn SERIAL();
    }
    
    #[doc(hidden)]
    pub union Vector {
        // must be public for macro
        pub handler: unsafe extern "C" fn(),
        reserved: usize,
    }
    
    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 3] = [
        Vector { reserved: 0 },
        Vector { handler: GPIO },
        Vector { handler: SERIAL },
    ];

    // must keep for macro to work
    pub type PLIC = plic::Plic<0x4000_0000>;
}

use pac::{interrupt, Interrupt};

// if you modify function's name, it would become compile error
// this is detected from fields in `pac::Interrupt`.

#[interrupt]
fn GPIO() {
    // interrupt handler
}

#[interrupt]
fn SERIAL() { 
    // interrupt handler
}

// though there would be many interrupts in applications,
// the macro would generate `MachineExternal` symbol, but only generate once 

fn main() {

}
