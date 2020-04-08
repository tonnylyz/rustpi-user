use super::syscall::*;

const STACK_TOP: usize = 0x8000_0000;
const TEMP: usize = 0x4000_0000;
const PAGE_SIZE: usize = 4096;

pub fn fork() -> i32 {
  match process_alloc() {
    Ok(pid) => {
    if pid == 0 {
      0
    } else {
      if let Ok(_) = mem_alloc(0, TEMP, 0) {
        mem_map(0, TEMP, pid, STACK_TOP - PAGE_SIZE, 0);
        unsafe {
          for i in (0..4096).step_by(8) {
            core::intrinsics::volatile_copy_memory(TEMP as *mut u8, (STACK_TOP - PAGE_SIZE) as usize as *const u8, PAGE_SIZE);
          }
        }
        mem_unmap(0, TEMP);
        for i in (0x40000usize..0x43000usize).step_by(PAGE_SIZE) {
          // TODO: fix hardcoded `.text` and `.rodata` pages' mapping
          mem_map(0, i, pid, i, 0);
        }
        process_set_status(pid, ProcessStatus::PsRunnable);
        pid as i32
      } else {
        println!("mem_alloc error");
        -2
      }
    }},
    Err(e) => { println!("process_alloc error {:?}", e); -1 },
  }
}