#![no_std] // No standard library. We can't use this.
#![no_main] // We do have a main, but not in the standard Rust way.
#![feature(asm, global_asm)]

// Include assembly file during compilation.
// We need to include some things at the top of
// the text section.
global_asm!(include_str!("asm/init.s"));

// Rust modules that we include with the project.
pub mod otter;
mod panic;

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
        // Load in switches value.
        let sw = otter::switches_rd() as u32;

        // Calculate fibonacci number.
        let fib = fib(sw) as u16;

        // Write it to the seven segment display.
        otter::sseg_wr(fib);
    }
}

// While RISC-V is a supported platform for Rust, it does not have
// a stable ABI (on any platform, for that matter).
// 'no_mangle' and 'extern "C"' makes Rust use the C ABI.
// Once we are calling Rust from Rust, we don't need this anymore.
//
// Rust will not let you do a lot of unsafe things.
// Returning from your entry-point on baremetal is one of those
// things. This code will not compile if it is possible to
// return from _rust_entry(). Hence, the panic (which does not return).
#[no_mangle]
pub extern "C" fn _rust_entry() -> ! {
    main();
    panic!();
}
