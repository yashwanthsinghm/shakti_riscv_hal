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

pub const I2C_OFFSET: usize = 0x0004_0000;

pub const I2C_INI: u8 = 1 << 7;
pub const I2C_STS: u8 = 1 << 5;
pub const I2C_BER: u8 = 1 << 4;
pub const I2C_AD0_LRB: u8 = 1 << 3;
pub const I2C_AAS: u8 = 1 << 2;
pub const I2C_LAB: u8 = 1 << 1;
pub const I2C_BB: u8 = 1 << 0;

pub const I2C_PIN: u8 = 1 << 7;
pub const I2C_ES0: u8 = 1 << 6;
pub const I2C_ENI_LRB: u8 = 1 << 3;
pub const I2C_STA: u8 = 1 << 2;
pub const I2C_STO: u8 = 1 << 1;
pub const I2C_ACK: u8 = 1 << 0;

register_structs! {
    #[allow(non_snake_case)]
    pub RegistersBlock{
        (0x00 => PRESCALE: ReadWrite<u16>),
        (0x02 => _reserved0),
        (0x08 => CONTROL: ReadWrite<u8>),
        (0x09 => _reserved1),
        (0x10 => DATA: ReadWrite<u8>),
        (0x11 => _reserved2),
        (0x18 => STATUS : ReadWrite<u8>),
        (0x19 => _reserved3),
        (0x38 => SCL : ReadWrite<u8>),
        (0x39 => _reserved4)
,       (0x3C => @END),
    }
}

register_bitfields! {
    u32,

    PRESCALE [
        PRESCALE_VALUE OFFSET(0) NUMBITS(8) []
    ],
    SCL[
        SCL_COUNT OFFSET(0) NUMBITS(8) []
    ],
    STATUS [

        I2C_INI OFFSET(7) NUMBITS(1) [],

        I2C_STS OFFSET(5) NUMBITS(1) [],

        I2C_BER OFFSET(4) NUMBITS(1) [],

        I2C_AD0_LRB OFFSET(3) NUMBITS(1) [],

        I2C_AAS OFFSET(2) NUMBITS(1) [],

        I2C_LAB OFFSET(1) NUMBITS(1) [],

        I2C_BB OFFSET(0) NUMBITS(1) []

    ],

    CONTROL [

    I2C_PIN OFFSET(7) NUMBITS(1) [],

    I2C_ES0 OFFSET(6) NUMBITS(1) [
        REGISTER_INITIALIZED = 0,
        I2C_SERIAL_TRANSMISSION = 1,
    ],

    I2C_ENI OFFSET(3) NUMBITS(1) [],

    I2C_STA OFFSET(2) NUMBITS(1) [],

    I2C_STO OFFSET(1) NUMBITS(1) [],

    I2C_ACK OFFSET(0) NUMBITS(1) [
        NAK = 0,
        ACK = 1,
    ]

],


}

type Registers = MMIODerefWrapper<RegistersBlock>;

pub struct I2CInner {
    registers: Registers,
}

impl I2CInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}