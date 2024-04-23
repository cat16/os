use core::mem::MaybeUninit;

use crate::{mem::alloc::ALLOCATOR, print, println, qemu, StartInfo};

mod mem;

// SURELY I will not have threading tests that cause problems
// SURELY I don't need to pin the input to test_runner
static mut TESTS: &[&dyn Testable] = &[];
static mut TEST: usize = 0;
static mut FAILED: usize = 0;

static mut START_INFO: MaybeUninit<StartInfo> = MaybeUninit::uninit();

pub trait Testable {
    fn run(&self, info: &StartInfo) -> ();
}

// bruh...

// impl<T: Fn()> Testable for T {
//     fn run(&self, _: &StartInfo) {
//         // TODO: very temp solution that fails if names are changed lmao
//         const START: &str = "kernel::test::";
//         let name = &core::any::type_name::<T>()[START.len()..];
//         print!("test {}... ", name);
//         self();
//         println!("\x1b[92mok\x1b[0m");
//     }
// }

impl<T: Fn(&StartInfo)> Testable for T {
    fn run(&self, info: &StartInfo) {
        // TODO: very temp solution that fails if names are changed lmao
        const START: &str = "kernel::test::";
        let name = &core::any::type_name::<T>()[START.len()..];
        print!("test {}... ", name);
        self(info);
        println!("\x1b[92mok\x1b[0m");
    }
}

pub fn init(info: StartInfo) {
    unsafe {
        *START_INFO.as_mut_ptr() = info;
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    unsafe {
        TESTS = core::mem::transmute(tests);
    }
    println!("Running {} tests", tests.len());
    run_tests();
}

pub fn run_tests() -> ! {
    let info = unsafe {START_INFO.assume_init_ref()};
    unsafe {
        for i in TEST..TESTS.len() {
            let test = TESTS[i];
            TEST += 1;
            prepare(info);
            test.run(info);
        }
        print!(
            "results: {}. {} passed; {} failed",
            if FAILED > 0 {
                "\x1b[91mFAILED\x1b[0m"
            } else {
                "\x1b[92mok\x1b[0m"
            },
            TEST - FAILED,
            FAILED
        );
    }
    println!();
    qemu::exit(0)
}

fn prepare(info: &StartInfo) {
    unsafe {
        ALLOCATOR.reset(&info.mem_range);
    }
}

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    println!("\x1b[91mFAILED\x1b[0m");
    println!("\x1b[93m{}\x1b[0m", info);
    unsafe { FAILED += 1 };
    run_tests();
}
