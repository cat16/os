OUTPUT_ARCH( "riscv" )
ENTRY( _start )
SECTIONS
{
    . = 0x80000000;
    .text : {
        *(.text.init)
        *(.text)
    }
    PROVIDE(global_pointer = .);
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.sdata .sdata.*) *(.data .data.*)
    }
    .bss : {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    }
    . += 0x8000;
    . = ALIGN(16);
    PROVIDE(stack_top = .);
}

