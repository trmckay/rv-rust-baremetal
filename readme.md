# RISC-V Rust Baremetal Template

Template for Rust on baremetal `rv32i` targets. Examples of:

* Board/core specific stuff
* Calling assembly subroutine
* Inline assembly
* Panic implementation
* Linker script
* Makefile

## Setup

Requires:

* [`rustup`](https://rustup.rs/)
* [`riscv-gnu-toolchain`](https://github.com/riscv/riscv-gnu-toolchain) (if manipulating built binaries with `objcopy` or `objdump`)

```bash
rustup target add riscv32i-unknown-none-elf
rustup default nightly  # For language features like asm!()
```
