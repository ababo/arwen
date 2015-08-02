OUTPUT_FORMAT(elf64-littleaarch64)

ENTRY(__start)

SECTIONS {
    . = 1024M;
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
