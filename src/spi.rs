#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

/// SPI serial communication
/// SPI is a synchronous serial I/O port that allows a serial bit stream of programmed length to be
/// shifted into and out of the device at programmable bit transfer rate
use riscv::asm::delay;
use tock_registers::{
    fields::FieldValue,
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::common::MMIODerefWrapper;

use self::SPI_CR1::SPI_TOTAL_BITS_TX;

/// Base address of the SPI peripheral register block
pub const SPI_OFFSET: usize = 0x0002_0000;

register_bitfields! {
    u32,

    SPI_CR1 [

        /// Expected Total Number of bits to be received.

        SPI_TOTAL_BITS_RX OFFSET(24) NUMBITS(8) [

        ],

        /// Total Number of bits to be transmitted.

        SPI_TOTAL_BITS_TX OFFSET(16) NUMBITS(8) [

        ],

        /// Bidirectional data mode enable. This bit enables
        /// half-duplex communication using a common single
        /// bidirectional data line.
        /// 0: 2-line unidirectional data mode selected
        /// 1: 1-line unidirectional data mode selected

        SPI_BIDIMODE OFFSET(15) NUMBITS(1) [
            OUTPUT_ENABLE = 1,
            PUTPUT_DISABLE = 0,
        ],

        /// Output enable in bidirectional mode This bit combined with
        /// the BIDI-MODE bit selects the direction of transfer in bi-direction mode.
        /// 0: receive-only mode (Output Disabled)
        /// 1: transmit-only mode (Output Enabled)

        SPI_BIDIODE OFFSET(14) NUMBITS(1) [
            OUTPUT_ENABLE = 1,
            PUTPUT_DISABLE = 0,
        ],


        /// Hardware CRC calculation Enable.
        /// 0: CRC calculation disable
        /// 1: CRC calculation enable

        SPI_CRCEN OFFSET(13) NUMBITS(1) [
            HWCRC_ENABLE = 1,
            HWCRC_DISABLE = 0,
        ],

        /// Transmit CRC Next.
        /// 0: Next Transmit value is from Tx buffer
        /// 1: Next Transmit value is from Rx buffer

        SPI_CCRCNEXT OFFSET(12) NUMBITS(1) [
            CRCNEXT_RX = 1,
            CRCNEXT_TX = 0,
        ],

        /// CRC Length bit is set and cleared by software to select CRC Length

        SPI_CRCL OFFSET(11) NUMBITS(1) [

        ],

        /// Receive only mode enabled. This bit enables simplex
        /// communication using a single unidirectional line to
        /// receive data exclusively. Keep BIDIMODE bit clear when
        /// receiving the only mode is active.

        SPI_RXONLY OFFSET(10) NUMBITS(1) [

        ],

        /// Software Slave Management. When the SSM bit is set,
        /// the NSS pin input is replaced with the value from the SSI
        /// bit.
        /// 0: Software slave management disabled
        /// 1: Software slave management enabled

        SPI_SSM OFFSET(9) NUMBITS(1) [
            SSM_ENABLED  = 1,
            SSM_DISABLED = 0,
        ],

        /// Internal Slave Select.This bit has an effect only when the
        /// SSM bit is set. The value of this bit is forced onto the
        /// NSS pin and the I/O value of the NSS pin is ignored

        SPI_SSI OFFSET(8) NUMBITS(1) [

        ],

        /// Frame Format
        /// 0: data is transmitted/received with the MSB first
        /// 1: data is transmitted/received with the LSB first
        /// Note: This bit should not be changed when communication is ongoing

        SPI_LSBFIRST OFFSET(7) NUMBITS(1) [
            LSB_FIRST = 1,
            MSB_FIRST = 0,
        ],

        /// SPI Enable
        /// 0: SPI is disabled
        /// 1: SPI is enabled

        SPI_SPE OFFSET(6) NUMBITS(1) [
            ENABLED  = 1,
            DISABLED = 0,
        ],

        /// Baud Rate Control
        /// 000: fCLK/2
        /// 001: fCLK/4
        /// 010: fCLK/8
        /// 011: fCLK/16
        /// 100: fCLK/32
        /// 101: fCLK/64
        /// 110: fCLK/128
        /// 111: fCLK/256
        /// Note:This bit should not be changed when communication is ongoing

        SPI_BR OFFSET(3) NUMBITS(3) [

        ],

        /// Master Selection
        /// 0: Slave Configuration
        /// 1: Master Configuration
        /// Note This bit should not be changed when communication is ongoing

        SPI_MSTR OFFSET(2) NUMBITS(1) [
            SLAVE_CONFIG   = 1,
            MASTER_CONFIG  = 0,
        ],

        ///Clock Polarity
        ///0: CLK is 0 when idle
        ///1: CLK is 1 when idle

        SPI_CPOL OFFSET(1) NUMBITS(1) [
            ONE_IDLE   = 1,
            ZERO_IDLE  = 0,
        ],

        ///Clock Phase
        ///0: The first clock transition is the first data capture edge
        ///1: The second clock transition is the first data capture edge

        SPI_CPHA OFFSET(0) NUMBITS(1) [
            SECOND_CLK = 1,
            FIRST_CLK  = 0,
        ]
    ],

    SPI_CR2 [
        ///SPI_TOTAL_BITS_RX OFFSET(24) NUMBITS(7) [],

        SPI_RX_IMM_START OFFSET(16) NUMBITS(1) [],
        SPI_RX_START OFFSET(15) NUMBITS(1) [],
        SPI_LDMA_TX_START OFFSET(14) NUMBITS(1) [],
        SPI_LDMA_RX OFFSET(13) NUMBITS(1) [],

        /// FIFO reception threshold is used to set the threshold of
        /// the RXFIFO that triggers an RXNE event.
        /// 0: RXNE event is generated if the FIFO level is greater
        /// than or equal to 1/2 (16-bit)
        /// 1: RXNE event is generated if the FIFO level is greater
        /// than or equal to 1/4 (8-bit)

        SPI_FRXTH OFFSET(12) NUMBITS(1) [],

        /// Reserved bits
        /// SPI_DS OFFSET(8) NUMBITS(4) [],

        /// Interrupt enable for TXE event.
        /// 0: TXE interrupt masked
        /// 1: TXE interrupt is not interrupt masked

        SPI_TXEIE OFFSET(7) NUMBITS(1) [],

        /// Interrupt enable for RXNE event
        /// 0: RXNE interrupt masked
        /// 1: RXNE interrupt is not interrupt masked

        SPI_RXNEIE OFFSET(6) NUMBITS(1) [
            RXNE_UNMASKED = 1,
            RXNE_MASKED = 0,
        ],

        /// when an error condition occurs.
        /// 0: Error interrupt masked
        /// 1: Error interrupt not masked.
        //Frame Error (Sets when the stopis zero)

        SPI_ERRIE OFFSET(5) NUMBITS(1) [
            MASKED_INT   = 1,
            UNMASKED_INT = 0,
        ],

        /// Reserved bits
        /// SPI_FRF OFFSET(4) NUMBITS(1) [],
        /// SPI_NSSP OFFSET(3) NUMBITS(1) [],

        /// SS output enable
        /// 0: SS output is disabled in master mode and the SPI interface
        /// can work in a multi-master configuration
        /// 1: SS output is enabled in master mode and when the SPI
        /// interface is enabled. The SPI interface cannot work in a
        /// multi-master environment.

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

    ///Data registers

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
    ///Register Block  
    pub RegisterBlock {
        /// SPI Control Register 1 (SPI_CR1)
        (0x00 => SPI_CR1: ReadWrite<u32, SPI_CR1::Register>),

        /// SPI Control Register 2 (SPI_CR2)
        (0x04 => SPI_CR2: ReadWrite<u32, SPI_CR2::Register>),

        /// SPI Status Register (SPI_SR)
        (0x08 => SPI_SR: ReadOnly<u32, SPI_SR::Register>),

        /// SPI Data Register 1 (SPI_DR1)
        (0x0C => SPI_DR1: ReadWrite<u32, SPI_DR1::Register>),

        /// SPI Data Register 2 (SPI_DR2)
        (0x10 => SPI_DR2: ReadWrite<u32, SPI_DR2::Register>),

        /// SPI Data Register 3 (SPI_DR3)
        (0x14 => SPI_DR3: ReadWrite<u32, SPI_DR3::Register>),

        /// SPI Data Register 4 (SPI_DR4)
        (0x18 => SPI_DR4: ReadWrite<u32, SPI_DR4::Register>),

        /// SPI Data Register 5 (SPI_DR5)
        (0x1C => SPI_DR5: ReadWrite<u32, SPI_DR5::Register>),

        /// Reserved register (0x20)
        (0x20 => _reserved),

        /// End marker
        (0x2C => @END),
    }
}


/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

/// Internal representation of an SPI peripheral
pub struct SPIInner {
    /// Memory-mapped registers for interacting with the SPI hardware
    registers: Registers,
}

impl SPIInner {
    /// Creates a new instance of SPIInner, unsafe due to direct hardware access
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                /// Initializes the registers with the provided memory-mapped address
                registers: Registers::new(mmio_start_addr),
            }
        }
    }



    /// Function to initialize the SPI controller.

    /// By setting up baud rate and clock pole and phase. Initialize the spi controller in Mode 3 (CPOL =1 & CPHA =1) with SCK= clk/16
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn init(&mut self) {
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE + SPI_CR1::SPI_CPHA::SECOND_CLK + SPI_CR1::SPI_BR.val(7),
        );
    }

    /// Function to start receiving data as soon as the transmit state is complete.
    /// While receiving data from flash (reading Device ID, status register and reading flash)  
    /// in master mode use this function.

    /// Warning:
    /// Should be set before configuring the control register 1.

    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn spi_tx_rx_start(&mut self) {
        self.registers
            .SPI_CR2
            .modify(SPI_CR2::SPI_RX_IMM_START::SET);
    }

    /// Function to enable receive state. This is not in used when spi is in Master mode
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn spi_rx_enable(&mut self) {
        self.registers.SPI_CR2.modify(SPI_CR2::SPI_RX_START::SET);
    }

    /// Function to set the WEL (Write Enable Latch) bit in the status register.

    /// Before modifying content of flash, one should enable the WEL bit first
    /// Warning:
    /// Without enabling this bit one cannot erase/write into the flash

    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

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

    /// Function to read the device ID from flash.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - dr5: Device ID

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

    /// Function to reset the status register.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - u32: Always returns 1

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

    /// Function to extract a k number of bits from a (p-1) position of a 32 bit number.
    ///
    /// Method arguments:
    /// - number : Number
    /// - k : Number of bits to extract
    /// - p : Position from where the bits to be extracted
    ///
    /// Returns:
    /// - u32: 32 bit which have k bit from "number" and rest are zero

    pub fn bit_extracted(number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /// Function to send an 8-bit command + 32-bit address. Can be used for erase.
    ///
    /// Warning:
    /// to move data drom dr register to fifo there must be some data into spi_dr5

    /// Method arguments:
    /// - command : opcode
    /// - address : address after opcode
    ///
    /// Returns:
    /// - u32: Always returns 1

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
        unsafe {
            delay(1000);
        }
        self.spi_not_busy();
        // while self
        //     .registers
        //     .SPI_SR
        //     .any_matching_bits_set(SPI_SR::SPI_BSY::SET)
        // {}
        1
    }

    /// Function to send an 8-bit command + 32-bit address + 32-bit data.
    ///
    /// Warning:
    ///to move data from data register to fifo there must be some data into spi_dr5

    /// Method arguments:
    /// - command : opcode
    /// - address : address after opcode
    /// - data : data after the address
    ///
    /// Returns:
    /// - NONE

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
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(0)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(72),
        );
        unsafe { delay(1000) };
        self.spi_not_busy();
        //waitfor(20);
        // while self
        //     .registers
        //     .SPI_SR
        //     .any_matching_bits_set(SPI_SR::SPI_BSY::SET)
        // {}

        self.flash_status_register_read();
    }

    /// Function to write 4 bytes of data from a given address. flash_cmd_addr_data with opcode 12h.
    ///
    /// Warning:
    /// before writing into the flash one should enable the WEL bit spi_sr
    /// by using write_enable(void)

    /// Method arguments:
    /// - address : write address
    /// - data : write data
    ///
    /// Returns:
    /// - NONE

    pub fn flash_write(&mut self, address: u32, data: u32) {
        SPIInner::flash_write_enable(self);
        SPIInner::flash_cmd_addr_data(self, 0x12000000, address, data);
        // flash_status_register_read();
    }

    /// Function to send a 8 bit command for reading data from flash.
    /// for sending command of 8bit + read address of 32bit + 8bit of dummy cycle
    /// and receive 32bit value from flash

    /// Warning:
    /// As receive shoild start as soon as transmit state end, use spi_rx_tx_start()
    /// Before setting control register 1

    /// Method arguments:
    /// - command : opcode
    /// - address : address to be read
    ///
    /// Returns:
    /// - dr5 : value read

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

    /// Function to read 4 bytes of data from a given address. flash_cmd_to_read with opcode 0Bh for fast read
    ///
    /// Method arguments:
    /// - address : address to be read from
    ///
    /// Returns:
    /// - read_value: Value that is read

    pub fn flash_read(&mut self, address: u32) -> u32 {
        let read_value = SPIInner::flash_cmd_to_read(self, 0x0C000000, address);
        read_value
    }

    /// Function to send a 8 bit command for reading the status register.
    ///
    /// Method arguments:
    /// - command : (opcode)
    ///
    /// Returns:
    /// - dr5 : Flash response to opcode

    pub fn flash_cmd_read(&mut self, command: u32) -> u32 {
        let mut dr5 = 0;
        self.registers.SPI_DR1.set(command);
        self.registers.SPI_DR5.set(command);
        self.spi_tx_rx_start();
        self.registers.SPI_CR1.modify(
            SPI_CR1::SPI_CPOL::ONE_IDLE
                + SPI_CR1::SPI_CPHA::SECOND_CLK
                + SPI_CR1::SPI_BR.val(7)
                + SPI_CR1::SPI_SPE::ENABLED
                + SPI_CR1::SPI_TOTAL_BITS_RX.val(32)
                + SPI_CR1::SPI_TOTAL_BITS_TX.val(8),
        );

        if self.spi_rxne_enable() {
            dr5 = self.registers.SPI_DR5.get();
        }
        dr5
    }

    /// Function to extract a specified number of bits from a given position of a number.
    ///
    /// Method arguments:
    /// - number: The 32-bit integer from which bits are to be extracted.
    /// - k: number of bits to be extracted
    /// - p: position
    ///
    /// Returns:
    /// - u32: The bits extracted

    pub fn bitEXtracted(&mut self, number: u32, k: u32, p: u32) -> u32 {
        ((1 << k) - 1) & (number >> (p - 1))
    }

    /// Function to erase the flash. Erase the 64kb sector from given address
    ///
    /// WARNING:
    /// before erasing the flash one should enable the WEL bit spi_sr by using write_enable()

    /// Method arguments:
    /// - u32: Address (address from which data should erase)
    ///
    /// Returns:
    /// - NONE

    pub fn flash_erase(&mut self, address: u32) {
        self.flash_cmd_addr(0xdc000000, address);
    }

    /// Function to enable RXNE interrupt and wait until RXNE flag is set.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - bool: Always returns true

    pub fn spi_rxne_enable(&mut self) -> bool {
        while match self.registers.SPI_SR.get() & 0x01 {
            0x00 => true,
            _ => false,
        } {}
        true
    }

    /// Function to wait until SPI is not busy.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn spi_not_busy(&mut self) {
        while match self.registers.SPI_SR.get() & 0x80 {
            0x80 => true,
            _ => false,
        } {
            unsafe {
                delay(1000);
            }
        }
    }

    ///Function to read the status register
    /// Method arguments:
    /// -  NONE
    ///
    /// Returns:
    /// -  NONE

    pub fn flash_status_register_read(&mut self) {
        while match self.flash_cmd_read(0x05000000) & 0x03 {
            0x03 => true,
            _ => false,
        } {}
    }

}