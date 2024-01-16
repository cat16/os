use std::env;

use clap::ValueEnum;

#[derive(Copy, Clone, ValueEnum)]
pub enum Bootloader {
    UEFI,
    BIOS,
}

impl Bootloader {
    pub fn img_path(&self) -> &'static str {
        match self {
            Bootloader::UEFI => env!("UEFI_IMAGE"),
            Bootloader::BIOS => env!("BIOS_IMAGE"),
        }
    }
}
