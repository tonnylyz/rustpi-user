.PHONY: all clean user

ARM:=1
#RISCV:=1

ifdef ARM
ARCH:= aarch64
CROSS:= ${ARCH}-elf-
endif
ifdef RISCV
ARCH:= riscv64
CROSS:= ${ARCH}-unknown-elf-
endif

user:
	cargo build --target target.${ARCH}.json -Zbuild-std=core,alloc --release
	cp target/target.${ARCH}/release/rustpi-user rustpi-user.${ARCH}.elf
	${CROSS}objdump -D rustpi-user.${ARCH}.elf > debug.${ARCH}.D
	${CROSS}objdump -x rustpi-user.${ARCH}.elf > debug.${ARCH}.x
	${CROSS}nm -n rustpi-user.${ARCH}.elf > debug.${ARCH}.nm
	cp target/target.${ARCH}/release/rustpi-user ../rustpi/user/${ARCH}.elf

clean:
	cargo clean