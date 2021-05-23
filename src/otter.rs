use crate::util::TimeUnit;
use core::ptr::*;

pub const IMMEDIATE_MAX: u16 = 0xFFF;

pub const FREQ_HZ: u32 = 50000000;
pub const FREQ_KHZ: u32 = 50000;
pub const FREQ_MHZ: u32 = 50;

pub const LEDS: *const u16 = (0x11080000) as *const u16;
pub const LEDS_MUT: *mut u16 = (0x11080000) as *mut u16;

pub const SSEG: *const u16 = (0x110C0000) as *const u16;
pub const SSEG_MUT: *mut u16 = (0x110C0000) as *mut u16;

pub const SWITCHES: *const u16 = (0x11000000) as *const u16;

pub fn leds_rd() -> u16 {
    let leds;
    unsafe { leds = read_volatile(LEDS) }
    leds
}

pub fn leds_wr(n: u16) {
    unsafe { write_volatile(LEDS_MUT, n) }
}

pub fn sseg_rd(n: u16) {
    unsafe { read_volatile(SSEG as *const ()) }
}

pub fn sseg_wr(n: u16) {
    unsafe { write_volatile(SSEG_MUT, n) }
}

pub fn switches_rd() -> u16 {
    let sw;
    unsafe { sw = read_volatile(SWITCHES) }
    sw
}

pub fn switch_rd(index: u8) -> u16 {
    let sw = switches_rd();
    (sw & (0b1 << index)) >> index
}

pub fn delay(delay: u32, units: TimeUnit) {
    let cycles = delay
        * match units {
            TimeUnit::Seconds => FREQ_HZ,
            TimeUnit::Milliseconds => FREQ_KHZ,
            TimeUnit::Microseconds => FREQ_MHZ,
        };
    delay_cycles(cycles);
}

pub fn delay_cycles(cycles: u32) {
    unsafe {
        // Inline assembly:
        // Load the delay variable into a0.
        // This works sort of like a format string.
        asm!("mv a0, {}", in(reg) cycles);

        // Call the delay routine defined in included assembly.
        asm!("call _ASM_DELAY_CYCLES")
    }
}
