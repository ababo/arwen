OUTPUT_FORMAT(elf32-i386)

ENTRY(__start32)

SECTIONS {
	. = 1M;
	.kboot : {
		*(.kboot);
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
