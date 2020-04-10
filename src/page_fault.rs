use crate::syscall::*;
use crate::config::*;
use crate::page_table::*;

global_asm!(include_str!("page_fault.S"));

pub fn page_fault_handler(va: usize) {
  assert_eq!(va % PAGE_SIZE, 0);
  if let Some(pte) = query(va) {
    if !pte.copy_on_write {
      panic!("page_fault_handler not copy on write");
    }
    let mut va_tmp = STACK_BTM - PAGE_SIZE;
    loop {
      if let Some(_) = query(va_tmp) {
        va_tmp -= PAGE_SIZE;
      } else {
        break;
      }
    }
    mem_alloc(0, va_tmp, PTE_W);
    unsafe {
      core::intrinsics::volatile_copy_memory(va_tmp as *mut u8, va as *mut u8, PAGE_SIZE);
    }
    mem_map(0, va_tmp, 0, va, pte + PTE_W - PTE_COW);
    mem_unmap(0, va_tmp);
  } else {
    panic!("page_fault_handler not mapped");
  }
}

extern "C" {
  fn asm_page_fault_handler() -> !;
}

#[no_mangle]
pub static mut page_fault_handler_stub: usize = 0;

pub fn set_page_fault_handler(handler: usize) {
  unsafe {
    if page_fault_handler_stub == 0 {
      mem_alloc(0, EXCEPTION_STACK_BTM, PTE_W);
      process_set_exception_handler(0, asm_page_fault_handler as usize, EXCEPTION_STACK_TOP);
    }
    page_fault_handler_stub = handler;
  }
}