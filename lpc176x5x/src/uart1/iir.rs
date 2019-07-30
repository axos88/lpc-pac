#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IIR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `INTSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUSR {
    #[doc = "At least one interrupt is pending."]
    AT_LEAST_ONE_INTERRU,
    #[doc = "No interrupt is pending."]
    NO_INTERRUPT_IS_PEND,
}
impl crate::ToBits<bool> for INTSTATUSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTSTATUSR::AT_LEAST_ONE_INTERRU => false,
            INTSTATUSR::NO_INTERRUPT_IS_PEND => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTSTATUS_R = crate::FR<bool, INTSTATUSR>;
impl INTSTATUS_R {
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`"]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == INTSTATUSR::AT_LEAST_ONE_INTERRU
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`"]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == INTSTATUSR::NO_INTERRUPT_IS_PEND
    }
}
#[doc = "Possible values of the field `INTID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTIDR {
    #[doc = "1   - Receive Line Status (RLS)."]
    RLS,
    #[doc = "2a - Receive Data Available (RDA)."]
    RDA,
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    CTI,
    #[doc = "3   - THRE Interrupt."]
    THRE,
    #[doc = "4   - Modem Interrupt."]
    MODEM,
}
impl crate::ToBits<u8> for INTIDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INTIDR::RLS => 3,
            INTIDR::RDA => 2,
            INTIDR::CTI => 6,
            INTIDR::THRE => 1,
            INTIDR::MODEM => 0,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTID_R = crate::FR<u8, INTIDR>;
impl INTID_R {
    #[doc = "Checks if the value of the field is `RLS`"]
    #[inline(always)]
    pub fn is_rls(&self) -> bool {
        *self == INTIDR::RLS
    }
    #[doc = "Checks if the value of the field is `RDA`"]
    #[inline(always)]
    pub fn is_rda(&self) -> bool {
        *self == INTIDR::RDA
    }
    #[doc = "Checks if the value of the field is `CTI`"]
    #[inline(always)]
    pub fn is_cti(&self) -> bool {
        *self == INTIDR::CTI
    }
    #[doc = "Checks if the value of the field is `THRE`"]
    #[inline(always)]
    pub fn is_thre(&self) -> bool {
        *self == INTIDR::THRE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == INTIDR::MODEM
    }
}
#[doc = r"Reader of the field"]
pub type FIFOENABLE_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ABEOINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ABTOINT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\] not listed below are reserved (100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
