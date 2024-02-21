#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! I2C two-wire interface

/// I2C is a serial protocol for a two-wire interface to connect low-speed devices like
/// microcontrollers, EEPROMs, A/D and D/A converters, I/O interfaces and
/// other similar peripherals in embedded systems.
/// Multiple slave devices can be connected to a single master with I2C.
/// I2C only uses two wires to transmit data between devices
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

/// Base address of the I2C peripheral register block
pub const I2C_OFFSET: usize = 0x0004_0000;

/// I2C_INI Register bit masks
/// Initialization bit
pub const I2C_INI: u8 = 1 << 7; 
/// Status bit
pub const I2C_STS: u8 = 1 << 5; 
/// Bus Error bit
pub const I2C_BER: u8 = 1 << 4; 
/// Address/Data 0 (Read/Write bit)
pub const I2C_AD0_LRB: u8 = 1 << 3;
/// Acknowledge Address/Stop bit 
pub const I2C_AAS: u8 = 1 << 2; 
/// Last Bit bit
pub const I2C_LAB: u8 = 1 << 1;
/// Bus Busy bit 
pub const I2C_BB: u8 = 1 << 0; 

// I2C_PIN Register bit masks
/// Pin Select bit
pub const I2C_PIN: u8 = 1 << 7; 
/// Enable Slave 0 bit
pub const I2C_ES0: u8 = 1 << 6; 
/// Enable Interrupt (Read/Write bit)
pub const I2C_ENI_LRB: u8 = 1 << 3;
/// Start condition bit 
pub const I2C_STA: u8 = 1 << 2; 
/// Stop condition bit
pub const I2C_STO: u8 = 1 << 1; 
/// Acknowledge bit
pub const I2C_ACK: u8 = 1 << 0; 

register_structs! {
    #[allow(non_snake_case)]
    ///Register Block 
    pub RegistersBlock {
        /// Prescaler register (PRESCALE)
        (0x00 => PRESCALE: ReadWrite<u16>),

        /// Reserved register (0x02)
        (0x02 => _reserved0),

        /// Control register (CONTROL)
        (0x08 => CONTROL: ReadWrite<u8>),

        /// Reserved register (0x09)
        (0x09 => _reserved1),

        /// Data register (DATA)
        (0x10 => DATA: ReadWrite<u8>),

        /// Reserved register (0x11)
        (0x11 => _reserved2),

        /// Status register (STATUS)
        (0x18 => STATUS: ReadWrite<u8>),

        /// Reserved register (0x19)
        (0x19 => _reserved3),

        /// Serial Clock register (SCL)
        (0x38 => SCL: ReadWrite<u8>),

        /// Reserved register (0x39)
        (0x39 => _reserved4),

        /// End marker
        (0x3C => @END),
    }
}


register_bitfields! {
    u32,

    /// I2C Prescale Register divides the System clock by (Prescale value + 1).
    /// This clock is used as clock input for I2C Serial Clock.
    /// I2C Prescaler clock = System Clock / (Prescaler Value + 1)

    PRESCALE [
        PRESCALE_VALUE OFFSET(0) NUMBITS(8) []
    ],

    ///I2C SCL Register divides the I2C Prescaler clock by (SCL value + 1). This clock is used as
    ///I2C SCL clock = I2C Prescaler Clock / (SCL COUNT + 1).

    SCL[
        SCL_COUNT OFFSET(0) NUMBITS(8) []
    ],

    ///High when I2C communication in progress. Becomes low once I2C communication is complete.

    STATUS [

        I2C_INI OFFSET(7) NUMBITS(1) [],

        /// When in slave receiver mode, this flag is asserted when an
        /// externally generated STOP condition is detected (used only in
        /// slave receiver mode).

        I2C_STS OFFSET(5) NUMBITS(1) [],

        ///Bus error; a misplaced START or STOP condition has been detected

        I2C_BER OFFSET(4) NUMBITS(1) [],

        ///AD0(Address 0) - General Call bit used for Broadcast

        ///LRB - Last Received Bit through I2C Bus
        ///This status bit serves a dual function, and is valid only while
        ///PIN = 0:

        ///1. LRB holds the value of the last received bit over the
        ///I2C-bus while AAS = 0 (not addressed as slave).
        ///Normally this will be the value of the slave acknowledgement; thus
        ///checking for slave acknowledgement is done via testing of the
        ///LRB.

        ///2. AD0; when AAS = 1 (‘Addressed As Slave’ condition), the
        ///I2C-bus controller has been addressed as a slave.
        ///Under this condition, this bit becomes the ‘AD0’ bit and will be set to
        ///logic 1 if the slave address received was the ‘general call’
        ///(00H) address, or logic 0 if it was the I2C-bus controller’s own
        ///slave address.

        I2C_AD0_LRB OFFSET(3) NUMBITS(1) [],

        ///Addressed As Slave bit.
        ///Valid only when PIN = 0. When acting as slave receiver, this flag is set when an incoming
        ///address over the I2C-bus matches the value in own address register

        I2C_AAS OFFSET(2) NUMBITS(1) [],

        ///Lost Arbitration Bit.
        ///This bit is set when, in multi-master
        ///operation, arbitration is lost to another master on the I2C-bus

        I2C_LAB OFFSET(1) NUMBITS(1) [],

        ///Bus Busy bit.
        ///This is a read-only flag indicating when the I2C-bus is in use.
        ///A zero indicates that the bus is busy, and access is not possible

        I2C_BB OFFSET(0) NUMBITS(1) []

    ],

    CONTROL [

    ///Pending Interrupt Not, Used as a software reset.

    I2C_PIN OFFSET(7) NUMBITS(1) [],

    /// Enable Serial Output
    ///0 - Registers can be initialized.
    ///1 - I2C Serial Transmission

    I2C_ES0 OFFSET(6) NUMBITS(1) [
        REGISTER_INITIALIZED = 0,
        I2C_SERIAL_TRANSMISSION = 1,
    ],

    ///Enables the external interrupt output, which is generated when the PIN is active (Low)

    I2C_ENI OFFSET(3) NUMBITS(1) [],

    ///Transmits Start condition + Slave address..

    I2C_STA OFFSET(2) NUMBITS(1) [],

    ///Transmits the stop condition.

    I2C_STO OFFSET(1) NUMBITS(1) [],

    ///Acknowledgement bit:
    ///1: I2C automatically sends an acknowledgement after a read/write transaction.
    ///0: I2C Master sends Negative Acknowledge to stop the I2C transfer

    I2C_ACK OFFSET(0) NUMBITS(1) [
        NAK = 0,
        ACK = 1,
    ]

],


}

type Registers = MMIODerefWrapper<RegistersBlock>;

/// Internal representation of an I2C peripheral
pub struct I2CInner {
    /// Memory-mapped registers for interacting with the I2C hardware
    registers: Registers,
}

impl I2CInner {
    /// Creates a new instance of I2CInner, unsafe due to direct hardware access
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                // Initializes the registers with the provided memory-mapped address
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}

