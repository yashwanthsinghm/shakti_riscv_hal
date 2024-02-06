/* device.x */

OUTPUT_ARCH( "riscv" )
ENTRY(_start)
/* Define sections from the linker script */
SECTIONS
{
    . = ORIGIN(RAM);

    .text.init :
    {
        KEEP(*(.text.init))
    } > REGION_TEXT

    .text :
    {
        KEEP(*(.text))
    } > REGION_TEXT

    .rodata :
    {
        __rodata_start = .;
        *(.rodata)
        *(.rodata.*)
        *(.gnu.linkonce.r.*)
        __rodata_end = .;
    } > REGION_RODATA

    .sdata :
    {
        __global_pointer$ = . + 0x800;
        *(.srodata.cst16)
        *(.srodata.cst8)
        *(.srodata.cst4)
        *(.srodata.cst2)
        *(.srodata*)
        *(.sdata .sdata.* .gnu.linkonce.s.*)
    } > REGION_DATA AT > REGION_RODATA

    .sbss :
    {
        __sbss_start = .;
        *(.sbss)
        *(.sbss.*)
        *(.gnu.linkonce.sb.*)
        __sbss_end = .;
    } > REGION_BSS

    .data :
    {
        . = ALIGN(4);
        __data_start = .;
        *(.data)
        *(.data.*)
        *(.gnu.linkonce.d.*)
        __data_end = .;
    } > REGION_DATA AT > REGION_RODATA

    .bss :
    {
        . = ALIGN(4);
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end = .;
    } > REGION_BSS

    .tdata :
    {
        _tls_data = .;
        *(.tdata.begin)
        *(.tdata)
        *(.tdata.end)
        _tls_end = .;
    } > REGION_DATA

    .tbss :
    {
        *(.tbss)
        *(.tbss.end)
    } > REGION_BSS

    . = ALIGN(4);
    _end = .;

    . = 0x80000000 + 0x8000000 - 0x400;
    _free_space = . - _end;
    _STACK_SIZE = (_free_space * 50) / 100 ;
    _HEAP_SIZE = _free_space - _STACK_SIZE;

    .stack :
    {
        _stack_end = .;
        __stack_pointer$ = .;
        _stack = . - _STACK_SIZE;
    } > REGION_STACK

    . = _stack;

    .heap :
    {
        _heap = . - _HEAP_SIZE;
        _heap_end = .;
    } > REGION_HEAP
}
