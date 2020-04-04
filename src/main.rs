#![feature(asm)]
#![feature(global_asm)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("syscall.S"));

extern "C" {
    /* Note:
       Linux(Aarch64) use x0~x7 for syscall arguments
       use x8 for syscall number
       use x0, x1 for return value
    */
    fn msyscall(x0: usize, x1: usize, x2: usize, x3: usize,
                x4: usize, x5: usize, x6: usize, x7: usize, no: usize) -> usize;
}

fn putc(c: char) {
    unsafe { msyscall(c as usize, 0, 0,0,0,0,0,0,1); }
}

fn puts(s: &'static str) {
    for c in s.chars() {
        putc(c);
    }
}

#[no_mangle]
fn _start() {
    loop { puts("Hello world!\n"); }
}
