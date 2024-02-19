use crate::{
    arch::csr::{self, satp},
    util::bits::bits,
};
use core::slice;

use super::mem::PROGRAM_END;

const PAGE_SIZE: usize = 4096;
const TABLE_LEN: usize = 2usize.pow(9);
const TABLE_SIZE: usize = core::mem::size_of::<Table>();

#[repr(C)]
pub struct Entry(usize);

impl Entry {
    pub fn set_table(&mut self, addr: *const Table) {
        self.0 = addr as usize >> 2;
        self.set_valid(true);
    }
    pub fn set_valid(&mut self, valid: bool) {
        if valid {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }
    pub fn set_addr(&mut self, addr: usize) {
        self.clear();
        self.0 |= addr as usize >> 2;
        self.0 |= 0b1110;
        self.set_valid(true);
    }
    pub fn get_table(&self) -> *const Table {
        ((self.0 & !0b1111111111) << 2) as *const Table
    }
    pub fn get_addr(&self) -> usize {
        let val = self.0;
        bits!(val;10,53) << 12
    }
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

#[repr(C, align(4096))]
pub struct Table {
    pub entries: [Entry; TABLE_LEN],
}

pub fn init(mem_end: *mut u8) -> *mut u8 {
    unsafe {
        let total_pages = mem_end as usize / 4096;
        let lvl1_count = (total_pages.saturating_sub(1)) / TABLE_LEN + 1;
        let lvl2_count = (lvl1_count.saturating_sub(1)) / TABLE_LEN + 1;
        let lvl2 = &mut *(PROGRAM_END as *mut Table);
        let lvl1_arr = slice::from_raw_parts_mut((lvl2 as *mut Table).add(TABLE_SIZE), lvl2_count);
        let lvl0_arr = slice::from_raw_parts_mut(
            lvl1_arr.as_ptr().add(lvl1_arr.len()) as *mut Table,
            lvl1_count,
        );
        let mut lvl1itr = lvl1_arr.iter();
        for entry in &mut lvl2.entries {
            if let Some(table) = lvl1itr.next() {
                entry.set_table(table as *const Table);
            } else {
                entry.clear();
            }
        }
        let mut lvl0itr = lvl0_arr.iter();
        for lvl1 in lvl1_arr {
            for entry in &mut lvl1.entries {
                if let Some(table) = lvl0itr.next() {
                    entry.set_table(table as *const Table);
                } else {
                    entry.clear();
                }
            }
        }
        let mut i = 0;
        for lvl0 in &mut *lvl0_arr {
            for entry in &mut lvl0.entries {
                if i < total_pages {
                    entry.set_addr(i * PAGE_SIZE);
                } else {
                    entry.clear()
                }
                i += 1;
            }
        }

        csr::satp::write(satp::Satp {
            mode: satp::Mode::Sv39,
            asid: 0,
            ppn: lvl2,
        });
        let table_end = lvl0_arr.as_ptr().add(lvl0_arr.len()) as *mut u8;
        table_end
    }
}

pub fn virt_to_physical(table: &Table, addr: usize) -> usize {
    let ppn2 = bits!(addr;30,38);
    let ppn1 = bits!(addr;21,29);
    let ppn0 = bits!(addr;12,20);
    let offset = bits!(addr;0,11);
    // let satp = csr::satp::read();
    unsafe {
        let lvl2 = table as *const Table;
        let lvl1 = (*lvl2).entries[ppn2].get_table();
        let lvl0 = (*lvl1).entries[ppn1].get_table();
        let base = (*lvl0).entries[ppn0].get_addr();
        let addr = base + offset;
        addr
    }
}
