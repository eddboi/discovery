#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{stm32f3xx_hal::hal::blocking::delay, Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    //let ms = 50_u8;
    let half_period = 500_u16;

    loop {
        //for curr in (0..8) {
          //  let next = (curr + 1) % 8;

            //leds[next].on().ok();
            leds[0].on().ok();
            //delay.delay_ms(ms);
            delay.delay_ms(half_period);
            //leds[curr].off().ok();
            leds[0].off().ok();
            //delay.delay_ms(ms);
            delay.delay_ms(half_period);
        //}
    } 
}

// Lights up everyother LED and then the remaining others
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

//     let ms = 100_u8;
//     loop {
//         for curr in (0..8).step_by(2) {
//             leds[curr].on().ok();
//             delay.delay_ms(ms);
//             leds[curr].off().ok();
//             delay.delay_ms(ms);
//         }
//         for curr in (1..8).step_by(2) {
//             leds[curr].on().ok();
//             delay.delay_ms(ms);
//             leds[curr].off().ok();
//             delay.delay_ms(ms);
//         }
//     }
// }