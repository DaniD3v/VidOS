#![no_std]
#![no_main]

use core::hint::black_box;
use core::mem::transmute;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const SECOND: u32 = 340000000; // TODO this is so bad pls make a hw-clock implementation
const FRAMES: usize = 205;

static VIDEO: &[[u8; 4000]; FRAMES] = unsafe { transmute(include_bytes!("../../converter/examples/videos/ser/BadApple.bin")) };

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        for frame in VIDEO {
            display_image(frame);
            sleep(SECOND / 30);
        }
    }
}

fn display_image(image: &[u8; 4000]) {
    for (i, &byte) in image.iter().enumerate() {
        unsafe { *VGA_BUFFER.add(i) = byte; }
    }
}

fn sleep(ticks: u32) {
    for _ in 0..ticks {
        black_box(1 + 1);
    }
}
