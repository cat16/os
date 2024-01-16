use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    println!("{kernel_path}");
    let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path.clone()));

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();
    let uefi_path = out_dir.join(name.clone() + "-uefi.img");
    let bios_path = out_dir.join(name + "-bios.img");

    disk_builder.create_uefi_image(&uefi_path).unwrap();
    disk_builder.create_bios_image(&bios_path).unwrap();

    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
    println!("cargo:rustc-env=KERNEL_BIN={}", kernel_path);
}
