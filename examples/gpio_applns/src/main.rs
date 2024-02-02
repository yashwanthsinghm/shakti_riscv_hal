#![no_std]
#![no_main]
#![feature(asm)]

use riscv::{asm::delay, delay};
// use cortex_m_rt::entry;
use riscv_rt::entry;
use shakti_riscv_hal::gpio::{GPIOInner, GPIO_OFFSET} ;

mod gpio_constants;
use gpio_constants::*;

// Define GPIO pins or LEDs (replace these with actual GPIO pin identifiers)
const LED0_B: u32 = GPIO18;
const LED0_R: u32 = GPIO16;
const LED0_G: u32 = GPIO17;
const LED1_B: u32 = GPIO21;
const LED1_R: u32 = GPIO19;
const LED1_G: u32 = GPIO20;
const LED2: u32 = GPIO22;
const LED3: u32 = GPIO23;

// Define GPIO direction control register and data register
const GPIO_DIRECTION_CNTRL_REG: u32 = 0x00; // Assuming the register address is 0x00
const GPIO_DATA_REG: u32 = 0x08; // Assuming the register address is 0x08

// Define delay constants
const DELAY1: u32 = 1000;
const DELAY2: u32 = 1000;

// pub struct GPIOInner {
//     pub registers: Registers, // Change `registers` visibility to public
// }

// impl GPIOInner {

//     pub fn turn_on_ledx(&mut self, led: u32) {
//     // Implementation specific to the GPIO crate
//     self.registers.DATA_REG.write(1 << led);
// }
// }
struct GPIO_ACCESS
{
  gpio : GPIOInner
}
impl GPIO_ACCESS
{
    pub fn new() -> GPIO_ACCESS
    {
       GPIO_ACCESS{
        gpio : unsafe { GPIOInner::new(GPIO_OFFSET) }
       }
    }
    pub fn turn_on_ledx(&mut self, led: u32) {
        // Implementation specific to the GPIO crate
        //gpio.registers.DATA_REG.write(1 << led);
        self.gpio.set_data_register(led);
    }
    
    // Function to turn off an LED
    pub fn turn_off_ledx(&mut self) {
        // Implementation specific to the GPIO crate
        self.gpio.set_data_register(0);
    }

    pub fn set_direction(&mut self,value:u32) {
        // Implementation specific to the GPIO crate
        self.gpio.set_direction_control(value);
    }
    

}


fn main() {
    
    let gpio_mmio_start_addr = 0x1000;
    
        
    let mut gpio_access = GPIO_ACCESS::new(); 
    gpio_access.set_direction(0x00);
    gpio_access.turn_off_ledx();  

        // Set the direction control register
    //gpio.set_direction_control(0x0);
    
        // Write to GPIO_DATA_REG to initialize GPIO pins
    //gpio.set_data_register(0x0); // Assuming initialization value is 0x0
    

    loop {
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED0_G);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED0_B);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED1_R);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED1_G);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED1_B);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED2);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_on_ledx(LED3);
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);

            gpio_access.turn_off_ledx();
            delay_loop(DELAY1, DELAY2);
    }
}


// Function to turn on an LED

// Function to perform delay loop

pub fn delay_loop(mut cntr1: u32, mut cntr2: u32) {
        while cntr1 > 0 {
            let mut tmp_cntr = cntr2;

            while tmp_cntr > 0 {
                tmp_cntr -= 1;
            }

            cntr1 -= 1;
        }
    }


 #[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}