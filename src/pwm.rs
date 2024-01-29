//! Pulse Width Modulators (PWM) are used to generate pulses with variable duty cycle
/**
 * Pulse Width Modulators (PWM) are used to generate pulses with variable duty cycle.
The duty cycle and the period of the pulse can be varied through DUTY and PERIOD registers
respectively.
 * 
 */


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
        (0x06 => _reserved1),
        (0x08 => CONTROL_REGISTER: ReadWrite<u8>),
        (0x09 => _reserved2),
        (0x0C => CLOCK_REGISTER : ReadWrite<u16>),
        (0x0E => _reserved3),
        (0x1C => @END),
    }
}


///Note:
///PWM Frequency = system clock / (2 * prescaler reg * period reg )
///= 50000000/(2 * 61440 * 240)
///= 1.688Hz
///PWM Period = 1 / PWM frequency
///= 1 / 1.688Hz = 0.589824 seconds
///PWM On time = (PWM Duty reg * PWM period ) / ( PWM Period reg)
///= 128 * 0.589824 / 240 = 0.3145728 seconds
register_bitfields! {
    u32,
///PWM PERIOD_REGISTER further divides the system clock by the (Prescaler value + 1).
    PERIOD_REGISER [
        PWM_PERIOD OFFSET(0) NUMBITS(16) []
    ],
    ///PWM Period = 1 / PWM frequency
    ///PWM Frequency = system clock / (2 * prescaler reg * period reg )
    DUTY_REGISTER[
        PWM_DUTY OFFSET(0) NUMBITS(16) []
    ],
    CONTROL_REGISTER [
        ///PWM Reset
        RESET_COUNTER OFFSET(7) NUMBITS(1) [],
    ///0: Interrupt not Occured.
///1: Interrupt occurred.
        INTERRUPT OFFSET(5) NUMBITS(1) [
            INTERRUPT_NOT_OCCURED = 0,
            INTERRUPT_OCCURED = 1,
        ],
///0: Disable PWM output
///1: Enable PWM output
        PWM_OUTPUT_ENABLE OFFSET(4) NUMBITS(1) [
            DISABLE_PWM_OUTPUT = 0,
            ENABLE_PWM_OUTPUT = 1,
        ],
///0: In timer mode, continuous mode is Off.
///1: In timer mode, continuous mode is ON.


        CONTINOUS_ONCE OFFSET(3) NUMBITS(1) [
            COUNTINOUS_MODE_OFF = 0,
            COUNTINOUS_MODE_ON  = 1,

        ],
///1:Start PWM Operation

        PWMSTART OFFSET(2) NUMBITS(1) [],
///0: Timer mode enable
///1: PWM enable
        PWM_ENABLE OFFSET(1) NUMBITS(1) [
            TIMER_NODE_ENABLE = 0,
            PWM_ENABLE = 1,
        ],
///0: Internal clock source selected
///1:External clock source selected
        CLOCK_SELECT OFFSET(0) NUMBITS(1) [
            INTERNAL_CLOCK_SOURCE_SELECTED = 0,
            EXTERNAL_CLOCK_SOURCE_SELECTED = 1,
        ]

    ],
///PWM CLOCK_REGISTER divides the system clock by the (Prescaler value + 1).
    CLOCK_REGISTER[
        PWM_PRESCALER OFFSET(1) NUMBITS(16) []
    ],


}


type Registers = MMIODerefWrapper<RegistersBlock>;

pub struct PWMInner {
    registers: Registers,
}

impl PWMInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}