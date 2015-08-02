SECTIONS {
    .kboot : {
        *(.rodata.*arch*boot*);
    }
    .text : {
        *(.text*);
    }
    .rodata : {
        *(.rodata*);
    }
    .data : {
        *(.data*);
    }
    .bss : {
        *(.bss*);
        *(COMMON*);
    }
    /DISCARD/ : {
        *(*);
    }
}
