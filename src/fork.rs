use crate::syscall::*;
use crate::page_table::*;
use crate::config::*;

fn duppage(pid: u16, va: usize, pte: PageTableEntryAttr) {
  if pte.shared {
    mem_map(0, va, pid, va, pte);
  } else if pte.writable && !pte.copy_on_write {
    mem_map(0, va, pid, va, pte - PTE_W + PTE_COW);
    mem_map(0, va, 0, va, pte - PTE_W + PTE_COW);
  } else {
    mem_map(0, va, pid, va, pte);
  }
}

pub fn fork() -> i32 {
  match process_alloc() {
    Ok(pid) => {
    if pid == 0 {
      0
    } else {
      for va in (0..LIMIT).step_by(PAGE_SIZE) {
        if let Some(pte) = query(va) {
          duppage(pid, va, pte);
        }
      }
      // TODO: install page fault handler
      process_set_status(pid, ProcessStatus::PsRunnable);
      pid as i32
    }},
    Err(e) => { println!("process_alloc error {:?}", e); -1 },
  }
}