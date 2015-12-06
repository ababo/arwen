SECTIONS {
    .header : {
        *(.header);
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
