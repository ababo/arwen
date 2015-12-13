OUTPUT_FORMAT(elf64-littleaarch64)

ENTRY(__start)

SECTIONS {
    . = 1024M + 64K /* dtb */;
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
