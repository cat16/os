OUTPUT_ARCH( "riscv" )
ENTRY( _start )
SECTIONS
{
    . = 0x80000000;
    .text : {
        *(.text.init)
        *(.text .text.*)
    }
    PROVIDE(_global_pointer = .);
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.sdata .sdata.*) *(.data .data.*)
    }
    .bss : {
        *(.sbss .sbss.*) *(.bss .bss.*)
    }
    . = ALIGN(16);
    PROVIDE(_stack_start = .);
    . += 0x8000;
    PROVIDE(_stack_end = .);
    . = ALIGN(4096); /* 2^12 */
    PROVIDE(_end = .);
}

