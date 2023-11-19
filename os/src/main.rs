#![no_std]
#![no_main]

use core::hint::black_box;
use core::mem::transmute;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const SECOND: u32 = 290000000; // TODO this is so bad pls make a hw-clock implementation

macro_rules! path {
    () => { "../../converter/examples/videos/ser/BadApple.bin" };
}


const FRAME_COUNT: usize = {
    let input = include_bytes!(path!());
    if input.len() % FRAME_SIZE != 0 {
        // TODO warning
    }

    input.len() / FRAME_SIZE
};
const FRAME_SIZE: usize = 4000;

static VIDEO: &[[u8; FRAME_SIZE]; FRAME_COUNT] = unsafe { transmute(include_bytes!(path!())) };

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
