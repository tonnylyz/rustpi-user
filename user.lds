SECTIONS
{
    . = 0x40000;
    .text : {
        *(.text*)
    }
    .rodata : {
        *(.rodata*)
    }
    .data : {
        *(.data*)
    }
    .bss : {
        *(.bss*)
    }
    /DISCARD/ : {
        *(.comment)
    }
}
