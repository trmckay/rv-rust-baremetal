#![no_std] // No standard library. We can't use this.
#![no_main] // We do have a main, but not in the standard Rust way.
#![feature(asm, global_asm)]

// Code availability:
// Code in these modules can be accessed with 'modname::symbol'.
mod otter;
mod panic;
mod util;

// Namespace inclusion:
// This uses the namespace of the util module.
use util::*;

// Include assembly file during compilation.
global_asm!(include_str!("asm/delay.s"));

// Compute the nth Fibonacci number.
// fib(0) = 1, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)
fn fib(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

// main() is called by Rust, so we can drop the C ABI.
fn main() {
    loop {
        // The underlying MMIO functions use unsafe code. However,
        // Rust lets us treat the function as "atomically safe."

        // Read the switches as an 16-bit unsigned integer,
        // compute the fibonacci number at that index,
        // and display it on the seven segment.
        let sw = otter::switches_rd();

        otter::delay(1, TimeUnit::Seconds);

        otter::sseg_wr(fib(sw as u32) as u16);

        // Add one to LEDs if the switches are even.
        if otter::switch_rd(0) == 0 {
            otter::leds_wr(otter::leds_rd() + 1)
        }
    }
}

// While RISC-V is a supported platform for Rust, it does not have
// a stable ABI (on any platform, for that matter).
// 'no_mangle' and 'extern "C"' makes Rust use the stable C ABI.
// Once we are calling Rust from Rust, we don't need this anymore.
//
// Rust will not let you do a lot of unsafe things.
// Returning from your entry-point on baremetal is one of those
// things. This code will not compile if it is possible to
// return from _start(). Hence, the panic (which does not return).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    panic!();
}
