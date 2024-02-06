#![no_std]
#![no_main]
#![feature(asm)]

use riscv::{asm::delay, delay};
// use cortex_m_rt::entry;
use riscv_rt::entry;
use shakti_riscv_hal::gpio::{GPIOInner, GPIO_OFFSET};
use shakti_riscv_hal::uart::{UartInner, UART_OFFSET};



#[entry]
fn main() -> ! {
    let mut uart = unsafe { UartInner::new(UART_OFFSET) };

    uart.write_uart_string("Hello from shakti \n ");
    let d = add_variable(5, 10);

    uart.write_uart_string("END of the main function");

    loop {
        uart.read_uart_char();
    }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}

fn add_variable(a: i32, b: i32) -> i32 {
    a + b
}
