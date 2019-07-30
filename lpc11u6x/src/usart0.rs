#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 4usize],
    _reserved_1_dlm: [u8; 4usize],
    _reserved_2_fcr: [u8; 4usize],
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register."]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register."]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad Register. Eight-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    #[doc = "0x24 - IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
    pub icr: ICR,
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    #[doc = "0x2c - Oversampling Register. Controls the degree of oversampling during each bit time."]
    pub osr: OSR,
    #[doc = "0x30 - Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
    pub ter: TER,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - Half duplex enable register."]
    pub hden: HDEN,
    _reserved14: [u8; 4usize],
    #[doc = "0x48 - Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
    pub scictrl: SCICTRL,
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
    #[doc = "0x58 - Synchronous mode control register."]
    pub syncctrl: SYNCCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dll(&self) -> &DLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DLL) }
    }
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dll_mut(&self) -> &mut DLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DLL) }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub fn thr(&self) -> &THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const THR) }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub fn thr_mut(&self) -> &mut THR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut THR) }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub fn rbr(&self) -> &RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR) }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub fn rbr_mut(&self) -> &mut RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut RBR) }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const IER) }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut IER) }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dlm(&self) -> &DLM {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM) }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dlm_mut(&self) -> &mut DLM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut DLM) }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub fn fcr(&self) -> &FCR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const FCR) }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub fn fcr_mut(&self) -> &mut FCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut FCR) }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub fn iir(&self) -> &IIR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR) }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub fn iir_mut(&self) -> &mut IIR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IIR) }
    }
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
pub struct RBR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
pub mod rbr;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
pub struct THR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
pub mod thr;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub struct DLL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dll;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub struct DLM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dlm;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
pub struct IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
pub mod ier;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub struct IIR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub struct FCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub mod fcr;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub struct LCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "Modem Control Register."]
pub struct MCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Modem Control Register."]
pub mod mcr;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub struct LSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "Modem Status Register."]
pub struct MSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Modem Status Register."]
pub mod msr;
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub struct SCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub mod scr;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub struct ACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
pub struct ICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
pub mod icr;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub struct FDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub struct OSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub struct TER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub mod ter;
#[doc = "Half duplex enable register."]
pub struct HDEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Half duplex enable register."]
pub mod hden;
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
pub struct SCICTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
pub mod scictrl;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub struct RS485CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub struct RS485ADRMATCH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS-485/EIA-485 direction control delay."]
pub struct RS485DLY {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "Synchronous mode control register."]
pub struct SYNCCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
