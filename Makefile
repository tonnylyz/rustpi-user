.PHONY: all clean user

all: user

clean:
	cargo clean

user:
	cargo xbuild --target aarch64-none-elf.json --release --verbose
	cp target/aarch64-none-elf/release/rustpi-user user.elf
	aarch64-elf-objdump -D user.elf > debug.D
	aarch64-elf-objdump -x user.elf > debug.x
	aarch64-elf-nm -n user.elf > debug.nm
	aarch64-elf-ld -r -b binary -o user.o user.elf
	cp user.o ../rustpi/user
	aarch64-elf-nm -n user.o
