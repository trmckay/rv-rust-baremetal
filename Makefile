PROJECT = otter-rust
BUILD = target/riscv32i-unknown-none-elf/release
DEST = target/otter

RISCV_PREFIX = riscv32-unknown-elf-

OBJCOPY = $(RISCV_PREFIX)objcopy
OBJDUMP = $(RISCV_PREFIX)objdump

all: clean dirs $(DEST)/mem.txt $(DEST)/$(PROJECT).dump
	@echo
	@echo "Program dump at $(DEST)/$(PROJECT).dump"
	@echo "Binary blob at $(DEST)/mem.bin"
	@echo "Hex-dump at $(DEST)/mem.txt"
	@echo
	cat $(DEST)/mem.txt

$(BUILD)/$(PROJECT):
	cargo build --release

$(DEST)/$(PROJECT).dump: $(BUILD)/$(PROJECT)
	$(OBJDUMP) -S -s $< > $@

$(DEST)/mem.bin: $(BUILD)/$(PROJECT)
	$(OBJCOPY) -O binary --only-section=.data* --only-section=.text* $< $@

$(DEST)/mem.txt: $(DEST)/mem.bin
	hexdump -v -e '"%08x\n"' $< > $@

dirs:
	mkdir -p $(DEST)

clean:
	rm -rf $(TEST)
	cargo clean
