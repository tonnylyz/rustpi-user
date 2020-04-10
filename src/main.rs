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
mod page_fault;
mod syscall;
mod fork;
mod ipc;
mod page_table;
mod vm_descriptor;

use fork::*;
use syscall::*;
use page_table::*;
use ipc::*;

#[no_mangle]
fn _start(arg: usize) -> ! {
  set_self_ipc(getpid());
  match arg {
    0 => { fktest() },
    1 => { pingpong() },
    _ => {}
  }
  panic!("main returned");
}

fn pingpong() {
  let who= fork();
  if who > 0 {
    println!("send 0 from {} to {}", getpid(), who);
    ipc::send(who as u16, 0, 0, PTE_W);
  }
  loop {
    println!("{} is waiting", getpid());
    let (who, value, _) = ipc::receive(0);
    println!("{} received {} from {}", getpid(), value, who);
    if value == 10 {
      return;
    }
    let value = value + 1;
    println!("{} send {} to {}", getpid(), value, who);
    ipc::send(who, value, 0, PTE_W);
    if value == 10 {
      return;
    }
  }
}

fn fktest() {
  println!("fktest started pid {}", getpid());
  let mut a = 0;
  let mut id = fork();
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
