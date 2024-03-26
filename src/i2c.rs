#![allow(dead_code)]
#![allow(non_snake_case)]
#![deny(unused_must_use)]
//#![deny(missing_docs)]

//! I2C two-wire interface

/// I2C is a serial protocol for a two-wire interface to connect low-speed devices like
/// microcontrollers, EEPROMs, A/D and D/A converters, I/O interfaces and
/// other similar peripherals in embedded systems.
/// Multiple slave devices can be connected to a single master with I2C.
/// I2C only uses two wires to transmit data between devices
use crate::common::MMIODerefWrapper;
use riscv::{
    asm::{delay, nop},
    register,
};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// Base address of the I2C peripheral register block
pub const I2C_OFFSET: usize = 0x0004_0000;
///Max I2C count
pub const MAX_I2C_COUNT: usize = 2;
///Error Remote IO
pub const EREMOTEIO: isize = -81;

/// I2C_INI Register bit masks
/// Initialization bit
pub const I2C_INI: u8 = 1 << 7;
/// Status bit
pub const I2C_STS: u8 = 1 << 5;
/// Bus Error bit
pub const I2C_BER: u8 = 1 << 4;
/// Address/Data 0 (Read/Write bit)
pub const I2C_AD0_LRB: u8 = 1 << 3;
/// Acknowledge Address/Stop bit
pub const I2C_AAS: u8 = 1 << 2;
/// Last Bit bit
pub const I2C_LAB: u8 = 1 << 1;
/// Bus Busy bit
pub const I2C_BB: u8 = 1 << 0;

// I2C_PIN Register bit masks
/// Pin Select bit
pub const I2C_PIN: u8 = 1 << 7;
/// Enable Slave 0 bit
pub const I2C_ES0: u8 = 1 << 6;
/// Enable Interrupt (Read/Write bit)
pub const I2C_ENI_LRB: u8 = 1 << 3;
/// Start condition bit
pub const I2C_STA: u8 = 1 << 2;
/// Stop condition bit
pub const I2C_STO: u8 = 1 << 1;
/// LRB bit
pub const I2C_LRB: u8 = 0x08;
/// Acknowledge bit
pub const I2C_ACK: u8 = 1 << 0;
/// ESO bit
pub const I2C_ESO: u8 = 0x40;
/// IDLE BIT
pub const I2C_IDLE: u8 = I2C_PIN | I2C_ESO | I2C_ACK;
/// TIMEOUT BIT
pub const DEF_TIMEOUT: u8 = 60;
/// I2C STOP BIT
pub const I2C_STOP: u8 = I2C_PIN | I2C_ESO | I2C_STO | I2C_ACK;
/// I2C ENI BIT
pub const I2C_ENI: u8 = 0x08;
/// I2C READ BIT
pub const I2C_READ: u8 = 1;
/// I2C WRITE BIT
pub const I2C_WRITE: u8 = 0;
/// I2C REPSTART ENI BIT
pub const I2C_REPSTART_ENI: u8 = I2C_ESO | I2C_STA | I2C_ACK | I2C_ENI;
/// I2C REPSTART
pub const I2C_REPSTART: u8 = I2C_ESO | I2C_STA | I2C_ACK;
/// I2C START BIT
pub const I2C_START: u8 = I2C_PIN | I2C_ESO | I2C_STA | I2C_ACK;
/// I2C START ENI
pub const I2C_START_ENI: u8 = I2C_PIN | I2C_ESO | I2C_STA | I2C_ACK | I2C_ENI;
/// I2C STOP ENI
pub const I2C_STOP_ENI: u8 = I2C_PIN | I2C_ESO | I2C_STO | I2C_ACK | I2C_ENI;

register_structs! {
    #[allow(non_snake_case)]
    ///Register Block
    pub RegistersBlock {
        /// Prescaler register (PRESCALE)
        (0x00 => PRESCALE: ReadWrite<u16>),

        /// Reserved register (0x02)
        (0x02 => _reserved0),

        /// Control register (CONTROL)
        (0x08 => CONTROL: ReadWrite<u8>),

        /// Reserved register (0x09)
        (0x09 => _reserved1),

        /// Data register (DATA)
        (0x10 => DATA: ReadWrite<u8>),

        /// Reserved register (0x11)
        (0x11 => _reserved2),

        /// Status register (STATUS)
        (0x18 => STATUS: ReadWrite<u8>),

        /// Reserved register (0x19)
        (0x19 => _reserved3),

        /// Serial Clock register (SCL)
        (0x38 => SCL: ReadWrite<u8>),

        /// Reserved register (0x39)
        (0x39 => _reserved4),

        /// End marker
        (0x3C => @END),
    }
}

register_bitfields! {
    u32,

    /// I2C Prescale Register divides the System clock by (Prescale value + 1).
    /// This clock is used as clock input for I2C Serial Clock.
    /// I2C Prescaler clock = System Clock / (Prescaler Value + 1)

    PRESCALE [
        PRESCALE_VALUE OFFSET(0) NUMBITS(8) []
    ],

    ///I2C SCL Register divides the I2C Prescaler clock by (SCL value + 1). This clock is used as
    ///I2C SCL clock = I2C Prescaler Clock / (SCL COUNT + 1).

    SCL[
        SCL_COUNT OFFSET(0) NUMBITS(8) []
    ],

    ///High when I2C communication in progress. Becomes low once I2C communication is complete.

    STATUS [

        I2C_INI OFFSET(7) NUMBITS(1) [],

        /// When in slave receiver mode, this flag is asserted when an
        /// externally generated STOP condition is detected (used only in
        /// slave receiver mode).

        I2C_STS OFFSET(5) NUMBITS(1) [],

        ///Bus error; a misplaced START or STOP condition has been detected

        I2C_BER OFFSET(4) NUMBITS(1) [],

        ///AD0(Address 0) - General Call bit used for Broadcast

        ///LRB - Last Received Bit through I2C Bus
        ///This status bit serves a dual function, and is valid only while
        ///PIN = 0:

        ///1. LRB holds the value of the last received bit over the
        ///I2C-bus while AAS = 0 (not addressed as slave).
        ///Normally this will be the value of the slave acknowledgement; thus
        ///checking for slave acknowledgement is done via testing of the
        ///LRB.

        ///2. AD0; when AAS = 1 (‘Addressed As Slave’ condition), the
        ///I2C-bus controller has been addressed as a slave.
        ///Under this condition, this bit becomes the ‘AD0’ bit and will be set to
        ///logic 1 if the slave address received was the ‘general call’
        ///(00H) address, or logic 0 if it was the I2C-bus controller’s own
        ///slave address.

        I2C_AD0_LRB OFFSET(3) NUMBITS(1) [],

        ///Addressed As Slave bit.
        ///Valid only when PIN = 0. When acting as slave receiver, this flag is set when an incoming
        ///address over the I2C-bus matches the value in own address register

        I2C_AAS OFFSET(2) NUMBITS(1) [],

        ///Lost Arbitration Bit.
        ///This bit is set when, in multi-master
        ///operation, arbitration is lost to another master on the I2C-bus

        I2C_LAB OFFSET(1) NUMBITS(1) [],

        ///Bus Busy bit.
        ///This is a read-only flag indicating when the I2C-bus is in use.
        ///A zero indicates that the bus is busy, and access is not possible

        I2C_BB OFFSET(0) NUMBITS(1) []

    ],

    CONTROL [

    ///Pending Interrupt Not, Used as a software reset.

    I2C_PIN OFFSET(7) NUMBITS(1) [],

    /// Enable Serial Output
    ///0 - Registers can be initialized.
    ///1 - I2C Serial Transmission

    I2C_ES0 OFFSET(6) NUMBITS(1) [
        REGISTER_INITIALIZED = 0,
        I2C_SERIAL_TRANSMISSION = 1,
    ],

    ///Enables the external interrupt output, which is generated when the PIN is active (Low)

    I2C_ENI OFFSET(3) NUMBITS(1) [],

    ///Transmits Start condition + Slave address..

    I2C_STA OFFSET(2) NUMBITS(1) [],

    ///Transmits the stop condition.

    I2C_STO OFFSET(1) NUMBITS(1) [],

    ///Acknowledgement bit:
    ///1: I2C automatically sends an acknowledgement after a read/write transaction.
    ///0: I2C Master sends Negative Acknowledge to stop the I2C transfer

    I2C_ACK OFFSET(0) NUMBITS(1) [
        NAK = 0,
        ACK = 1,
    ]

],
}

#[derive(Debug)]
enum ErrorTypes {
    ETIMEOUT = -60,
    DEF_TIMEOUT = 60,
    ETIMEDOUT = -80,
    ENXIO = -82,
    EREMOTEIO = -81,
    I2C_SUCCESS = 0,
    EAXI_ERROR = -1,
    EI2C_BUS_ERROR = -2,
    EI2C_PIN_ERROR = -3,
    EI2C_LRB_ERROR = -4,
}

type Registers = MMIODerefWrapper<RegistersBlock>;

/// Internal representation of an I2C peripheral
pub struct I2CInner {
    /// Memory-mapped registers for interacting with the I2C hardware
    registers: Registers,
}

impl I2CInner {
    /// Creates a new instance of I2CInner, unsafe due to direct hardware access
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        unsafe {
            Self {
                // Initializes the registers with the provided memory-mapped address
                registers: Registers::new(mmio_start_addr),
            }
        }
    }

    ///i2c init
    pub fn i2c_init(&mut self) {}

    ///Configure I2C
    ///This routine configures the serial clock frequency count and prescaler count.
    /// There are 4 registers which are configurable. This function writes into the registr based on the passed address. he serial clock count
    /// and prescalar count decides the frequency (sck) that needs to be used for
    /// the I2C serial communication. Then resets status register.

    pub fn config_i2c(&mut self, prescale_div: u16, scl_div: u8) -> Result<(), ErrorTypes> {
        let mut temp: u16 = 0;

        if prescale_div != self.registers.PRESCALE.get() {
            self.registers.PRESCALE.set(prescale_div);

            temp = self.registers.PRESCALE.get();

            if (temp | 0x00) != prescale_div {
                //log_error("\t Failed to write Prescale division Written Value: 0x{:x}; read Value: 0x{:x}\n", prescale_div, temp);
                return Err(ErrorTypes::ENXIO); // Use libc for error codes
            } else {
                return Ok(());
                //log_debug("\tPrescaler successfully initalized\n");
            }
        }

        if scl_div != self.registers.SCL.get() {
            self.registers.SCL.set(scl_div);

            let temp = self.registers.SCL.get();
            if (temp | 0x00) != scl_div {
                //println!("\tClock initialization failed Write Value: 0x{:x}; read Value: 0x{:x}",scl_div, temp);
                return Err(ErrorTypes::ENXIO);
            } else {
                //println!("\tClock successfully initialized");
                return Ok(());
            }
        }

        // Clearing the status register.
        //println!("\tClearing the status register.");
        self.registers.CONTROL.set(I2C_PIN);

        // S1=0x80 S0 selected, serial interface off
        // Reading set control Register Value to ensure sanctity
        //println!("\tReading Status Register");
        let temp = self.registers.CONTROL.get();
        // Check whether the status register is cleared or not.
        if (temp & 0x7f) != 0 {
            //println!("\tDevice Not Recognized");
            return Err(ErrorTypes::ENXIO);
        }
        //std::println!("\tWaiting for a specified time");
        // 1 Second software wait -- Should be 900000 but setting to 900 now since simulation is already slow
        self.waitfor(900);
        //println!("\tDone Waiting");
        //println!("\nControl: 0x{:x}; Status: 0x{:x}",self.register.CONTROL.get(),self.register.STATUS.get());
        // Enable Serial Interface
        self.registers.CONTROL.set(I2C_IDLE);
        // 1 Second software wait -- Should be 900000 but setting to 900 now since simulation is already slow
        self.waitfor(900);

        let temp = self.registers.STATUS.get();
        // Check to see if I2C is really in Idle and see if we can access the status register -- If not something wrong in initialization.
        // This also verifies if Control is properly written since zero bit will be initialized to zero
        if temp != (I2C_PIN | I2C_BB) {
            //println!("\n\tInitialization failed; Status Reg: 0x{:x}", temp);
            return Err(ErrorTypes::ENXIO);
        }

        //println!("\tI2C Initialization success");
        Ok(())
    }

    ///Wait for the i2c bus to be free.
    ///Once a I2C Transaction is started, the bus needs to be freed for other devices
    ///to use the bus. This function checks the busy bit in status register to become free
    ///for a particular time. If it becomes free, returns 0, else negative value.

    fn wait_till_i2c_bus_free(&mut self) -> Result<bool, bool> {
        //println!("\tCheck for I2C Bus Busy to be free.");
        let mut timeout = DEF_TIMEOUT;
        let mut status;

        status = self.registers.STATUS.get();

        while (status & I2C_BB) == 0 && timeout > 0 {
            self.waitfor(20000); /* wait for 100 us */
            status = self.registers.STATUS.get();
            timeout -= 1;
        }

        if timeout == 0 {
            //println!("\t Bus busy wait - timed out. Resetting");
            return Err(true);
        }

        Ok(false)
    }

    ///Waits in the loop till the i2c tx/rx operation completes
    ///The PIN bit in the status register becomes high when tx/rx operation
    /// starts and becomes low once done. This function checks whether tx/rx operaiton is complete or not.

    fn wait_till_txrx_operation_completes(&mut self, mut status: u8) -> Result<usize, ErrorTypes> {
        let mut timeout = DEF_TIMEOUT;
        status = self.registers.STATUS.get();

        while (status & I2C_PIN) != 0 && timeout > 0 {
            self.waitfor(10000); /* wait for 100 us */
            status = self.registers.STATUS.get();
            timeout -= 1;
        }

        if timeout == 0 {
            //println!("\tWait for pin timed out");
            return Err(ErrorTypes::ETIMEDOUT);
        }
        self.waitfor(10000); /* wait for 100 us */
        //println!("\n I2C tx_rx operation is completed");
        Ok(0)
    }

    ///writes "n" number of bits over i2c bus.
    ///Called when the user wants to write n number of bits over i2c bus.

    fn sendbytes(
        &mut self,
        buf: &[u8],
        count: usize,
        last: bool,
        eni: bool,
    ) -> Result<usize, ErrorTypes> {
        let wrcount: usize = 0x00;
        let status: u8 = 0x00;
        let timeout: usize;

        //println!("\tStarting Write Transaction -- Did you create tri1 nets for SDA and SCL in verilog?");

        for wrcount in 0..count {
            self.registers.DATA.set(buf[wrcount]);

            let mut timeout = self.wait_till_txrx_operation_completes(status).unwrap();

            if timeout != 0 {
                //println!("\tTimeout happened - Write did not go through the BFM -- Diagnose");

                self.registers.CONTROL.set(I2C_STOP);

                return Err(ErrorTypes::EREMOTEIO);
            }

            if status & I2C_LRB != 0 {
                self.registers.CONTROL.set(I2C_STOP);

                //println!("\tSome status check failing");

                return Err(ErrorTypes::EREMOTEIO);
            }
        }

        if last {
            //println!("\tLast byte sent: Issue a stop");

            self.registers.CONTROL.set(I2C_STOP);
        } else {
            //println!("\tSending Rep Start and doing some other R/W transaction");

            if !eni {
                self.registers.CONTROL.set(I2C_REPSTART);
            } else {
                self.registers.CONTROL.set(I2C_REPSTART_ENI);
            }
        }

        Ok(wrcount)
    }

    ///Reads "n" number of bytes from I2C Bus
    ///Reads n number of bytes over I2C Bus and store the same in "buf" pointer.

    fn readbytes(&mut self, buf: &mut [u8], count: usize, last: bool) -> Result<usize, ErrorTypes> {
        let mut status = 0;
        let mut wfp;

        /* Increment number of bytes to read by one -- read dummy byte */
        for i in 0..=count {
            wfp = self.wait_till_txrx_operation_completes(status).unwrap();

            if wfp != 0 {
                self.registers.CONTROL.set(I2C_STOP);
                return Err(ErrorTypes::EAXI_ERROR);
            }

            if (status & I2C_LRB) != 0 && i != count {
                self.registers.CONTROL.set(I2C_STOP);
                //println!("\tNo ack");
                return Err(ErrorTypes::EAXI_ERROR);
            }

            if i > 0 {
                buf[i - 1] = self.registers.DATA.get();
                //println!("\n Read Value: {}", buf[i - 1]);
            } else {
                self.registers.DATA.set(!self.registers.DATA.get()); /* dummy read */
            }

            if i == count - 1 {
                self.registers.CONTROL.set(I2C_ESO);
            } else if i == count {
                if last {
                    self.registers.CONTROL.set(I2C_STOP);
                } else {
                    self.registers.CONTROL.set(I2C_REPSTART_ENI);
                }
            }
        }

        Ok(count - 1) // excluding the dummy read
    }

    ///Performs the intilization of i2c slave.
    ///Writes slave addresss into the i2b to start write or read operation.
    
    fn i2c_send_slave_address(
        &mut self,
        slave_address: u8,
        rd_wr_cntrl: u8,
        delay: u64,
    ) -> Result<usize, ErrorTypes> {
        let mut timeout;
        let mut temp = 0;
        let mut status = 0;

        let _ = delay; // Unused variable

        if rd_wr_cntrl == 0 {
            slave_address | I2C_WRITE
        } else {
            slave_address | I2C_READ
        };

        self.registers.DATA.set(slave_address);
        //log_debug("\tSlave Address 0x{:x} written into data register\n", slave_address);

        temp = self.registers.DATA.get(); // Reads the slave address from I2C controller

        if slave_address != temp as u8 {
            //log_error("\tSlave address is not matching; Written Add. Value: 0x{:x}; Read Add. Value: 0x{:x}\n", slave_address, temp);
            // log_error("\n There is some issue in AXI interface. Please check.");
            return Err(ErrorTypes::EAXI_ERROR);
        }

        while self.wait_till_i2c_bus_free().unwrap() {
            //log_error("\tError in Waiting for BB\n");
            return Err(ErrorTypes::EI2C_BUS_ERROR);
        }

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]

        #[cfg(feature = "USE_SA_WRITE_I2C_INTERRUPT")]
        self.registers.CONTROL.set(I2C_START);
        self.waitfor(900);

        timeout = self.wait_till_txrx_operation_completes(status).unwrap();

        if timeout != 0 {
            //println!("\tTimeout happened - Write did not go through the BFM -- Diagnose");
            self.registers.CONTROL.set(I2C_STOP);
            return Err(ErrorTypes::EI2C_PIN_ERROR);
        }
        if status & I2C_LRB != 0 {
            self.registers.CONTROL.set(I2C_STOP);
            //println!("\tSome status check failing");
            return Err(ErrorTypes::EI2C_LRB_ERROR);
        }

        #[cfg(not(feature = "USE_SA_WRITE_I2C_INTERRUPT"))]
        let mut i2c_complete_flag = 0;
        self.registers.CONTROL.set(I2C_START_ENI);
        while i2c_complete_flag == 0 {}
        //println!("\n Slave Address Write Operation is complete.");
        i2c_complete_flag = 0;

        Ok(0)
    }

    // fn receivebytes(
    //     instance: &mut I2cStruct,
    //     buf: &mut [u8],
    //     count: usize,
    //     last: bool,
    //     eni: bool,
    // ) -> Result<usize, I2cError> {
    //     println!("\tReceivebytes entered");
    //     if count == 0 {
    //         return Ok(0);
    //     }
    //     if eni {
    //         if instance.control == 0 {
    //             return Err(I2cError::Ei2cpinerror);
    //         }
    //     }
    //     let mut received = 0;
    //     while received < count {
    //         if instance.control == 0 {
    //             return Err(I2cError::Ei2cpinerror);
    //         }
    //         if last && received == count - 1 {
    //             instance.control = 0x58;
    //         } else {
    //             instance.control = 0x50;
    //         }
    //         let mut status = 0;
    //         if let Err(err) = wait_till_txrx_operation_completes(instance, &mut status) {
    //             return Err(err);
    //         }
    //         if status == 0x48 {
    //             //println!("\tNACK Received while trying to write address. Terminating read transaction");
    //             return Err(ErrorTypes::EI2C_BUS_ERROR);
    //         }
    //         if status == 0x58 {
    //             //println!("\tRead transaction complete");
    //             return Ok(received);
    //         }
    //         buf[received] = instance.data;
    //         received += 1;
    //     }
    //     Ok(received)
    // }

    ///It does the reading or writing from the address specified .
    ///Writes one byte to the slave I2C DEVICE.
    
    fn i2c_write_data(&mut self, write_data: u8, delay: u8) -> Result<usize, ErrorTypes> {
        let mut timeout: u8;
        let mut status = 0;
        let _ = delay; // Unused variable

        self.registers.DATA.set(write_data);

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]

        #[cfg(feature = "USE_WRITE_I2C_INTERRUPT")]
        {
            timeout = self.wait_till_txrx_operation_completes(status);
            if timeout != 0 {
                //println!("\tTimeout happened - Write did not go through the BFM -- Diagnose");
                self.registers.CONTROL.set(I2C_STOP);
                return Err(ErrorTypes::EREMOTEIO);
            }
            if status & I2C_LRB != 0 {
                self.registers.CONTROL.set(I2C_STOP);
                //println!("\tSome status check failing");
                return Err(ErrorTypes::EI2C_LRB_ERROR);
            }
        }
        #[cfg(not(feature = "USE_WRITE_I2C_INTERRUPT"))]
        {
            let mut i2c_complete_flag: u8 = 0;
            self.registers.CONTROL.set(I2C_STOP_ENI);
            while i2c_complete_flag == 0 {}
            //println!("\n Write Operation is complete.");
            i2c_complete_flag = 0;
        }

        Ok(0)
    }

    ///It does the reading or writing from the address specified .
    ///Reads a byte of data over I2C bus from the passed I2C location.
    fn i2c_read_data(&mut self, read_data: &mut u8, delay: u8) -> Result<usize, ErrorTypes> {
        let mut status = 0;
        let _ = delay; // Unused variable

        *read_data = self.registers.DATA.get();

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]
        #[cfg(USE_READ_I2C_INTERRUPT)]
        {
            while self.wait_till_txrx_operation_completes(status) {
                //println!("\twaiting for pin");
                self.waitfor(delay);
            }
        }
        #[cfg(not(USE_READ_I2C_INTERRUPT))]
        {
            let i2c_complete_flag = 0;
            self.registers.CONTROL.set(I2C_REPSTART_ENI);

            while i2c_complete_flag == 0 {}
            *read_data = self.registers.DATA.get();
            //println!("\n I2C Read Data = {}", *read_data);
        }

        Ok(0)
    }

    ///It does the reading or writing from the address specified .
    /// Reads a byte of data over I2C bus from the passed I2C location. Then sends request to send in the next byte.
    
    fn i2c_read_data_nack(&mut self, read_data: &mut u8, delay: u8) -> Result<usize, ErrorTypes> {
        let _ = delay; // Unused variable

        *read_data = self.registers.DATA.get();

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]
        #[cfg(USE_WRITE_I2C_INTERRUPT)]
        {
            let i2c_complete_flag = 0;
            self.registers.CONTROL.set(I2C_REPSTART_ENI);
            while i2c_complete_flag == 0 {}
            *read_data = self.registers.DATA.get();
            //println!("\n I2C Read Data = {}", *read_data);
        }

        //println!("\n I2C Read Data = {}", *read_data);
        Ok(0)
    }

    ///Sends the slave address over I2C Bus.
    ///Interrupt based routine to send slave address to the I2C slave device
 
    fn i2c_send_interrupt_slave_address(
        &mut self,
        slave_address: u8,
        rd_wr_cntrl: u8,
        delay: u64,
    ) -> Result<usize, ErrorTypes> {
        let mut timeout: u8;
        let mut temp = 0;
        let mut status = 0;
        let _ = delay; // Unused variable

        if rd_wr_cntrl == 0 {
            slave_address | I2C_WRITE
        } else {
            slave_address | I2C_READ
        };

        //log_debug("\n\tSetting Slave Address : 0x{:x}\n", slave_address);
        self.registers.DATA.set(slave_address);
        //log_debug("\tSlave Address is written into data register\n");

        temp = self.registers.DATA.get();

        if slave_address != temp as u8 {
            //log_error("\tSlave address is not matching; Written Add. Value: 0x{:x}; Read Add. Value: 0x{:x}\n", slave_address, temp);
            //log_error("\n There is some issue in AXI interface. Please check.");
            return Err(ErrorTypes::EAXI_ERROR);
        }

        while self.wait_till_i2c_bus_free().unwrap() {
            //log_error("\tError in Waiting for BB\n");
            return Err(ErrorTypes::EI2C_BUS_ERROR);
        }

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]
        #[cfg(features = "USE_SA_WRITE_I2C_INTERRUPT")]
        {
            self.registers.CONTROL.set(I2C_START);
            timeout = self.wait_till_txrx_operation_completes(status);
            if timeout != 0 {
                //println!("\tTimeout happened - Write did not go through the BFM -- Diagnose");
                self.registers.CONTROL.set(I2C_STOP);
                return Err(ErrorTypes::EI2C_PIN_ERROR);
            }
            if status & I2C_LRB != 0 {
                self.registers.CONTROL.set(I2C_STOP);
                //println!("\tSome status check failing");
                return Err(ErrorTypes::EI2C_LRB_ERROR);
            }
        }
        #[cfg(not(features = "USE_SA_WRITE_I2C_INTERRUPT"))]
        {
            let mut i2c_complete_flag = 0;
            self.registers.CONTROL.set(I2C_REPSTART_ENI);
            while i2c_complete_flag == 0 {}
            //println!("\n Slave Address Write Operation is complete.");
            i2c_complete_flag = 0;
        }

        //log_info("\n Slave address is written successfully");
        Ok(0)
    }

    ///Interrupt based I2C read 
    ///Interrupt based i2c read function to read from the I2C slave.
 
    fn i2c_read_interrupt_data(
        &mut self,
        read_data: &mut u8,
        delay: u8,
        last: u8,
    ) -> Result<usize, ErrorTypes> {
        let mut status = 0;
        //let _ = delay; // Unused variable

        *read_data = self.registers.DATA.get();

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]

        #[cfg(features = "USE_READ_I2C_INTERRUPT")]
        {
            let mut i2c_complete_flag = 0;
            if last != 0 {
                self.registers.CONTROL.set(I2C_STOP_ENI);
                while i2c_complete_flag == 0 {}
            } else {
                // Needs to be tested
                // instance.control = I2C_REPSTART_ENI;
                // println!("\n Call I2C rep. start eni");
                // while !i2c_complete_flag {}
            }
            //println!("\n I2C Read Data = {}", *read_data);
        }
        #[cfg(not(features = "USE_READ_I2C_INTERRUPT"))]
        {}
        while match self.wait_till_txrx_operation_completes(status).unwrap() {
            0 => true,
            _ => false,
        } {
            //println!("\twaiting for pin");
            self.waitfor(delay as usize);
        }
        if last == 0 {
            //println!("\n Rep Start");
        } else {
            //println!("\nCall I2C Stop");
            self.registers.CONTROL.set(I2C_STOP);
        }

        Ok(0)
    }

    ///Interrupt based I2C write function.
 ///Writes a byte of data into slave I2C bus using interrupt.
    fn i2c_write_interrupt_data(
        &mut self,
        write_data: u8,
        delay: u8,
        last: u8,
    ) -> Result<usize, ErrorTypes> {
        let mut timeout: u8;
        let mut status: u8 = 0;
        let _ = delay; // Unused variable

        self.registers.DATA.set(write_data);

        //#![allow(unused_assignments)]
        //#![allow(unreachable_code)]

        #[cfg(USE_WRITE_I2C_INTERRUPT)]
        {
            timeout = self.wait_till_txrx_operation_completes(status);
            if timeout != 0 {
                //println!("\tTimeout happened - Write did not go through the BFM -- Diagnose");
                self.registers.CONTROL.set(I2C_STOP);
                return Err(ErrorTypes::EREMOTEIO);
            }
            if status & I2C_LRB != 0 {
                self.registers.CONTROL.set(I2C_STOP);
                //println!("\tSome status check failing");
                return Err(ErrorTypes::EI2C_LRB_ERROR);
            }
            if last != 0 {
                self.registers.CONTROL.set(I2C_STOP);
                //println!("\tI2C Write Success and completes");
            }
        }

        #[cfg(not(USE_WRITE_I2C_INTERRUPT))]
        let mut i2c_complete_flag = 0;
        if last != 0 {
            self.registers.CONTROL.set(I2C_STOP_ENI);
            //println!("\n Calling stop eni write");
            while i2c_complete_flag == 0 {}
        } else {
            // instance.control = I2C_REPSTART_ENI;
            // println!("\n Calling repstart eni write");
            // while !i2c_complete_flag {}
        }
        //println!("\n Write Operation is complete.");
        i2c_complete_flag = 0;

        Ok(0)
    }

    ///Stall the process for given time
    fn waitfor(&mut self, duration: usize) {
        // Placeholder for waitfor function

        let mut time = 0;
        while time < duration {
            time += 1;
        }
    }
}
