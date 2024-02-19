macro_rules! csrr {
    ($name:expr) => {{
        let mut out: u64;
        core::arch::asm!(
            concat!("csrr {out}, ", $name),
            out = out(reg) out,
        );
        out
    }};
}

macro_rules! csrw_fn {
    ($name:expr, $func:path) => {
        unsafe {
            core::arch::asm!(
                "la t0, {func}",
                concat!("csrw ", $name, ", t0"),
                func = sym $func,
            );
        }
    };
}
pub(crate) use csrw_fn;

macro_rules! csrw {
    ($name:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", $name, ", {val}"),
                val = in(reg) $val
            );
        }
    };
}

pub mod hartid {
    pub fn read() -> u64 {
        unsafe { csrr!("mhartid") }
    }
}

pub mod mtvec {
    macro_rules! init {
        ($func:path) => {
            let _: fn() -> ! = $func;
            crate::arch::csr::csrw_fn!("mtvec", $func);
        };
    }
    pub(crate) use init;
    pub fn read() -> u64 {
        unsafe { csrr!("mtvec") }
    }
}

pub mod mcause {
    use core::mem::transmute;

    #[derive(Debug)]
    pub enum MCause {
        // interrupt = 1
        SupervisorSoftwareInterrupt = (1 << 63) | 1,
        VirtualSupervisorSoftwareInterrupt = (1 << 63) | 2,
        MachineSoftwareInterrupt = (1 << 63) | 3,

        SupervisorTimerInterrupt = (1 << 63) | 5,
        VirtualSupervisorTimerInterrupt = (1 << 63) | 6,
        MachineTimerInterrupt = (1 << 63) | 7,

        SupervisorExternalInterrupt = (1 << 63) | 9,
        VirtualSupervisorExternalInterrupt = (1 << 63) | 10,
        MachineExternalInterrupt = (1 << 63) | 11,

        SupervisorGuestExternalInterrupt = (1 << 63) | 12,

        // interrupt = 0
        InstructionAddrMisaligned = 0,
        InstructionAccessFault = 1,
        IllegalInstruction = 2,
        Breakpoint = 3,
        LoadAddressMisaligned = 4,
        LoadAccessFault = 5,
        StoreAMOAddressMisaligned = 6,
        StoreAMOAccessFault = 7,
        EnvCallUorVU = 8,
        EnvCallHS = 9,
        EnvCallVS = 10,
        EnvCallM = 11,
        InstructionPageFault = 12,
        LoadPageFault = 13,
        StoreAMOPageFault = 15,
        InstructionGuestPageFault = 20,
        LoadGuestPageFault = 21,
        VirtualInstruction = 22,
        StoreAMOGuestPageFault = 23,
    }
    pub fn read() -> MCause {
        unsafe { transmute(csrr!("mcause")) }
    }
}

pub mod satp {
    use core::mem::transmute;

    use crate::{arch::paging::Table, util::bits::bits};

    #[derive(Debug)]
    #[repr(u64)]
    pub enum Mode {
        Bare = 0,
        Reserved1 = 1,
        Reserved2 = 2,
        Reserved3 = 3,
        Reserved4 = 4,
        Reserved5 = 5,
        Reserved6 = 6,
        Reserved7 = 7,
        Sv39 = 8,
        Sv48 = 9,
        Sv57 = 10,
        Sv64 = 11,
        Reserved8 = 12,
        Reserved9 = 13,
        Custom1 = 14,
        Custom2 = 15,
    }
    #[derive(Debug)]
    pub struct Satp {
        pub mode: Mode,
        pub asid: u64,
        pub ppn: *mut Table,
    }
    pub fn read() -> Satp {
        let satp = unsafe { csrr!("satp") };
        let mode = unsafe { transmute(bits!(satp;60,63)) };
        let asid = bits!(satp;44,59);
        let ppn = unsafe { transmute(bits!(satp;0,43) << 12) };
        Satp { mode, asid, ppn }
    }
    pub fn write(satp: Satp) {
        let val = (satp.mode as u64) << 60 | satp.asid << 44 | (satp.ppn as u64 >> 12);
        csrw!("satp", val);
    }
}

pub mod mstatus {
    pub fn read() -> u64 {
        unsafe { csrr!("mstatus") }
    }
    pub fn write(val: u64) {
        csrw!("mstatus", val);
    }
}
