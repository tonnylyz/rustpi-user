use crate::page_table::*;

global_asm!(include_str!("syscall.S"));

#[derive(Debug)]
pub enum SystemCallError {
  ScePidNotFound = 1,
  ScePidNoParent = 2,
  ScePidParentNotMatched = 3,
  SceMemVaExceedLimit = 4,
  SceMemSrcVaNotMapped = 5,
  SceMemPageTableError = 6,
  SceProcessDirectoryNone = 7,
  SceCurrentProcessNone = 8,
  SceProcessPoolError = 9,
  SceIpcNotReceiving = 10,
  SceUnknownError = 11,
}

use SystemCallError::*;

pub enum ProcessStatus {
  PsRunnable = 1,
  PsNotRunnable = 2,
}

extern "C" {
  fn syscall_1(x0: u8);
  fn syscall_2() -> u16;
  fn syscall_3();
  fn syscall_4(pid: u16) -> isize;
  fn syscall_5(pid: u16, value: usize, sp: usize) -> isize;
  fn syscall_6(pid: u16, va: usize, attr: usize) -> isize;
  fn syscall_7(src_pid: u16, src_va: usize, dst_pid: u16, dst_va: usize, attr: usize) -> isize;
  fn syscall_8(pid: u16, va: usize) -> isize;
  fn syscall_9() -> isize;
  fn syscall_10(pid: u16, status: usize) -> isize;
  fn syscall_11(dst_va: usize);
  fn syscall_12(pid: u16, value: usize, src_va: usize, attr: usize) -> isize;
  fn syscall_13(addr: usize);
}

fn num2err<T>(n: isize) -> Result<T, SystemCallError> {
  Err(match n {
    1 => ScePidNotFound,
    2 => ScePidNoParent,
    3 => ScePidParentNotMatched,
    4 => SceMemVaExceedLimit,
    5 => SceMemSrcVaNotMapped,
    6 => SceMemPageTableError,
    7 => SceProcessDirectoryNone,
    8 => SceCurrentProcessNone,
    9 => SceProcessPoolError,
    10 => SceIpcNotReceiving,
    _ => SceUnknownError
  })
}

pub fn putc(c: char) {
  unsafe { syscall_1(c as u8); }
}

pub fn getpid() -> u16 {
  unsafe { syscall_2() }
}

pub fn process_yield() {
  unsafe { syscall_3() }
}

pub fn process_destroy(pid: u16) -> Result<(), SystemCallError> {
  let i = unsafe { syscall_4(pid) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn process_set_exception_handler(pid: u16, value: usize, sp: usize) -> Result<(), SystemCallError> {
  let i = unsafe { syscall_5(pid, value, sp) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn mem_alloc(pid: u16, va: usize, attr: PageTableEntryAttr) -> Result<(), SystemCallError> {
  let attr = ArchPageTableEntryAttr::from(attr).to_usize();
  let i = unsafe { syscall_6(pid, va, attr) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn mem_map(src_pid: u16, src_va: usize, dst_pid: u16, dst_va: usize, attr: PageTableEntryAttr) -> Result<(), SystemCallError> {
  let attr = ArchPageTableEntryAttr::from(attr).to_usize();
  let i = unsafe { syscall_7(src_pid, src_va, dst_pid, dst_va, attr) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn mem_unmap(pid: u16, va: usize) -> Result<(), SystemCallError> {
  let i = unsafe { syscall_8(pid, va) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

#[inline(always)]
pub fn process_alloc() -> Result<u16, SystemCallError> {
  let i = unsafe { syscall_9() };
  if i >= 0 {
    Ok(i as usize as u16)
  } else {
    num2err((usize::max_value() - (i as usize)) as isize)
  }
}

pub fn process_set_status(pid: u16, status: ProcessStatus) -> Result<(), SystemCallError> {
  let i = unsafe { syscall_10(pid, status as usize) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn ipc_receive(dst_va: usize) {
  unsafe { syscall_11(dst_va); }
}

pub fn ipc_can_send(pid: u16, value: usize, src_va: usize, attr: PageTableEntryAttr) -> Result<(), SystemCallError> {
  let attr = ArchPageTableEntryAttr::from(attr).to_usize();
  let i = unsafe { syscall_12(pid, value, src_va, attr) };
  if i == 0 {
    Ok(())
  } else {
    num2err(i)
  }
}

pub fn process_set_context_frame(addr: usize) { unsafe { syscall_13(addr); } }