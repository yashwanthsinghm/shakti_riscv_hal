MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  /* We'll need prepend a 256-byte rustBoot header. So add an offset - 0x100 */
  RAM_TEXT      (rwx) : ORIGIN = 0x80000000, LENGTH = 0x400000
  RAM_RODATA    (rwx) : ORIGIN = 0x80400000, LENGTH = 0x100000
  RAM_DATA      (rwx) : ORIGIN = 0x80500000, LENGTH = 0x100000
  RAM_BSS       (rwx) : ORIGIN = 0x80600000, LENGTH = 0x100000
  RAM_HEAP      (rwx) : ORIGIN = 0x80700000, LENGTH = 0x100000
  RAM_STACK     (rwx) : ORIGIN = 0x80800000, LENGTH = 0x100000
}

REGION_ALIAS("REGION_TEXT", RAM_TEXT);
REGION_ALIAS("REGION_RODATA", RAM_RODATA);
REGION_ALIAS("REGION_DATA", RAM_DATA);
REGION_ALIAS("REGION_BSS", RAM_BSS);
REGION_ALIAS("REGION_HEAP", RAM_HEAP);
REGION_ALIAS("REGION_STACK", RAM_STACK);

INCLUDE vajra.x