OUTPUT_FORMAT(elf64-littleaarch64)

ENTRY(__start)

SECTIONS {
	. = 1024M;
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
	/* linker error: discarded output section: `.got.plt'
	/DISCARD/ : {
		*(*);
	}
	*/
}