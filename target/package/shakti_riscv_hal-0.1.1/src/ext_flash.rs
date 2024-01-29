use crate::spi::{self, SPIInner};

pub struct Flash {
    spi: SPIInner,
}

impl Flash {
    pub fn new(spi: SPIInner) -> Self {
        Flash { spi }
    }
    /**
     * @fn int flash_write_enable(void)
     * @brief to set the WEL (Write Enable Latch) bit in status register
     * @details Before modifying content of flash, one should enable the WEL bit first
     * @warning Without enabling this bit one cannot erase/write into the flash
     * @return int
     */
    pub fn flash_write_enable(&mut self) {}

    pub fn flash_device_id(&mut self) {

        // need to implement further
    }

    /**
     * @fn int flash_clear_sr(void)
     * @brief to reset the status register
     * @details It will reset the bits of status register
     * @return int
     */
    pub fn flash_clear_sr(&mut self) {}

    /**
     * @fn int bitExtracted(int number, int k, int p)
     * @brief Extract the k number of bit from (p-1) position of 'number'
     * @details If one want to extract the k bits from (p-1) position in 32 bit "number".   
     * @param int (number (32 bit))
     * @param int (k (number of bits to be extracted))
     * @param int (p (position from where the bits to be extracted))
     * @return int (32 bit which have k bit from "number" and rest are zero)
     */
    pub fn bit_extracted(number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /**
     * @fn int flash_cmd_addr(int command, int addr)
     * @brief Use for sending 8bit of command + 32 bit of address
     * @details Useful for function like erase
     * @warning to move data drom dr register to fifo there must be some data into spi_dr5
     * @param int (command (opcode))
     * @param int (addr (address after the opcode))
     * @return int
     */
    pub fn flash_cmd_addr(&mut self, command: u32, addr: u32) {}

    /**
     * @fn void flash_cmd_addr_data(int command, int addr, int data)
     * @brief useful for function like Write
     * @details use for sending 8bit command +32bit of write address + 32 bit of write data
     * @warning to move data from data register to fifo there must be some data into spi_dr5
     * @param int (command (opcode))
     * @param int (addr(address after the opcode))
     * @param int (data (data after the address))
     */
    pub fn flash_cmd_addr_data(&mut self, command: u32, addr: u32, data: u32) {

        //flash_status_register_read();
    }

    /**
     * @fn void flash_write(int address, int data)
     * @brief  Write 4bytes of data from given address
     * @details flash_cmd_addr_data with opcode 12h.  
     * @warning before writing into the flash one should enable the WEL bit spi_sr by using write_enable(void)
     * @param int (addres (write address))
     * @param int(data (write data))
     */
    pub fn flash_write(&mut self, address: u32, data: u32) {}

    /**
     * @fn int flash_cmd_to_read(int command, int addr)
     * @briefUse useful for function like read
     * @details for sending command of 8bit + read address of 32bit + 8bit of dummy cycle and receive
     *          32bit value from flash
     * @warning As receive shoild start as soon as transmit state end, use spi_rx_tx_start() Before
     *          setting control register 1
     * @param int (command (opcode))
     * @param int (addr(read_address))
     * @return int
     */
    pub fn flash_cmd_to_read(&mut self, command: u32, addr: u32) {}

    /** @fn int flash_read(int address)
     * @brief read the 4bytes data from given address
     * @details flash_cmd_to_read with opcode 0Bh for fast read
     * @param int (address (read address))
     * @return int
     */
    pub fn flash_read(&mut self, address: u32) {}

    /**
     * @fn int flash_cmd_read(int command)
     * @brief usefull for reading status register
     * @details use for sending 8bit command and receive the 32bit of data
     * @param int command (opcode)
     * @return int  value (flash response to opcode)
     */
    pub fn flash_cmd_read(&mut self, command: u32) {}

    pub fn bitEXtracted(&mut self, number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /**
     * @fn  void flash_erase(int address)
     * @brief Erase the flash
     * @details Erase the 64kb sector from given address
     * @warning before erasing the flash one should enable the WEL bit spi_sr by using write_enable()
     * @param int (address (address from which data should erase))
     */
    pub fn flash_erase(&mut self, address: u32) {}
}
