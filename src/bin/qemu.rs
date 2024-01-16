use std::{
    env,
    process::{self, Command, Stdio},
};

use clap::Parser;
use os::Bootloader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// whether to use BIOS or UEFI
    #[arg(long, short, id = "type", default_value = "bios")]
    bootloader: Bootloader,
    /// whether to use gdb
    #[arg(long, short, id = "port")]
    gdb: Option<Option<u16>>,
}

fn main() {
    let args = Cli::parse();
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
    qemu.args(["-serial", "stdio"]);
    qemu.arg("-drive");
    qemu.arg(format!("format=raw,file={}", args.bootloader.img_path()));
    if let Bootloader::UEFI = args.bootloader {
        qemu.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    }
    if let Some(port) = args.gdb {
        let port = port.unwrap_or(1234);
        qemu.arg("-S");
        qemu.args(["-gdb", &format!("tcp::{}", port)]);
        let mut gdb = Command::new("rust-gdb");
        gdb.arg("-q");
        gdb.args(["-ex", "target remote :1234"]);
        gdb.args([
            "-ex",
            &format!("symbol-file {} -o 0x8000000000", env!("KERNEL_BIN")),
        ]);
        gdb.args(["-ex", "b kernel::kernel_main"]);
        gdb.args(["-ex", "c"]);
        let handle = std::thread::spawn(move || {
            qemu.stdin(Stdio::null());
            qemu.stdout(Stdio::null());
            let exit_status = qemu.status().unwrap();
        });
        gdb.status().unwrap();
        handle.join().unwrap();
    } else {
        let exit_status = qemu.status().unwrap();
        process::exit(exit_status.code().unwrap_or(-1));
    }
}
