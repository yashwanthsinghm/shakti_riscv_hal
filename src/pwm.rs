#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

/// PULSE WIDTH MODULATOR

/// Pulse Width Modulators (PWM) are used to generate pulses with variable duty cycle.
/// The duty cycle and the period of the pulse can be varied through DUTY and PERIOD registers respectively.
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

/// Base address of the PWM peripheral register block
pub const PWM_OFFSET: usize = 0x0003_0000;

/// PWM register bit masks
/// Reset counter bit
pub const RESET_COUNTER: u8 = 1 << 7;        
/// Interrupt enable bit
pub const INTERRUPT: u8 = 1 << 5;            
/// PWM output enable bit
pub const PWM_OUTPUT_ENABLE: u8 = 1 << 4;     
/// Continuous or one-shot mode bit
pub const CONTINOUS_ONCE: u8 = 1 << 3;       
/// Start PWM generation bit 
pub const PWMSTART: u8 = 1 << 2;             
/// Enable PWM module bit
pub const PWM_ENABLE: u8 = 1 << 1;            
/// Clock source selection bit
pub const CLOCK_SELECT: u8 = 1 << 0;          

register_structs! {
    #[allow(non_snake_case)]
    ///Register Block 
    pub RegistersBlock {
        /// PWM Period Register (PERIOD_REGISTOR)
        (0x00 => PERIOD_REGISTOR: ReadWrite<u16>),

        /// Reserved register (0x02)
        (0x02 => _reserved0),

        /// PWM Duty Cycle Register (DUTY_REGISTER)
        (0x04 => DUTY_REGISTER: ReadWrite<u16>),

        /// Reserved register (0x06)
        (0x06 => _reserved1),

        /// PWM Control Register (CONTROL_REGISTER)
        (0x08 => CONTROL_REGISTER: ReadWrite<u8>),

        /// Reserved register (0x09)
        (0x09 => _reserved2),

        /// PWM Clock Register (CLOCK_REGISTER)
        (0x0C => CLOCK_REGISTER: ReadWrite<u16>),

        /// Reserved register (0x0E)
        (0x0E => _reserved3),

        /// End marker
        (0x1C => @END),
    }
}



//Note:
//PWM Frequency = system clock / (2 * prescaler reg * period reg )
//= 50000000/(2 * 61440 * 240)
//= 1.688Hz
//PWM Period = 1 / PWM frequency
//= 1 / 1.688Hz = 0.589824 seconds
//PWM On time = (PWM Duty reg * PWM period ) / ( PWM Period reg)
//= 128 * 0.589824 / 240 = 0.3145728 seconds

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

/// Internal representation of a PWM peripheral
pub struct PWMInner {
   /// Memory-mapped registers for interacting with the PWM hardware
    registers: Registers,
}

impl PWMInner {
    /// Creates a new instance of PWMInner, unsafe due to direct hardware access
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                // Initializes the registers with the provided memory-mapped address
                registers: Registers::new(mmio_start_addr),
            }
        }
    }
}
