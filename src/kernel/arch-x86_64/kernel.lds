OUTPUT_FORMAT(elf32-i386)

ENTRY(__start32)

SECTIONS {
    . = 1M;
    .header : {
        *(.header);
        __kbaddr = LOADADDR(.header);
        __keaddr = LOADADDR(.bss) + SIZEOF(.bss);
    }
    .text : {
        *(.text);
    }
    .rodata : {
        *(.rodata);
    }
    .data : {
        *(.data);
    }
    .bss : {
        *(.bss);
    }
}
