PROJECT = otter-rust
BUILD = target/riscv32i-unknown-none-elf/release
DEST = target/otter
RUST_BINARY = $(BUILD)/$(PROJECT)

RISCV_PREFIX = riscv32-unknown-elf-

OBJCOPY = $(RISCV_PREFIX)objcopy
OBJCOPY_FLAGS = --only-section=.data* --only-section=.text*
STRIPPED_BINARY = $(DEST)/$(PROJECT).mem.bin

HEXDUMP = hexdump
HEXDUMP_FLAGS = -v -e '"%08x\n"'
HEXDUMP_FILE = $(DEST)/$(PROJECT).mem.txt

OBJDUMP = $(RISCV_PREFIX)objdump
OBJDUMP_FLAGS = -S -s
DUMP = $(DEST)/$(PROJECT).dump

CARGO_FLAGS = --release
RUST_FLAGS = -A dead_code

all: dirs cargo $(STRIPPED_BINARY) $(DUMP) $(HEXDUMP_FILE)
	@echo
	@echo "Program dump at $(DUMP)"
	@echo "Binary at $(STRIPPED_BINARY)"
	@echo "Hex-dump at $(HEXDUMP_FILE)"

cargo:
	CARGO_BUILD_RUSTFLAGS="$(RUST_FLAGS)" cargo build $(CARGO_FLAGS)
	@echo

$(STRIPPED_BINARY):
	$(OBJCOPY) -O binary $(OBJCOPY_FLAGS) $(RUST_BINARY) $@

$(DUMP):
	$(OBJDUMP) $(OBJDUMP_FLAGS) $(RUST_BINARY) > $@


$(HEXDUMP_FILE): $(STRIPPED_BINARY)
	hexdump $(HEXDUMP_FLAGS) $< > $@

dirs:
	mkdir -p $(DEST)

clean:
	rm -rf $(DEST)
	cargo clean
