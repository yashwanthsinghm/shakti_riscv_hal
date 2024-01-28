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

pub const I2C_OFFSET: usize = 0x0200_0000;

register_structs! {
    #[allow(non_snake_case)]
    pub RegistersBlock{
        (0xBFF8 => MTIME: ReadWrite<u64>),
        (0x4000 => MTIMECMP: ReadWrite<u64>),
        (0x0C => @END),
    }
}

register_bitfields! {
    u32,
/*
A memory mapped register of a real time counter.
Address offset: 0xBFF8
*/
    MTIME [
        MTIME_REG OFFSET(0) NUMBITS(32) []
    ],

    /*
    Machine mode timer compare register which causes a timer interrupt to be posted when the
mtime register contains a value greater than or equal to the value in the mtimecmp register
Address offset: 0x4000

mtimecmp = mtime + N * (Clock frequency / mtime divisor)
N - Number of seconds after which the interrupt is generated.
Clock Frequency - Frequency at which device clock in running
mtime divisor - Value of this divisor varies as per board
Pinaka - 16
Parashu - 256
Vajra - 256
 */
    MTIMECMP[
        MTIMECMP_REG OFFSET(0) NUMBITS(32) []
    ],

}


type Registers = MMIODerefWrapper<RegistersBlock>;

pub struct CNINTInner {
    registers: Registers,
}

impl CNINTInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}