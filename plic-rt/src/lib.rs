#![no_std]

// todo: generated once by plic-rt-macros

// use riscv::register::mhartid;

// #[export_name = "MachineExternal"]
// fn plic_trap_handler() {
//     let hart_id = mhartid::read();
//     let threshold = pac::PLIC::get_threshold(hart_id);
//     let irq = pac::PLIC::claim(hart_id).unwrap();
//     let prio = pac::PLIC::get_priority(irq);
//     unsafe { 
//         pac::PLIC::set_threshold(hart_id, prio);
//         mie::clear_msoft();
//     }
//     // actual ...
//     unsafe { 
//         mie::set_msoft();
//         pac::PLIC::set_threshold(hart_id, threshold);
//     }
//     pac::PLIC::complete(hart_id, irq);
// }
