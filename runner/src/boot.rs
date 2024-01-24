use bootloader::DiskImageBuilder;
use clap::ValueEnum;
use std::{path::PathBuf, process};

use crate::target::Target;

#[derive(Copy, Clone, ValueEnum)]
pub enum Bootloader {
    UEFI,
    BIOS,
}

impl Bootloader {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::UEFI => "UEFI",
            Self::BIOS => "BIOS",
        }
    }
}

pub fn build_bootloader_img(bootloader: &Bootloader) {
    let target = Target::X86_64 {
        bootloader: *bootloader,
    };
    let kernel_path = PathBuf::from(target.binary_path());
    let disk_builder = DiskImageBuilder::new(kernel_path.clone());

    let img_path = PathBuf::from(target.qemu_bin_path());

    let result = match bootloader {
        Bootloader::UEFI => disk_builder.create_uefi_image(&img_path),
        Bootloader::BIOS => disk_builder.create_bios_image(&img_path),
    };
    if let Err(err) = result {
        println!("Failed to build bootloader: {}", err);
        process::exit(1);
    }
}

