use super::csr;

pub fn init() {
    csr::mtvec::init!(stuff);
}

#[repr(align(4))]
pub fn stuff() -> ! {
    let mcause = csr::mcause::read();
    crate::println!("interrupt triggered: {mcause:?}");
    super::qemu::exit();
}
