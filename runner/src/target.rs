use std::process::Command;

use clap::Subcommand;

use crate::boot::Bootloader;

#[derive(Copy, Clone, Subcommand)]
pub enum Target {
    X86_64 {
        #[arg(long, short, id = "type", default_value = "bios")]
        bootloader: Bootloader,
    },
    Riscv64,
}

impl Default for Target {
    fn default() -> Self {
        Self::Riscv64
    }
}

impl Target {
    pub const X86_64_RUST_TARGET: &'static str = "x86_64-unknown-none";
    pub fn qemu(&self) -> Command {
        match self {
            Self::X86_64 { bootloader } => {
                let mut cmd = Command::new("qemu-system-x86_64");
                if let Bootloader::UEFI = bootloader {
                    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
                }
                cmd.args([
                    "-drive",
                    &format!("format=raw,file={}", self.qemu_bin_path()),
                ]);
                cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
                cmd.args(["-serial", "stdio"]);
                cmd
            }
            Self::Riscv64 => {
                let mut cmd = Command::new("qemu-system-riscv64");
                cmd.arg("-nographic");
                cmd.arg("-semihosting");
                cmd.args(["-cpu", "rv64"]);
                cmd.args(["-machine", "virt"]);
                cmd.args(["-bios", "none"]);
                cmd.args(["-smp", "4"]);
                cmd.args(["-kernel", &self.qemu_bin_path()]);
                cmd
            }
        }
    }
    pub fn gdb(&self) -> Command {
        match self {
            Self::X86_64 { .. } => {
                let mut cmd = Command::new("rust-gdb");
                cmd.args([
                    "-ex",
                    &format!("symbol-file {} -o 0x8000000000", self.qemu_bin_path()),
                ]);
                cmd
            }
            Self::Riscv64 => {
                let mut cmd = Command::new("gdb");
                cmd.arg(self.qemu_bin_path());
                cmd
            }
        }
    }
    pub fn rust_target(&self) -> &'static str {
        match self {
            Self::X86_64 { .. } => Self::X86_64_RUST_TARGET,
            Self::Riscv64 => "riscv64gc-unknown-none-elf",
        }
    }
    pub fn target_folder(&self) -> String {
        format!("target/{}/debug", self.rust_target())
    }
    pub fn binary_path(&self) -> String {
        format!("{}/kernel", self.target_folder())
    }
    pub fn qemu_bin_path(&self) -> String {
        match self {
            Self::X86_64 { bootloader } => {
                self.binary_path() + &format!("-{}.img", bootloader.to_str().to_lowercase())
            }
            Self::Riscv64 => self.binary_path(),
        }
    }
}
