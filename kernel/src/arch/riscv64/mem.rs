use super::asm::linker_static;

linker_static!(PROGRAM_END: usize, ".dword _end");
