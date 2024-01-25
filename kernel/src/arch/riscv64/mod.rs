pub mod csr;
pub mod init;
pub mod instructions;
pub mod interrupts;
pub mod page;
pub mod qemu;

pub fn wait() -> ! {
    loop {
        instructions::wfi();
    }
}
