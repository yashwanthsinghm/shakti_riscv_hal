#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

/// external spi flash
use crate::spi::{self, SPIInner};

pub struct Flash {
    spi: SPIInner,
}

impl Flash {
    pub fn new(spi: SPIInner) -> Self {
        Flash { spi }
    }

    /// Write Enable Latch (WEL) bit setting.
    ///
    /// This function sets the Write Enable Latch (WEL) bit in the status register of the flash.
    /// Before modifying the content of the flash, the WEL bit must be enabled.
    ///
    /// Warning:
    /// Without enabling this bit, one cannot erase/write into the flash.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn flash_write_enable(&mut self) {}

    /// Flash device ID retrieval.
    ///
    /// This function retrieves the device ID of the flash.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn flash_device_id(&mut self) {

        // need to implement further
    }

    /// Status register clearing.
    ///
    /// This function resets the status register of the flash by clearing its bits.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn flash_clear_sr(&mut self) {}

    /// Bit extraction function.
    ///
    /// This function extracts k number of bits from a (p-1) position in a 32-bit number.
    ///
    /// Method arguments:
    /// - number : The 32-bit number.
    /// - k : The number of bits to be extracted.
    /// - p : The position from where the bits should be extracted.
    ///
    /// Returns:
    /// - 32 bit which have k bit from "number" and rest are zero

    pub fn bit_extracted(number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /// Command and address transmission.
    ///
    /// This function is useful for sending an 8-bit command followed by a 32-bit address
    /// through the SPI interface. Useful for a function like erase.
    ///
    /// Warning:
    /// to move data drom dr register to fifo there must be some data into spi_dr5
    ///
    /// Method arguments:
    /// - command : The 8-bit command (opcode).
    /// - addr : The 32-bit address after the command.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_cmd_addr(&mut self, command: u32, addr: u32) {}

    /// Command, address, and data transmission.
    ///
    /// Used for sending 8bit command +32bit of write address + 32 bit of write data
    ///
    /// Warning:
    /// to move data from data register to fifo there must be some data into spi_dr5
    ///
    /// Method arguments:
    /// - command : The 8-bit command (opcode).
    /// - addr : The address after the command.
    /// - data : The data after the address.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_cmd_addr_data(&mut self, command: u32, addr: u32, data: u32) {

        //flash_status_register_read();
    }

    /// Flash memory writing.
    ///
    /// This function writes 4 bytes of data to the specified address in the flash memory. flash_cmd_addr_data with opcode 12h.
    ///
    /// Warning:
    /// before writing into the flash one should enable the WEL bit spi_sr by using write_enable(void)
    ///
    /// Method arguments:
    /// - address : The write address in the flash memory.
    /// - data : The data to be written.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_write(&mut self, address: u32, data: u32) {}

    /// Command and address transmission for read operation.
    ///
    /// For sending command of 8bit + read address of 32bit + 8bit of dummy cycle and receive
    /// 32bit value from flash
    ///
    /// Warning:
    ///  As receive shoild start as soon as transmit state end, use spi_rx_tx_start()
    /// Before setting control register 1
    ///
    /// Method arguments:
    /// - `command`: The 8-bit command (opcode).
    /// - `addr`: The read address after the command.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_cmd_to_read(&mut self, command: u32, addr: u32) {}

    /// Flash memory reading.
    ///
    /// This function reads 4 bytes of data from the specified address in the flash memory.
    ///
    /// Method arguments:
    /// - address : The read address in the flash memory.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_read(&mut self, address: u32) {}

    /// Command transmission for read operation.
    ///
    /// This function is useful for reading status register and for sending an 8-bit command for a read operation
    /// and receiving 32 bits of data through the SPI interface.
    ///
    /// Method arguments:
    /// - command : The 8-bit command (opcode).
    ///
    /// Returns:
    /// - NONE

    pub fn flash_cmd_read(&mut self, command: u32) {}

    /// Bit extraction function
    ///
    /// This function extracts a specified number of bits from a given position in a 32-bit number.
    ///
    /// Method arguments:
    /// - number : The 32-bit number.
    /// - k : The number of bits to be extracted.
    /// - p : The position from where the bits should be extracted.
    ///
    /// Returns:
    /// - u32 : A 32-bit integer with the extracted bits and zeros in other positions.

    pub fn bitEXtracted(&mut self, number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /// Flash memory erasing.
    ///
    /// This function erases a 64KB sector from the specified address in the flash memory.
    ///
    /// Warning:
    /// Before erasing the flash one should enable the WEL bit spi_sr by using write_enable()
    ///
    /// Method arguments:
    /// - address : The address from which the data should be erased.
    ///
    /// Returns:
    /// - NONE

    pub fn flash_erase(&mut self, address: u32) {}
}
