use crate::{print, println, qemu};

mod mem;

// SURELY I will not have threading tests that cause problems
// SURELY I don't need to pin the input to test_runner
static mut TESTS: &[&dyn Testable] = &[];
static mut TEST: usize = 0;
static mut FAILED: usize = 0;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        print!("test {}... ", core::any::type_name::<T>());
        self();
        println!("\x1b[92mok\x1b[0m");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    unsafe { TESTS = core::mem::transmute(tests) };
    println!("Running {} tests", tests.len());
    run_tests();
}

pub fn run_tests() -> ! {
    unsafe {
        for i in TEST..TESTS.len() {
            let test = TESTS[i];
            TEST += 1;
            test.run();
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

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    println!("\x1b[91mFAILED\x1b[0m");
    println!("\x1b[93m{}\x1b[0m", info);
    unsafe { FAILED += 1 };
    run_tests();
}
