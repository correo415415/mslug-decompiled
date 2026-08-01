/*
 * Metal Slug 1 - Linker Script para binario 100% matched bit-a-bit
 * ==================================================================
 */

OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)

MEMORY
{
    ROM (rx) : ORIGIN = 0x000000, LENGTH = 0x200000
}

SECTIONS
{
    /* Bank 1: Vectores + Header + Codigo */
    .vectors 0x000000 : { KEEP(*(.vectors)) } > ROM
    .carthead 0x000100 : { KEEP(*(.carthead)) } > ROM
    .bank1_code 0x000200 :
    {
        KEEP(*(.text.bank1_code))
    } > ROM

    /* Bank 2: Codigo (empieza en 0x100000 del binario, mapeado a CPU $200000) */
    .bank2_code 0x100000 :
    {
        KEEP(*(.bank2_code))
    } > ROM

    /DISCARD/ :
    {
        *(.text)
        *(.text.*)
        *(.data)
        *(.data.*)
        *(.bss)
        *(.bss.*)
        *(.rodata)
        *(.rodata.*)
        *(.comment)
        *(.note.*)
        *(.eh_frame)
        *(COMMON)
    }
}
