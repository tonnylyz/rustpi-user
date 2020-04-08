use core::fmt;
use crate::syscall::putc;

struct Writer;

static mut WRITER: Writer = Writer;

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      putc(c);
    }
    Ok(())
  }
}

pub fn print_arg(args: fmt::Arguments) {
  use core::fmt::Write;
  unsafe {
    WRITER.write_fmt(args).unwrap();
  }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  if let Some(m) = info.message() {
    println!("\nUser space panic: {} \n {}", m, info.location().unwrap());
  } else {
    println!("\nUser space panic!");
  }
  loop {}
}