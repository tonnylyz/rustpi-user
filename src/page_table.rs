use super::config::*;
use super::vm_descriptor::*;

fn read_directory_entry(l1_index: usize) -> u64 {
  let l1x = RECURSIVE_PAGE_TABLE_BTM >> PAGE_TABLE_L1_SHIFT;
  let l2x = RECURSIVE_PAGE_TABLE_BTM >> PAGE_TABLE_L1_SHIFT;
  let l3x = l1_index;
  let ppte = RECURSIVE_PAGE_TABLE_BTM + l1x * (1 << PAGE_TABLE_L2_SHIFT) + l2x * (1 << PAGE_TABLE_L3_SHIFT) + l3x * (1 << WORD_SHIFT);
  unsafe { core::intrinsics::volatile_load(ppte as *const u64) }
}

fn read_level_1_entry(l1_index: usize, l2_index: usize) -> u64 {
  let l1x = RECURSIVE_PAGE_TABLE_BTM >> PAGE_TABLE_L1_SHIFT;
  let l2x = l1_index;
  let l3x = l2_index;
  let ppte = RECURSIVE_PAGE_TABLE_BTM + l1x * (1 << PAGE_TABLE_L2_SHIFT) + l2x * (1 << PAGE_TABLE_L3_SHIFT) + l3x * (1 << WORD_SHIFT);
  unsafe { core::intrinsics::volatile_load(ppte as *const u64) }
}

fn read_level_2_entry(l1_index: usize, l2_index: usize, l3_index: usize) -> u64 {
  let l1x = l1_index;
  let l2x = l2_index;
  let l3x = l3_index;
  let ppte = RECURSIVE_PAGE_TABLE_BTM + l1x * (1 << PAGE_TABLE_L2_SHIFT) + l2x * (1 << PAGE_TABLE_L3_SHIFT) + l3x * (1 << WORD_SHIFT);
  unsafe { core::intrinsics::volatile_load(ppte as *const u64) }
}

fn read_page_table_entry(va: usize) -> Option<u64> {
  let l1x = (va >> PAGE_TABLE_L1_SHIFT) & (PAGE_SIZE / WORD_SIZE - 1);
  let l2x = (va >> PAGE_TABLE_L2_SHIFT) & (PAGE_SIZE / WORD_SIZE - 1);
  let l3x = (va >> PAGE_TABLE_L3_SHIFT) & (PAGE_SIZE / WORD_SIZE - 1);
  if read_directory_entry(l1x) & 0b11 != 0 {
    if read_level_1_entry(l1x, l2x) & 0b11 != 0 {
      let r = read_level_2_entry(l1x, l2x, l3x);
      if r & 0b11 != 0 {
        Some(r)
      } else {
        None
      }
    } else {
      None
    }
  } else {
    None
  }
}

#[derive(Copy, Clone, Debug)]
pub struct PageTableEntryAttr {
  pub executable: bool,
  pub writable: bool,
  pub copy_on_write: bool,
  pub shared: bool,
}

impl Default for PageTableEntryAttr {
  fn default() -> Self {
    PageTableEntryAttr {
      executable: false,
      writable: false,
      copy_on_write: false,
      shared: false
    }
  }
}

impl PageTableEntryAttr {
  pub const fn executable() -> Self {
    PageTableEntryAttr {
      executable: true,
      writable: false,
      copy_on_write: false,
      shared: false
    }
  }
  pub const fn writable() -> Self {
    PageTableEntryAttr {
      executable: false,
      writable: true,
      copy_on_write: false,
      shared: false
    }
  }
  pub const fn copy_on_write() -> Self {
    PageTableEntryAttr {
      executable: false,
      writable: false,
      copy_on_write: true,
      shared: false
    }
  }
  pub const fn shared() -> Self {
    PageTableEntryAttr {
      executable: false,
      writable: false,
      copy_on_write: false,
      shared: true
    }
  }
}

impl core::ops::Add<PageTableEntryAttr> for PageTableEntryAttr {
  type Output = PageTableEntryAttr;

  fn add(self, rhs: PageTableEntryAttr) -> Self::Output {
    PageTableEntryAttr {
      executable: self.executable || rhs.executable,
      writable: self.writable || rhs.writable,
      copy_on_write: self.copy_on_write || rhs.copy_on_write,
      shared: self.shared || rhs.shared
    }
  }
}


impl core::ops::Sub<PageTableEntryAttr> for PageTableEntryAttr {
  type Output = PageTableEntryAttr;

  fn sub(self, rhs: PageTableEntryAttr) -> Self::Output {
    PageTableEntryAttr {
      executable: self.executable &&! rhs.executable,
      writable: self.writable &&! rhs.writable,
      copy_on_write: self.copy_on_write &&! rhs.copy_on_write,
      shared: self.shared &&! rhs.shared
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct ArchPageTableEntryAttr(u64);

impl ArchPageTableEntryAttr {
  pub fn to_usize(&self) -> usize { self.0 as usize }
}

impl core::convert::From<PageTableEntryAttr> for ArchPageTableEntryAttr {
  fn from(pte: PageTableEntryAttr) -> Self {
    ArchPageTableEntryAttr(
      (if pte.writable {
        PAGE_DESCRIPTOR::AP::RW_EL1_EL0
      } else {
        PAGE_DESCRIPTOR::AP::RO_EL1_EL0
      } + if pte.executable {
        PAGE_DESCRIPTOR::UXN::False
      } else {
        PAGE_DESCRIPTOR::UXN::True
      } + if pte.copy_on_write {
        PAGE_DESCRIPTOR::COW::True
      } else {
        PAGE_DESCRIPTOR::COW::False
      } + if pte.shared {
        PAGE_DESCRIPTOR::LIB::True
      } else {
        PAGE_DESCRIPTOR::LIB::False
      }).value
    )
  }
}

impl core::convert::From<ArchPageTableEntryAttr> for PageTableEntryAttr {
  fn from(apte: ArchPageTableEntryAttr) -> Self {
    use register::*;
    let reg = LocalRegisterCopy::<u64, PAGE_DESCRIPTOR::Register>::new(apte.0);
    PageTableEntryAttr {
      executable: !reg.is_set(PAGE_DESCRIPTOR::UXN),
      writable: reg.matches_all(PAGE_DESCRIPTOR::AP::RW_EL1_EL0),
      copy_on_write: reg.is_set(PAGE_DESCRIPTOR::COW),
      shared: reg.is_set(PAGE_DESCRIPTOR::LIB)
    }
  }
}

pub const PTE_X: PageTableEntryAttr = PageTableEntryAttr::executable();
pub const PTE_W: PageTableEntryAttr = PageTableEntryAttr::writable();
pub const PTE_COW: PageTableEntryAttr = PageTableEntryAttr::copy_on_write();
pub const PTE_LIB: PageTableEntryAttr = PageTableEntryAttr::shared();

pub fn query(va: usize) -> Option<PageTableEntryAttr> {
  if let Some(pte) = read_page_table_entry(va) {
    Some(PageTableEntryAttr::from(ArchPageTableEntryAttr(pte)))
  } else {
    None
  }
}

pub fn traverse<F>(f: F) where F: Fn(usize, PageTableEntryAttr) -> () {
  for l1x in 0..(PAGE_SIZE / WORD_SIZE) {
    let l1e = read_directory_entry(l1x);
    if l1e & 0b11 == 0 {
      continue;
    }
    for l2x in 0..(PAGE_SIZE / WORD_SIZE) {
      let l2e = read_level_1_entry(l1x, l2x);
      if l2e & 0b11 == 0{
        continue;
      }
      for l3x in 0..(PAGE_SIZE / WORD_SIZE) {
        let va = (l1x << PAGE_TABLE_L1_SHIFT) + (l2x << PAGE_TABLE_L2_SHIFT) + (l3x << PAGE_TABLE_L3_SHIFT);
        if va >= TRAVERSE_LIMIT {
          return;
        }
        let l3e = read_level_2_entry(l1x, l2x, l3x);
        if l3e & 0b11 != 0 {
          f(va, PageTableEntryAttr::from(ArchPageTableEntryAttr(l3e)));
        }
      }
    }
  }
}