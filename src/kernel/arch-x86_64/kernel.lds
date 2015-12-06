OUTPUT_FORMAT(elf32-i386)

ENTRY(__start32)

SECTIONS {
	. = 1M;
	.header : {
		*(.header);
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
