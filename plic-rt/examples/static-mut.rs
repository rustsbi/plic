mod pac {
    use core::convert::TryFrom;
    pub use plic_rt::interrupt;

    #[doc = r"Enumeration of all the interrupts"]
    #[derive(Copy, Clone, Debug)]
    #[repr(u16)]
    pub enum Interrupt {
        GPIO = 10,
    }

    impl From<Interrupt> for plic::Nr {
        fn from(src: Interrupt) -> plic::Nr {
            // note(unwrap): always success for non zero numbers
            plic::Nr::try_from(src as u16).unwrap()
        }
    }

    extern "C" {
        fn GPIO();
    }

    #[doc(hidden)]
    pub union Vector {
        pub handler: unsafe extern "C" fn(),
        reserved: usize,
    }

    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 2] = [Vector { reserved: 0 }, Vector { handler: GPIO }];

    pub type PLIC = plic::Plic<0x4000_0000, 3>;
}

use pac::{interrupt, Interrupt};

#[interrupt]
fn GPIO() {
    // if you modify this function's name, it would become compile error
    static mut SAFE_STATIC_MUT: usize = 0;

    // by deref this variable, you get a `&mut usize`.

    let _a = *SAFE_STATIC_MUT; // this is safe
    *SAFE_STATIC_MUT = 1; // this is safe
}

fn main() {}
