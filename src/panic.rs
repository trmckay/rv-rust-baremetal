use core::panic::PanicInfo;

use crate::otter;

// Unlike C, Rust panics sometimes. This can be very
// helpful when you don't have a lot of debugging
// visibility.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    otter::leds_wr(0xFFFF);
    loop {}
}
