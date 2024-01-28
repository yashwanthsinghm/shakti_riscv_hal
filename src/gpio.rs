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

pub const I2C_OFFSET: usize = 0x0004_0100;

register_structs! {
    #[allow(non_snake_case)]
    pub RegistersBlock{
        (0x00 => DIRECTION_CR_REG: ReadWrite<u32>),
        (0x04 => _reserved0),
        (0x08 => DATA_REG: ReadWrite<u32>),
        (0x0C => @END),
    }
}

register_bitfields! {
    u32,

    DIRECTION_CR_REG [
        CR OFFSET(0) NUMBITS(32) []
    ],
    SCL[
        DATA_REG OFFSET(0) NUMBITS(32) []
    ],

}

type Registers = MMIODerefWrapper<RegistersBlock>;

pub struct GPIOInner {
    registers: Registers,
}

impl GPIOInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}
