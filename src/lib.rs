#![no_std]


/**
 * 
 */
//# Shakti_riscv_hal
//****This crate is hardware abstraction layer for Shakti's C-class PARASU,PINAKA and E-class VAJRA processor which is riscv architecture.****


///SHAKTI is an open-source initiative by the Reconfigurable Intelligent Systems Engineering (RISE)
///group at IIT-Madras . The aim of the SHAKTI initiative includes building open source
///production grade processors, complete System on Chips (SoCs), development boards
///and SHAKTI-based software platform. The SHAKTI project is building a family of 6
///processors, based on the RISC-V ISA. There is a road-map to develop reference
///System on Chips (SoC) for each class of processors, which will serve as an exemplar for
///that family . The team has channelized years of research on processor architecture to
///build these SoCs which has competitive commercial offerings in the market with respect
///to occupied power, area and performance. The current SoC (as of December 2019)
///developments are for the Controller (C- Class)  and Embedded (E- Class) classes .

pub mod gpio;
pub mod uart;
pub mod common;
pub mod spi;
pub mod i2c;
pub mod pwm;
pub mod ext_flash;
