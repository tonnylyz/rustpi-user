use crate::syscall::*;
use crate::page_table::*;
use crate::config::*;
use crate::page_fault::*;
use crate::ipc::*;

fn duplicate_page(pid: u16, va: usize, pte: PageTableEntryAttr) {
  if pte.shared {
    mem_map(0, va, pid, va, pte);
  } else if pte.writable && !pte.copy_on_write {
    mem_map(0, va, pid, va, pte - PTE_W + PTE_COW);
    mem_map(0, va, 0, va, pte - PTE_W + PTE_COW);
  } else {
    mem_map(0, va, pid, va, pte);
  }
}

extern "C" {
  fn asm_page_fault_handler() -> !;
}

pub fn fork() -> i32 {
  set_page_fault_handler(page_fault_handler as usize);
  match process_alloc() {
    Ok(pid) => if pid == 0 {
      set_self_ipc(getpid());
      0
    } else {
      traverse(|va, attr| {
        duplicate_page(pid, va, attr)
      });
      mem_alloc(pid, EXCEPTION_STACK_BTM, PTE_W);
      process_set_exception_handler(pid, asm_page_fault_handler as usize, EXCEPTION_STACK_TOP);
      process_set_status(pid, ProcessStatus::PsRunnable);
      pid as i32
    },
    Err(e) => {
      println!("process_alloc error {:?}", e);
      -1
    }
  }
}