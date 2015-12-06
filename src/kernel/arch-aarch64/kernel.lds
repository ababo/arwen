OUTPUT_FORMAT(elf64-littleaarch64)

ENTRY(__start)

SECTIONS {
    . = 1024M + 64K /* dtb */;
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
