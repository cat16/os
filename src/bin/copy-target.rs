use std::{env, fs};

fn main() {
    let current_exe = env::current_exe().unwrap();
    let uefi = current_exe.with_file_name("uefi.img");
    let bios = current_exe.with_file_name("bios.img");

    fs::copy(env!("UEFI_IMAGE"), &uefi).unwrap();
    fs::copy(env!("BIOS_IMAGE"), &bios).unwrap();
}
