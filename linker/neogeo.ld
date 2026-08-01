OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)
ENTRY(_start)

MEMORY
{
    ROM1 (rx)  : ORIGIN = 0x000000, LENGTH = 0x100000
    RAM   (rw)  : ORIGIN = 0x100000, LENGTH = 0x00F000
    BIOSRAM (rw): ORIGIN = 0x10F300, LENGTH = 0x000D00
    ROM2 (rx)  : ORIGIN = 0x200000, LENGTH = 0x100000
}

SECTIONS
{
    .vectors 0x000000 :
    {
        KEEP(*(.vectors))
    } > ROM1

    .carthead 0x000100 :
    {
        KEEP(*(.carthead))
    } > ROM1

    .text 0x000204 :
    {
        *(.text)
        *(.text.*)
        *(.rodata)
        *(.rodata.*)
    } > ROM1

    .data 0x100000 :
    {
        *(.data)
        *(.data.*)
    } > RAM

    .bss (NOLOAD) :
    {
        *(.bss)
        *(.bss.*)
        *(COMMON)
    } > RAM

    .bank2 0x200000 :
    {
        KEEP(*(.bank2))
    } > ROM2

    _stack_top = 0x10F300;
    _ram_start = 0x100000;
    _ram_end   = 0x10F000;
}
