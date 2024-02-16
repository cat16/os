pub mod csr;
pub mod init;
pub mod asm;
pub mod interrupts;
pub mod paging;
pub mod qemu;
pub mod mem;

pub fn wait() -> ! {
    loop {
        asm::wfi();
    }
}
