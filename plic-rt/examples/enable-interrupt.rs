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
    pub type PLIC = plic::Plic<0x4000_0000, 3>;
}

use pac::{interrupt, Interrupt};

#[interrupt]
fn GPIO() {
    // interrupt handler
}

#[interrupt]
fn SERIAL() { 
    // interrupt handler
}

fn main() {
    let gpio_enabled = pac::PLIC::is_enabled(0, Interrupt::GPIO);
    println!("Is GPIO interrupt enabled for context 0? {}", gpio_enabled);
    println!("Now enable the GPIO interrupt for context 0.");
    unsafe {
        pac::PLIC::unmask(0, Interrupt::GPIO);
    }
}
