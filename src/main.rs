#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::print_arg(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::print_arg(format_args_nl!($($arg)*));
    })
}

mod config;
mod print;
mod syscall;
mod fork;
mod page_table;
mod vm_descriptor;

use config::*;
use fork::*;
use syscall::*;

#[no_mangle]
fn _start() -> ! {
  main();
  panic!("main returned");
}



fn main() {
  println!("fktest started pid {}", getpid());
  let mut a = 0;
  let mut id = 0;
  id = fork();
  if id == 0 {
    id = fork();
    if id == 0 {
      a += 3;
      loop {
        println!("\t\tthis is child2 :a:{}", a);
      }
    }
    a += 2;
    loop {
      println!("\tthis is child :a:{}", a);
    }
  }
  a += 1;
  loop {
    println!("this is father :a:{}", a);
  }
}
