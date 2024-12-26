MEMORY
{
  /* TODO Adjust memory regions to match device memory layout */
  FLASH : ORIGIN = 0x8000000, LENGTH = 512K 
  RAM   : ORIGIN = 0x20000000, LENGTH = 32K
  /* SRAM1 : ORIGIN = 0x20000000, LENGTH = 80K */
  SRAM2 : ORIGIN = 0x20014000, LENGTH = 16K
  CCM   : ORIGIN = 0x10000000, LENGTH = 32K
}