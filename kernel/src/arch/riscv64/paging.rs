use crate::{
    arch::csr::{self, satp},
    fdt::print_mem_layout,
    println,
};

use super::asm::linker_static;

linker_static!(HEAP_START: usize, ".dword _heap_start");
static HEAP_SIZE: usize = 128 * 1024 * 1024;

pub struct Entry(u64);

pub struct Table {
    pub entries: [Entry; 2usize.pow(9)],
}

pub fn init(fdt: usize) {
    unsafe {
        println!("heap start: 0x{:x}", HEAP_START);
        print_mem_layout(fdt);
        let table_start = HEAP_START as *mut Table;
        csr::satp::write(satp::Satp {
            mode: satp::Mode::Sv39,
            asid: 0,
            ppn: table_start,
        });
        let satp = csr::satp::read();
        println!("satp: {satp:?}");
        let x = *(0x9000_0000 as *mut u8);
        println!("we got {x}");
    }
}
