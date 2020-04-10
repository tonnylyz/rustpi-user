use crate::syscall::*;
use crate::page_table::*;
use crate::config::*;

#[repr(C, align(32))]
#[derive(Copy, Clone, Debug)]
pub struct InterProcessComm {
  pub index: u16,
  pub ipc_from: u16,
  pub ipc_receiving: bool,
  pub ipc_value: usize,
  pub ipc_dst_addr: usize,
  pub ipc_dst_attr: usize,
}

static mut IPC_SELF: usize = 0;

pub fn set_self_ipc(pid: u16) {
  unsafe {
    IPC_SELF = IPC_LIST_BTM + IPC_PCB_SIZE * ((pid - 1) as usize);
  }
}

pub fn get_self_ipc() -> *const InterProcessComm {
  unsafe {
    IPC_SELF as *const InterProcessComm
  }
}


pub fn send(whom: u16, value: usize, src_va: usize, attr: PageTableEntryAttr) {
  loop {
    match ipc_can_send(whom, value, src_va, attr) {
      Ok(_) => {break},
      Err(SystemCallError::SceIpcNotReceiving) => { process_yield(); },
      Err(e) => { println!("ipc send {:?}", e) },
    }
  }
}

pub fn receive(dst_va: usize) -> (u16, usize, ArchPageTableEntryAttr) {
  ipc_receive(dst_va);
  unsafe {
    ((*get_self_ipc()).ipc_from, (*get_self_ipc()).ipc_value, ArchPageTableEntryAttr::new(0))
  }
}