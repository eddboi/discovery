#![no_main]
#![no_std]

//use core::{f32::consts, ptr};

//#[allow(unused_imports)]
//use aux7::{entry, iprint, iprintln};

//#[entry]
// fn main() -> ! {
//     aux7::init();

//     unsafe {
//         // A magic address!
//         const GPIOE_BSRR: u32 = 0x48001018;

//         // Turn on the "North" LED (red)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

//         // Turn on the "East" LED (green)
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

//         // Turn off the "North" LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

//         // Turn off the "East" LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
//     }

//     loop {}
// }

// use core::ptr;

// #[allow(unused_imports)]
// use aux7::{entry, iprintln, ITM};

// fn iprint_odr(itm: &mut ITM) {
//     const  GPIOE_ODR: u32 = 0x4800_1014;
    
//     unsafe {
//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );
//     }
// }

// #[entry]
// fn main() -> ! {
//     let mut itm= aux7::init().0;

//     unsafe {
//         // A magic address
//         const GPIOE_BSRR: u32 = 0x4800_1018;

//         // Print initial contents of ODR
//         iprint_odr(&mut itm);

//         // Turn ON the NORTH (red) LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
//         iprint_odr(&mut itm);

//         // Turn ON the EAST (green) LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
//         iprint_odr(&mut itm);

//         // Turn OFF the NORTH LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
//         iprint_odr(&mut itm);

//         // Turn OFF the EAST LED
//         ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
//         iprint_odr(&mut itm);
//     }
//     loop {}
// }

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM, RegisterBlock};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}