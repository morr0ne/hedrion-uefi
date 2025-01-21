#![no_std]
#![no_main]

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hedrion love you <3";

    unsafe {
        // Write text to VGA buffer
        for (i, &byte) in msg.iter().enumerate() {
            // Each character takes 2 bytes - ASCII value and color attribute
            *VGA_BUFFER.add(i * 2) = byte;
            // White text on black background
            *VGA_BUFFER.add(i * 2 + 1) = 0x0F;
        }
    }

    loop {}
}
