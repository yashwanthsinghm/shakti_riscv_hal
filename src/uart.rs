#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! asynchronous serial communication
/// UART

/// UART module provides a two-wire asynchronous serial non-return-to-zero (NRZ)
/// communication with RS-232 (RS-422/485) interface.
///Each UART module has transmit and receive buffers that can hold upto 16 entries.
/// Data transfer rate can be modified by providing appropriate value to UARTBAUD register.
use crate::common::MMIODerefWrapper;
use riscv::{
    asm::{delay, nop},
    register,
};
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// Base address of the UART peripheral register block
pub const UART_OFFSET: usize = 0x0001_1300;

// Status register flags
/// Transmitter full flag
pub const STS_TX_FULL_FLAG: u8 = 0x02;
/// Receiver not empty flag 
pub const STS_RX_NOT_EMPTY_FLAG: u8 = 0x08; 

/// Error flags (also in the status register)
pub const BREAK_ERROR: u8 = 1 << 7;
///Frame error flag
pub const FRAME_ERROR: u8 = 1 << 6;
///Overrun error flag 
pub const OVERRUN: u8 = 1 << 5;
///Parity error flag 
pub const PARITY_ERROR: u8 = 1 << 4;

/// Status register bit positions
/// Receiver full flag
pub const STS_RX_FULL: u8 = 1 << 3; 
/// Receiver not empty flag
pub const STS_RX_NOT_EMPTY: u8 = 1 << 2;
/// Transmitter full flag
pub const STS_TX_FULL: u8 = 1 << 1; 
/// Transmitter empty flag
pub const STS_TX_EMPTY: u8 = 1 << 0; 


register_structs! {
    #[allow(non_snake_case)]
    /// Register blocks 
    pub RegistersBlock {
        /// UART Baud Rate Register (UBR)
        (0x00 => UBR: ReadWrite<u16>),

        /// Reserved register (0x02)
        (0x02 => _reserved0),

        /// UART Transmit Register (TX_REG)
        (0x04 => TX_REG: WriteOnly<u32>),

        /// UART Receive Register (RCV_REG)
        (0x08 => RCV_REG: ReadOnly<u32, RCV_REG::Register>),

        /// UART Status Register (USR)
        (0x0C => USR: ReadOnly<u8, USR::Register>),

        /// Reserved register (0x0D)
        (0x0D => _reserved1),

        /// UART Line Delay Register (DELAY)
        (0x10 => DELAY: ReadWrite<u32, DELAY::Register>),

        /// UART Control Register (UCR)
        (0x14 => UCR: ReadWrite<u32, UCR::Register>),

        /// UART Interrupt Enable Register (IEN)
        (0x18 => IEN: ReadWrite<u32, IEN::Register>),

        /// UART Integer Qualifier Cycles Register (IQCYCLES)
        (0x1C => IQCYCLES: ReadWrite<u32, IQCYCLES::Register>),

        /// UART Receive Threshold Register (RX_THRESHOLD)
        (0x20 => RX_THRESHOLD: WriteOnly<u32, RX_THRESHOLD::Register>),

        /// End marker
        (0x24 => @END),
    }
}


register_bitfields! {
    u32,

    /// UART Baud Register
    UBR [
        BAUD OFFSET(0) NUMBITS(16) []
    ],

    /// UART Status register

    /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.

    USR [

        ///Break Error : Sets when the data and stop are both zero
        BREAK_ERROR OFFSET(7) NUMBITS(1) [],


        ///Frame Error : Sets when the stopis zero
        FRAME_ERROR OFFSET(6) NUMBITS(1) [],



        ///Overrun Error : A data overrun error occurred in the receive
        ///shift register. This happens when additional data arrives
        ///while the FIFO is full.

        OVERRUN OFFSET(5) NUMBITS(1) [],


        ///Parity Error: Sets when The receive character does not have correct parity information and is suspect.
        PARITY_ERROR OFFSET(4) NUMBITS(1) [],


        ///Receiver Full (Sets when the Receive Buffer is Full)
        STS_RX_FULL OFFSET(3) NUMBITS(1) [],


        /// Receiver Not Empty (Sets when there is some data in the
        ///Receive Buffer).
        STS_RX_NOT_FULL OFFSET(2) NUMBITS(1) [],


        ///Transmitter Full (Sets when the transmit Buffer is full)
        STS_TX_FULL OFFSET(1) NUMBITS(1) [
            EMPTY = 0,
            FULL = 1,
        ],


        /// Transmitter Empty(Sets when the Transmit Buffer is empty).
        STS_TX_EMPTY OFFSET(0) NUMBITS(1) [
            EMPTY = 1,
            FULL = 0,
        ]

    ],

    ///Character size of data. Maximum length is 32 bits.
    UCR [
        UART_TX_RX_LEN OFFSET(5) NUMBITS(6) [],


        /// Insert Parity bits
        /// 00 - None
        /// 01 - Odd
        /// 10- Even
        /// 11 - Unused or Undefined

        PARITY OFFSET(3) NUMBITS(2) [
            None = 0b00,
            Odd = 0b01,
            Even = 0b10,
            Unused = 0b11

        ],

        /// Stop bits
        /// 00 - 1 Stop bits
        /// 01 - 1.5 Stop bits
        /// 10 - 2 Stop bits

        STOP_BITS OFFSET(1) NUMBITS(2) [

            StopBits1 = 0b00,
            StopBits1_5 = 0b01,
            StopBits2 = 0b10


        ],
    ],

    TX_REG [

        TX_DATA OFFSET(0) NUMBITS(32) []
    ],


    RCV_REG [

        RX_DATA OFFSET(0) NUMBITS(32) []
    ],

    IEN [

        ENABLE_TX_EMPTY OFFSET(0) NUMBITS(1) [],
        ENABLE_TX_FULL OFFSET(1) NUMBITS(1) [],
        ENABLE_RX_NOT_EMPTY OFFSET(2) NUMBITS(1) [],
        ENABLE_RX_FULL OFFSET(3) NUMBITS(1) [],
        ENABLE_PARITY_ERROR OFFSET(4) NUMBITS(1) [],
        ENABLE_OVERRUN OFFSET(5) NUMBITS(1) [],
        ENABLE_FRAME_ERROR OFFSET(6) NUMBITS(1) [],
        ENABLE_BREAK_ERROR OFFSET(7) NUMBITS(1) [],
        ENABLE_RX_THRESHOLD OFFSET(8) NUMBITS(1) []
    ],

///Delayed Transmit control is done by providing the required delay in UART DELAY register.

      DELAY [
        COUNT OFFSET(0) NUMBITS(8) []
      ],

///UART IQCYCLES Register holds the number of input qualification cycles for the receiver pin. .

      IQCYCLES[
        COUNT OFFSET(0) NUMBITS(8) []
      ],

/// UART RX_THRESHOLD register holds the receiver FIFO threshold value, when the RX FIFO
        ///level increases beyond the threshold, corresponding status bit will be set and when interrupt is
       ///enabled, interrupt will be raised

    RX_THRESHOLD [
        FIFO_RX OFFSET(0) NUMBITS(8) []
    ]

}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegistersBlock>;

/// Internal representation of a UART peripheral
pub struct UartInner {
    /// Memory-mapped registers for interacting with the UART hardware
    registers: Registers,
}

impl UartInner {
    /// Creates a new instance of UartInner, unsafe due to direct hardware access
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                /// Initializes the registers with the provided memory-mapped address
                registers: Registers::new(mmio_start_addr),
            }
        }
    }



    // raw access ==================================================

    /// Writes a single character to the UART.
    ///
    /// This function writes a single character (`char`) to the UART transmit register.
    /// It waits until the UART transmit buffer is not full before writing the character.
    ///
    /// Method arguments:
    /// - c : The character to be written to the UART.
    ///
    /// Returns:
    /// - NONE

    pub fn write_uart_char(&mut self, c: char) {
        unsafe {
            //let status = (*self.registers).USR.get().eq(&0x00);
            //let status = ;

            while match (*self.registers).USR.get() & STS_TX_FULL_FLAG {
                0x02 => true,
                _ => false,
            } {
                //(*self.registers).TX_REG.s
                //     self.registers.TX_REG.set(TX_REG::TX_DATA::CLEAR.into());
                // TX_REG::TX_DATA::CLEAR;
                //let value = (*self.registers).USR.get();
                // delay(10);
                // nop();
            }
            self.registers.TX_REG.set(c as u32);
            // let value  = self.registers.USR.read(USR::STS_RX_FULL);

            //self.print_register_value();
        }
    }

    /// Writes a string of characters to the UART.
    ///
    /// This function takes a string slice (`&str`) as input and iterates over
    /// its bytes, writing each byte as a character to the UART.
    ///
    /// Method arguments:
    /// - message : A reference to the string slice to be written to the UART.
    ///
    /// Returns:
    /// - NONE

    pub fn write_uart_string(&mut self, message: &str) {
        for i in message.as_bytes() {
            self.write_uart_char(*i as char);
        }
    }

    /// Prints the value of the UART status register.
    ///
    /// This function reads the value of the UART status register and prints it.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - value : The value of the UART status register as a u8.

    pub fn print_register_value(&mut self) -> u8 {
        unsafe {
            let value = (*self.registers).USR.get();
            //  baud_value
            // let val = self.registers.USR.get();
            self.registers.TX_REG.set(value.into());
            value
        }
    }

    /// Reads a character from the UART receiver buffer.
    ///
    /// Waits until a character is available in the receiver buffer and then reads it.
    /// Once a character is read, it's transmitted through the UART transmitter buffer.
    ///
    /// Method arguments:
    /// - NONE
    ///
    /// Returns:
    /// - NONE

    pub fn read_uart_char(&mut self) {
        while match (*self.registers).USR.get() & STS_RX_NOT_EMPTY_FLAG {
            0x08 => false,
            _ => true,
        } {}

        while match (*self.registers).USR.get() & STS_TX_FULL_FLAG {
            0x02 => true,
            _ => false,
        } {}
        self.registers.TX_REG.set(self.registers.RCV_REG.get())
    }

}