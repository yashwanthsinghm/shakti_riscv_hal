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

pub const PWM_OFFSET: usize = 0x0003_0000;

pub const RESET_COUNTER: u8 = 1 << 7;
pub const INTERRUPT: u8 = 1 << 5;
pub const PWM_OUTPUT_ENABLE: u8 = 1 << 4;
pub const CONTINOUS_ONCE: u8 = 1 << 3;
pub const PWMSTART: u8 = 1 << 2;
pub const PWM_ENABLE: u8 = 1 << 1;
pub const CLOCK_SELECT: u8 = 1 << 0;

register_structs! {
    #[allow(non_snake_case)]
    pub RegistersBlock{
        (0x00 => PERIOD_REGISTOR: ReadWrite<u16>),
        (0x02 => _reserved0),
        (0x04 => DUTY_REGISTER: ReadWrite<u16>),
        (0x06 => _reserved0),
        (0x08 => CONTROL_REGISTER: ReadWrite<u8>),
        (0x10 => _reserved0),
        (0x0C =>CLOCK_REGISTER : ReadWrite<u16>),
        (0x0E => @END),
    }
}

register_bitfields! {
    u32,

    PERIOD_REGISER [
        PWM_PERIOD OFFSET(0) NUMBITS(16) []
    ],
    DUTY_REGISTER[
        PWM_DUTY OFFSET(0) NUMBITS(16) []
    ],
    CONTROL_REGISTER [

        RESET_COUNTER OFFSET(7) NUMBITS(1) [],

        INTERRUPT OFFSET(5) NUMBITS(1) [
            INTERRUPT_NOT_OCCURED = 0,
            INTERRUPT_OCCURED = 1,
        ],

        PWM_OUTPUT_ENABLE OFFSET(4) NUMBITS(1) [
            DISABLE_PWM_OUTPUT = 0,
            ENABLE_PWM_OUTPUT = 1,
        ],



        CONTINOUS_ONCE OFFSET(3) NUMBITS(1) [
            COUNTINOUS_MODE_OFF = 0,
            COUNTINOUS_MODE_ON  = 1,

        ],


        PWMSTART OFFSET(2) NUMBITS(1) [],

        PWM_ENABLE OFFSET(1) NUMBITS(1) [
            TIMER_NODE_ENABLE = 0,
            PWM_ENABLE = 1,
        ],

        CLOCK_SELECT OFFSET(0) NUMBITS(1) [
            INTERNAL_CLOCK_SOURCE_SELECTED = 0,
            EXTERNAL_CLOCK_SOURCE_SELECTED = 1,
        ]

    ],

    CLOCK_REGISTER[
        PWM_PRESCALER OFFSET(1) NUMBITS(16) []
    ],


}
