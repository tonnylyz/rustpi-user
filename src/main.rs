#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate register;

use fork::*;
use ipc::*;
use page_fault::*;
use page_table::*;
use syscall::*;

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
mod heap;

#[no_mangle]
fn _start(arg: usize) -> ! {
  set_page_fault_handler(page_fault_handler as usize);
  set_self_ipc(getpid());
  heap::init();
  match arg {
    0 => { fktest() }
    1 => { pingpong() }
    2 => { heap_test() }
    _ => {}
  }
  panic!("main returned");
}

fn pingpong() {
  let who = fork();
  if who > 0 {
    println!("send 0 from {} to {}", getpid(), who);
    ipc::send(who as u16, 0, 0, PTE_DEFAULT);
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
    ipc::send(who, value, 0, PTE_DEFAULT);
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
        print!("{}", a);
      }
    }
    a += 2;
    loop {
      print!("{}", a);
    }
  }
  a += 1;
  loop {
    print!("{}", a);
  }
}

fn heap_test() {
  use alloc::vec::Vec;
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);
  let pid = fork();
  if pid == 0 {
    println!("child {}", a.len());
  } else {
    a.push(4);
    a.push(5);
    println!("parent {}", a.len());
  }
}