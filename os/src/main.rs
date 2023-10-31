#![no_std]
#![no_main]

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static IMAGE: &[u8; 4000] = include_bytes!("../../converter/examples/images/ser/incredible.bin");

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for (i, &byte) in IMAGE.iter().enumerate() {
        unsafe { *VGA_BUFFER.offset(i as isize) = byte; }
    }

    loop {}
}
