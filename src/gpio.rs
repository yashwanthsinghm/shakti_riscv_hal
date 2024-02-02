//! General purpose input and output
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


///Sequence of execution:
///1. Write into the GPIO Direction register to configure GPIO pin as an input or output.
///2. Write appropriate values to the GPIO DATA register.

/// The General Purpose Input/output operation can be used to generate custom
///waveforms, enable signals, generate interrupts, etc. The GPIO has a GPIO DIRECTION register
///which configures the GPIO pin as an input or output and the GPIO DATA register which holds
///the input data to GPIO or output data from GPIO. The GPIO pins 0 - 7 can accept External
///events as interrupts. To use a GPIO pin (0 - 7) as interrupt, that particular GPIO pin(s) should be
///configured as input. The GPIO data register is 1 byte, 2 byte and 4 byte accessible.

impl GPIOInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
    pub fn set_direction_control(&mut self,value: u32) {
        self.registers.DIRECTION_CR_REG.set(value);
    }

    pub fn set_data_register(&mut self,value: u32) {
        self.registers.DATA_REG.set(value);
    }
}
