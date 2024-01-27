use riscv::asm::delay;
use tock_registers::{
    fields::FieldValue,
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::common::MMIODerefWrapper;

use self::SPI_CR1::SPI_TOTAL_BITS_TX;

pub const SPI_OFFSET: usize = 0x0002_0000;

register_bitfields! {
    u32,

    SPI_CR1 [

        // Expected Total Number of bits to be received.
        SPI_TOTAL_BITS_RX OFFSET(24) NUMBITS(8) [

        ],

        // Total Number of bits to be transmitted.
        SPI_TOTAL_BITS_TX OFFSET(16) NUMBITS(8) [

        ],

        // Bidirectional data mode enable. This bit enables
        // half-duplex communication using a common single
        // bidirectional data line.
        // 0: 2-line unidirectional data mode selected
        // 1: 1-line unidirectional data mode selected
        SPI_BIDIMODE OFFSET(15) NUMBITS(1) [
            OUTPUT_ENABLE = 1,
            PUTPUT_DISABLE = 0,
        ],

        // Output enable in bidirectional mode This bit combined with
        // the BIDI-MODE bit selects the direction of transfer in bi-direction mode.
        // 0: receive-only mode (Output Disabled)
        // 1: transmit-only mode (Output Enabled)
        SPI_BIDIODE OFFSET(14) NUMBITS(1) [
            OUTPUT_ENABLE = 1,
            PUTPUT_DISABLE = 0,
        ],


        // Hardware CRC calculation Enable.
        // 0: CRC calculation disable
        // 1: CRC calculation enable
        SPI_CRCEN OFFSET(13) NUMBITS(1) [
            HWCRC_ENABLE = 1,
            HWCRC_DISABLE = 0,
        ],

        // Transmit CRC Next.
        // 0: Next Transmit value is from Tx buffer
        // 1: Next Transmit value is from Rx buffer
        SPI_CCRCNEXT OFFSET(12) NUMBITS(1) [
            CRCNEXT_RX = 1,
            CRCNEXT_TX = 0,
        ],

        // CRC Length bit is set and cleared by software to select CRC Length
        SPI_CRCL OFFSET(11) NUMBITS(1) [

        ],

        // Receive only mode enabled. This bit enables simplex
        // communication using a single unidirectional line to
        // receive data exclusively. Keep BIDIMODE bit clear when
        // receiving the only mode is active.
        SPI_RXONLY OFFSET(10) NUMBITS(1) [

        ],

        // Software Slave Management. When the SSM bit is set,
        // the NSS pin input is replaced with the value from the SSI
        // bit.
        // 0: Software slave management disabled
        // 1: Software slave management enabled
        SPI_SSM OFFSET(9) NUMBITS(1) [
            SSM_ENABLED  = 1,
            SSM_DISABLED = 0,
        ],

        // Internal Slave Select.This bit has an effect only when the
        // SSM bit is set. The value of this bit is forced onto the
        // NSS pin and the I/O value of the NSS pin is ignored
        SPI_SSI OFFSET(8) NUMBITS(1) [

        ],

        // Frame Format
        // 0: data is transmitted/received with the MSB first
        // 1: data is transmitted/received with the LSB first
        // Note: This bit should not be changed when communication is ongoing
        SPI_LSBFIRST OFFSET(7) NUMBITS(1) [
            LSB_FIRST = 1,
            MSB_FIRST = 0,
        ],

        // SPI Enable
        // 0: SPI is disabled
        // 1: SPI is enabled
        SPI_SPE OFFSET(6) NUMBITS(1) [
            ENABLED  = 1,
            DISABLED = 0,
        ],

        // Baud Rate Control
        // 000: fCLK/2
        // 001: fCLK/4
        // 010: fCLK/8
        // 011: fCLK/16
        // 100: fCLK/32
        // 101: fCLK/64
        // 110: fCLK/128
        // 111: fCLK/256
        // Note:This bit should not be changed when communication is ongoing
        SPI_BR OFFSET(3) NUMBITS(3) [

        ],

        // Master Selection
        // 0: Slave Configuration
        // 1: Master Configuration
        // Note This bit should not be changed when communication is ongoing
        SPI_MSTR OFFSET(2) NUMBITS(1) [
            SLAVE_CONFIG   = 1,
            MASTER_CONFIG  = 0,
        ],

        //Clock Polarity
        //0: CLK is 0 when idle
        //1: CLK is 1 when idle
        SPI_CPOL OFFSET(1) NUMBITS(1) [
            ONE_IDLE   = 1,
            ZERO_IDLE  = 0,
        ],

        //Clock Phase
        //0: The first clock transition is the first data capture edge
        //1: The second clock transition is the first data capture edge
        SPI_CPHA OFFSET(0) NUMBITS(1) [
            SECOND_CLK = 1,
            FIRST_CLK  = 0,
        ]
    ],

    SPI_CR2 [
        //SPI_TOTAL_BITS_RX OFFSET(24) NUMBITS(7) [],
        SPI_RX_IMM_START OFFSET(16) NUMBITS(1) [],
        SPI_RX_START OFFSET(15) NUMBITS(1) [],
        SPI_LDMA_TX_START OFFSET(14) NUMBITS(1) [],
        SPI_LDMA_RX OFFSET(13) NUMBITS(1) [],

        // FIFO reception threshold is used to set the threshold of
        // the RXFIFO that triggers an RXNE event.
        // 0: RXNE event is generated if the FIFO level is greater
        // than or equal to 1/2 (16-bit)
        // 1: RXNE event is generated if the FIFO level is greater
        // than or equal to 1/4 (8-bit)
        SPI_FRXTH OFFSET(12) NUMBITS(1) [],

        // Reserved bits
        // SPI_DS OFFSET(8) NUMBITS(4) [],

        // Interrupt enable for TXE event.
        // 0: TXE interrupt masked
        // 1: TXE interrupt is not interrupt masked
        SPI_TXEIE OFFSET(7) NUMBITS(1) [],

        // Interrupt enable for RXNE event
        // 0: RXNE interrupt masked
        // 1: RXNE interrupt is not interrupt masked
        SPI_RXNEIE OFFSET(6) NUMBITS(1) [
            RXNE_UNMASKED = 1,
            RXNE_MASKED = 0,
        ],

        // when an error condition occurs.
        // 0: Error interrupt masked
        // 1: Error interrupt not masked.
        //Frame Error (Sets when the stopis zero)
        SPI_ERRIE OFFSET(5) NUMBITS(1) [
            MASKED_INT   = 1,
            UNMASKED_INT = 0,
        ],

        // Reserved bits
        // SPI_FRF OFFSET(4) NUMBITS(1) [],
        // SPI_NSSP OFFSET(3) NUMBITS(1) [],

        // SS output enable
        // 0: SS output is disabled in master mode and the SPI interface
        // can work in a multi-master configuration
        // 1: SS output is enabled in master mode and when the SPI
        // interface is enabled. The SPI interface cannot work in a
        // multi-master environment.
        SPI_SSOE OFFSET(2) NUMBITS(1) [
            SSOE_ENABLED  = 1,
            SSOE_DISABLED = 0,
        ],

        // Reserved bits
        // SPI_TXDMAEN OFFSET(1) NUMBITS(1) [],
        // SPI_RXDMAEN OFFSET(0) NUMBITS(1) [],

    ],

    SPI_SR[

        SPI_FTLVL OFFSET(11) NUMBITS(2) [],
        SPI_FRLVL OFFSET(9) NUMBITS(2) [],

        SPI_FRE OFFSET(8) NUMBITS(1) [],
        SPI_BSY OFFSET(7) NUMBITS(1) [],
        SPI_OVR OFFSET(6) NUMBITS(1) [],

        SPI_MODF OFFSET(5) NUMBITS(1) [],

        SPI_CRCERR OFFSET(4) NUMBITS(1) [],

        SPI_TXE OFFSET(1) NUMBITS(1) [],

        SPI_RXNE OFFSET(0) NUMBITS(1) []

    ],

    SPI_DR1 [
        DR1 OFFSET(0) NUMBITS(32) []
    ],

    SPI_DR2 [
        DR2 OFFSET(0) NUMBITS(32) []
    ],

    SPI_DR3 [
        DR3 OFFSET(0) NUMBITS(32) []
    ],

    SPI_DR4 [
        DR4 OFFSET(0) NUMBITS(32) []
    ],

    SPI_DR5 [
        DR5 OFFSET(0) NUMBITS(32) []
    ],


}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock{
        (0x00 => SPI_CR1: ReadWrite<u32, SPI_CR1::Register>),
        (0x04 => SPI_CR2: ReadWrite<u32, SPI_CR2::Register>),
        (0x08 => SPI_SR:  ReadOnly <u32, SPI_SR ::Register>),
        (0x0C => SPI_DR1: ReadWrite <u32, SPI_DR1::Register>),
        (0x10 => SPI_DR2: ReadWrite <u32, SPI_DR2::Register>),
        (0x14 => SPI_DR3: ReadWrite <u32, SPI_DR3::Register>),
        (0x18 => SPI_DR4: ReadWrite <u32, SPI_DR4::Register>),
        (0x1C => SPI_DR5: ReadWrite <u32, SPI_DR5::Register>),
        (0x20 => _reserved),
        (0x2C => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

pub struct SPIInner {
    registers: Registers,
}

impl SPIInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }

    /** @fn void spi_init(void)
     * @brief setting up baud rate and clock pole and phase
     * @details Initialize the spi controller in Mode 3 (CPOL =1 & CPHA =1) with SCK= clk/16
     */
    pub fn init(&mut self) {
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE + SPI_CR1::SPI_CPHA::SECOND_CLK + SPI_CR1::SPI_BR.val(7),
        );
    }

    /** @fn  void spi_tx_rx_start(void)
     * @brief to start receiving data as soon as transmit state is complete
     * @details While receiving data from flash (reading Device ID, status register and reading flash)  
     *           in master mode use this function.
     * @warning Should be set before configuring the control register 1.
     */
    pub fn spi_tx_rx_start(&mut self) {
        self.registers
            .SPI_CR2
            .modify(SPI_CR2::SPI_RX_IMM_START::SET);
    }

    /** @fn void spi_rx_enable(void)
     * @brief to start receive state
     * @details This is not in used when spi is in Master mode
     */
    pub fn spi_rx_enable(&mut self) {
        self.registers.SPI_CR2.modify(SPI_CR2::SPI_RX_START::SET);
    }

    /**
     * @fn int flash_write_enable(void)
     * @brief to set the WEL (Write Enable Latch) bit in status register
     * @details Before modifying content of flash, one should enable the WEL bit first
     * @warning Without enabling this bit one cannot erase/write into the flash
     * @return int
     */
    pub fn flash_write_enable(&mut self) {
        self.registers.SPI_DR1.modify(SPI_DR1::DR1.val(0x06000000));
        self.registers.SPI_DR5.modify(SPI_DR5::DR5.val(0x06));
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(0)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(8),
        );
        self.spi_not_busy();
    }
    /*
    int flash_device_id(void)
    {
        int dr3;
        int val1, val2;

        flash_write_enable();
        set_spi(spi_dr1, 0x9f000000);
        set_spi(spi_dr5, 0x9f000000);
        spi_tx_rx_start();
        set_spi(spi_cr1, (SPI_BR(7)|SPI_TOTAL_BITS_TX(8)|SPI_TOTAL_BITS_RX(24)|SPI_SPE|SPI_CPHA|SPI_CPOL));

        if(spi_rxne_enable())
        {
            dr3 = *spi_dr5;
        }

        val1 = bitExtracted(dr3, 8, 17);
        val2 = bitExtracted(dr3, 16, 1);

        log_debug("Device ID %x \n", val1);
        log_debug("extracted device id %x \n",val2);

        return 1;
    }
     */
    pub fn flash_device_id(&mut self) -> u32 {
        let mut dr5 = 0;
        self.flash_write_enable();
        self.registers.SPI_DR1.modify(SPI_DR1::DR1.val(0x9F000000));
        self.registers.SPI_DR5.modify(SPI_DR5::DR5.val(0x9F000000));
        self.spi_tx_rx_start();
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(8)
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(24)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_CPOL::ONE_IDLE,
        );
        if self.spi_rxne_enable() {
            dr5 = self.registers.SPI_DR5.get();
        }
        dr5
        // need to implement further
    }

    /**
     * @fn int flash_clear_sr(void)
     * @brief to reset the status register
     * @details It will reset the bits of status register
     * @return int
     */
    pub fn flash_clear_sr(&mut self) -> u32 {
        self.registers.SPI_DR1.set(0x30000000);
        self.registers.SPI_DR5.set(0x30);
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::SET
                + SPI_CR1::SPI_TOTAL_BITS_RX::CLEAR
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(8),
        );
        while self
            .registers
            .SPI_SR
            .any_matching_bits_set(SPI_SR::SPI_BSY::SET)
        {}
        1
    }

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
    pub fn flash_cmd_addr(&mut self, command: u32, addr: u32) -> u32 {
        let address1 = SPIInner::bit_extracted(addr, 24, 9);
        let mut address2 = SPIInner::bit_extracted(addr, 8, 1);
        let data1 = command | address1;
        address2 = address2 << 24;
        self.registers.SPI_DR1.set(data1);
        self.registers.SPI_DR2.set(address2);
        self.registers.SPI_DR5.set(0x0);
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(0)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(40),
        );

        //waitfor(20);
        while self
            .registers
            .SPI_SR
            .any_matching_bits_set(SPI_SR::SPI_BSY::SET)
        {}
        1
    }

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
        let cmd_addr = command | ((addr & 0xFFFFFF00) >> 8);
        let data1 = ((addr & 0xFF) << 24) | ((data & 0xFFFFFF00) >> 8);
        let data2 = ((data & 0xFF) << 24) & 0xFF000000;

        //log_debug("\n cmd: %x;d1: %x; d2: %x", cmd_addr, data1, data2);

        self.registers.SPI_DR1.set(cmd_addr);
        self.registers.SPI_DR2.set(data1);
        self.registers.SPI_DR3.set(data2);
        self.registers.SPI_DR5.set(0x0);
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::SET
                + SPI_CR1::SPI_TOTAL_BITS_RX::CLEAR
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(72),
        );

        //waitfor(20);
        while self
            .registers
            .SPI_SR
            .any_matching_bits_set(SPI_SR::SPI_BSY::SET)
        {}

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
    pub fn flash_write(&mut self, address: u32, data: u32) {
        SPIInner::flash_write_enable(self);
        SPIInner::flash_cmd_addr_data(self, 0x12000000, address, data);
        // flash_status_register_read();
    }

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
    pub fn flash_cmd_to_read(&mut self, command: u32, addr: u32) -> u32 {
        let mut dr5 = 0;
        let mut address2 = SPIInner::bit_extracted(addr, 8, 1);

        address2 = address2 << 24;
        self.registers
            .SPI_DR1
            .set(command | ((addr & 0xFFFFFF00) >> 8));
        self.registers.SPI_DR2.set((addr & 0xFF) << 24);
        self.registers.SPI_DR5.set(0x0);

        self.spi_tx_rx_start();
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(32)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(48),
        );
        unsafe {
            delay(2000);
        }
        //waitfor(2000);

        if self.spi_rxne_enable() {
            dr5 = self.registers.SPI_DR5.get()
        }
        dr5
    }

    /** @fn int flash_read(int address)
     * @brief read the 4bytes data from given address
     * @details flash_cmd_to_read with opcode 0Bh for fast read
     * @param int (address (read address))
     * @return int
     */
    pub fn flash_read(&mut self, address: u32) -> u32 {
        let read_value = SPIInner::flash_cmd_to_read(self, 0x0C000000, address);
        read_value
    }

    /**
     * @fn int flash_cmd_read(int command)
     * @brief usefull for reading status register
     * @details use for sending 8bit command and receive the 32bit of data
     * @param int command (opcode)
     * @return int  value (flash response to opcode)
     */
    pub fn flash_cmd_read(&mut self, command: u32) -> u32 {
        let mut dr5 = 0;
        self.registers.SPI_DR1.set(command);
        self.registers.SPI_DR5.set(command);

        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(32)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(8),
        );

        if (self.spi_rxne_enable()) {
            dr5 = self.registers.SPI_DR5.get();
        }
     dr5
    }

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
    pub fn flash_erase(&mut self, address: u32) {
        SPIInner::flash_cmd_addr(self, 0xdc000000, address);
    }

    pub fn spi_rxne_enable(&mut self) -> bool {
        while match self.registers.SPI_SR.get() & 0x01 {
            0x00 => true,
            _ => false,
        } {}
        true
    }
    pub fn spi_not_busy(&mut self) {
        while match self.registers.SPI_SR.get() & 0x80 {
            0x80 => true,
            _ => false,
        } {}
    }
    pub fn flash_status_register_read(&mut self) {
        while match self.flash_cmd_read(0x05000000) & 0x03 {
            0x03 => true,
            _ => false,
        } {}
    }
}
