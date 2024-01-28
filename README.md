# Shakti_riscv_hal
****This crate is hardware abstraction layer for Shakti's C-class PARASU,PINAKA and E-class VAJRA processor which is riscv architecture.****


SHAKTI is an open-source initiative by the Reconfigurable Intelligent Systems Engineering (RISE)
group at IIT-Madras . The aim of the SHAKTI initiative includes building open source
production grade processors, complete System on Chips (SoCs), development boards
and SHAKTI-based software platform. The SHAKTI project is building a family of 6
processors, based on the RISC-V ISA. There is a road-map to develop reference
System on Chips (SoC) for each class of processors, which will serve as an exemplar for
that family . The team has channelized years of research on processor architecture to
build these SoCs which has competitive commercial offerings in the market with respect
to occupied power, area and performance. The current SoC (as of December 2019)
developments are for the Controller (C- Class)  and Embedded (E- Class) classes .

**Processors**
SHAKTI is a RISC-V based processor developed at RISE lab, IIT Madras. SHAKTI
has envisioned a family of processors as part of its road-map, catering to different segments
of the market. They have been broadly categorized into "Base Processors", "Multi-Core
Processors" and "Experimental Processors" . The E and C-classes are the first set
of indigenous processors aimed at Internet of Things (IoT), Embedded and Desktop
markets. The processor design is free of any royalty and is open-sourced under BSD3 license. A brief overview of the E and C-classes of processors is described below.

**E-class**
The E-Class  is a 32 bit micro processor capable of supporting all extensions of RISC-V
ISA as listed in Table 1. The E-class is an In-order 3-stage pipeline having an operational
frequency of less than 200MHz on silicon. It is positioned against ARM’s M-class (CorTexM series) cores . The major anticipated use of the E-class of processors is in lowpower compute environments, automotive and IoT applications such as smart-cards,
motor-controls and home automation. The E-class is also capable of running Real Time
Operating Systems (RTOS) like Zephyr OS  and FreeRTOS.

**PINAKA** (E32-A35) is a SoC built around E-class. Pinaka is a 32-bit E-class micro
controller with 4KB ROM and 128KB BRAM, has 32 General Purpose Input Output (GPIO)
pins (out of which upper 8 GPIO pins are dedicated to onboard LEDs and switches), a
Platform Level Interrupt Controller (PLIC), a Timer (CLINT), 2 Serial Peripheral (SPI), 3
Universal Asynchronous Receiver Transmitter (UART), 2 Inter Integrated Circuit (I2C),
6 Pulse Width Modulator (PWM), an in-built Xilinx Analog Digital Converter (X-ADC),
Soft Float library support, Physical Memory Protection (PMP) enabled, onboard FTDI
based debugger and Pin Mux support (Arduino compatible pin assignments). Table 2
describes in detail.

**PARASHU** (E32-A100) is a SoC built around E-class. Parashu is a 32-bit E-class
micro controller with 4 KB of ROM and 256 MB of DDR. The rest of the configuration in
this SoC, is the same as PINAKA.

**C-class**
The C-class  is an in-order 6-stage 64-bit micro controller supporting the entire RISCV ISA. It targets the mid-range compute systems supporting 200-800MHz. C-class targets
compute applications in the 0.5-1.5 Ghz range. The C-class is customizable for lowpower and high-performance variants. It is positioned against ARM’s Cortex A35/A55.
Linux, SEL4 and FreeRTOS are some of the Operating Systems ported and verified with
C-class.

**VAJRA**(C64-A100) is an SoC built around C-class. This SoC is a single-chip 64-bit
C-class micro controller with 4KB of ROM and 256MB DDR3 RAM. VAJRA is aimed at mid-range application workloads
like Industrial controllers and Desktop market.
