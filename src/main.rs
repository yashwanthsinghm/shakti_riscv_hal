#![no_std]
#![no_main]
#![feature(asm)]

use riscv::{asm::delay, delay};
// use cortex_m_rt::entry;
use riscv_rt::entry;
use spi::{SPIInner, SPI_OFFSET};
use uart::{UartInner, UART_OFFSET};
pub mod common;
pub mod spi;
pub mod uart;

#[entry]
fn main() -> ! {
    let mut uart = unsafe { UartInner::new(UART_OFFSET) };
    let mut spi = unsafe { SPIInner::new(SPI_OFFSET) };
    spi.init();
    let dr5 = spi.flash_device_id();
    unsafe {
        delay(1000000);
    }

    uart.write_uart_string("Akshaya not working \n ");
    let y = spi.flash_read(0x00B0_0000);
    spi.flash_write_enable();
    spi.flash_erase(0x00b0_0000);
    spi.flash_status_register_read();

    let z = spi.flash_write(0x00B0_0000, 0x12345678);

    let v = spi.flash_read(0x00B0_0000);

    spi.flash_write_enable();
    spi.flash_erase(0x00b0_0000);
    spi.flash_status_register_read();
    let y = spi.flash_read(0x00B0_0000);

    let x = add_variable(5, 10);

    uart.write_uart_string("bhavya not working");

    loop {}
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
