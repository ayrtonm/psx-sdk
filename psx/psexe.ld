OUTPUT_FORMAT(binary)
MAIN_RAM = 0x80000000;
STACK_INIT = 0x801FFF00;
RAM_SIZE = 2M;
RESERVED = 64K;
HEADER_SIZE = 2K;
/* lld doesn't seem to support DEFINED(X) ? Y : Z */
/* DEST_ADDR = DEFINED(DEST_ADDR) ? DEST_ADDR : MAIN_RAM + RESERVED; */
DEST_ADDR = MAIN_RAM + RESERVED;

__text_len = (__text_end - __text_start);
__data_len = (__data_end - __data_start);

MEMORY {
    HEADER     : ORIGIN = MAIN_RAM + RESERVED - HEADER_SIZE, LENGTH = HEADER_SIZE
    /* RAM through the KSEG0 region: 2MB minus 64K reserved for the BIOS */
    RAM (rwx)  : ORIGIN = MAIN_RAM + RESERVED, LENGTH = RAM_SIZE - RESERVED
}

EXTERN(_PSX_EXE)
EXTERN(_REGION)
ENTRY(_start)

SECTIONS {
    .psx_exe_header : {
        /* ASCII ID "PS-X EXE" */
        KEEP(*(.psx_exe));
        /* Zerofilled */
        LONG(0); LONG(0);
        /* Initial PC */
        LONG(ABSOLUTE(_start));
        /* Initial GP/R28 */
        LONG(0);
        /* Destination address in RAM */ 
        LONG(DEST_ADDR);
        /*a Filesize excluding 2K header */
        LONG(__text_len + __data_len);
        /* Unknown/unused */
        LONG(0); LONG(0);
        /* Memfill start and size */
        LONG(0); LONG(0);
        /* Initial SP/R29 & FP/R30 base */
        LONG(STACK_INIT);
        /* Initial SP/R29 & FP/R30 offset */
        LONG(0);
        /* 038h-04Bh Reserved for A(43h) Function (should be zerofilled in exefile) */
        LONG(0); LONG(0);
        LONG(0); LONG(0);
        LONG(0);
        /* 04Ch-xxxh ASCII marker */
        KEEP(*(.region));
        /* xxxh-7FFh Zerofilled */
        . = ALIGN(2048);
    } > HEADER

    __text_start = ABSOLUTE(.);

	.text DEST_ADDR : {
		*(.text*)
	} > RAM

    __text_end = ABSOLUTE(.);

	/* The PSX doesn't have an MMU so we don't really care about
	 * RO vs RW
	 */
    __data_start = ABSOLUTE(.);
	.data : {
		*(.data*)
		*(.rodata*)
        *(.got)
        /* Padding executable to multiple of 2K is required for loading from ISO */
        . = ALIGN(2048);
	} > RAM

    __data_end = ABSOLUTE(.);

	/* This section must be stored in the "memfill" part of the
	 * header to be initialized to 0 at bootup by the BIOS/loader.
	 */
	.bss (NOLOAD) : {
		*(.bss*)
		*(COMMON)
	} > RAM

    . = ALIGN(4);
    . = ADDR(.text) - 2K;

   /DISCARD/ : {
       *(.MIPS.abiflags)
       *(.reginfo)
       *(.eh_frame)
       *(.eh_frame_hdr)
   }
}
