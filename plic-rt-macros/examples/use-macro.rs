mod pac {
    pub use plic_rt_macros::interrupt;

    #[doc = r"Enumeration of all the interrupts"]
    #[derive(Copy, Clone, Debug)]
    #[repr(u16)]
    pub enum Interrupt {
        GPIO = 10,
    }

    impl plic::Nr for Interrupt {
        fn number(self) -> u16 {
            self as u16
        }
    }
}

use pac::{interrupt, Interrupt};

#[interrupt]
fn GPIO() { // if you modify this function's name, it would become compile error
    
}

fn main() {

}
