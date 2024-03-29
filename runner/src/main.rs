use clap::Parser;
use std::process::{self, Command, Stdio};
use target::Target;

pub mod boot;
pub mod target;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// what architecture / machine to target
    #[command(subcommand)]
    target: Option<Target>,
    /// whether to use gdb
    #[arg(long, short, id = "port")]
    gdb: Option<Option<u16>>,
}

fn main() {
    let args = Args::parse();
    let target = args.target.unwrap_or(Target::default());
    std::env::set_current_dir("../kernel").expect("uh oh");
    build(&target, false);
    run_qemu(&target, args.gdb);
}

fn run_qemu(target: &Target, gdb: Option<Option<u16>>) {
    let mut qemu = target.qemu();
    qemu.args(["-d", "guest_errors"]);
    qemu.args(["-m", "1G"]);
    qemu.args(["-monitor", "telnet:127.0.0.1:1235,server,nowait"]);
    if let Some(port) = gdb {
        let port = port.unwrap_or(1234);
        qemu.arg("-S");
        qemu.args(["-gdb", &format!("tcp::{}", port)]);
        let mut gdb = target.gdb();
        gdb.arg("-q");
        gdb.args(["-ex", &format!("target remote :{}", port)]);
        // gdb.args(["-ex", "b kernel::kernel_main"]);
        // gdb.args(["-ex", "c"]);
        let handle = std::thread::spawn(move || {
            qemu.stdin(Stdio::null());
            qemu.stdout(Stdio::null());
            qemu.status().unwrap();
        });
        gdb.status().unwrap();
        handle.join().unwrap();
    } else {
        qemu.status().unwrap();
        // process::exit(exit_status.code().unwrap_or(-1));
    }
}

fn build(target: &Target, test: bool) {
    let mut cargo = Command::new("cargo");
    if test {
        cargo.arg("test");
        cargo.arg("--no-run");
    } else {
        cargo.arg("build");
    }
    cargo.args(["--target", target.rust_target()]);
    let status = cargo.status().expect("uh oh");
    if !status.success() {
        process::exit(status.code().unwrap_or(-1));
    }
    if let Target::X86_64 { bootloader } = target {
        boot::build_bootloader_img(bootloader);
    }
}

#[cfg(test)]
mod tests {
    use crate::{build, run_qemu, target::Target};

    #[test]
    fn default() {
        std::env::set_current_dir("../kernel").expect("uh oh");
        let target = Target::default();
        build(&target, true);
        run_qemu(&target, None);
    }
}
