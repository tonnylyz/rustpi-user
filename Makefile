.PHONY: all clean aarch64 riscv64

all: aarch64 riscv64

aarch64:
	cargo build --target target.aarch64.json -Zbuild-std=core,alloc --release
	cp target/target.aarch64/release/rustpi-user aarch64.elf

riscv64:
	cargo build --target target.riscv64.json -Zbuild-std=core,alloc --release
	cp target/target.riscv64/release/rustpi-user riscv64.elf

clean:
	cargo clean