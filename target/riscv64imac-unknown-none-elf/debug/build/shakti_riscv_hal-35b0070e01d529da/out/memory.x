MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  /* We'll need prepend a 256-byte rustBoot header. So add an offset - 0x100 */
#  FLASH    (rx)  : ORIGIN = 0x80000000, LENGTH = 16M
  RAM      (rwx) : ORIGIN = 0x80000000, LENGTH = 128K 
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

INCLUDE link.x