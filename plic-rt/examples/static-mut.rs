mod pac {
    pub use plic_rt::interrupt;

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

    extern {
        fn GPIO();
    }

    #[doc(hidden)]
    pub union Vector {
        pub handler: unsafe extern "C" fn(),
        reserved: usize,
    }
    
    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 2] = [
        Vector { reserved: 0 },
        Vector { handler: GPIO },
    ];

    pub type PLIC = plic::Plic<0x4000_0000>;
}

use pac::{interrupt, Interrupt};

#[interrupt]
fn GPIO() { // if you modify this function's name, it would become compile error
    static mut SAFE_STATIC_MUT: usize = 0;
    
    // by deref this variable, you get a `&mut usize`.
    
    let _a = *SAFE_STATIC_MUT; // this is safe
    *SAFE_STATIC_MUT = 1; // this is safe
}

fn main() {

}
