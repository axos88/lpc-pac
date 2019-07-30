#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXSTATUS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXSTATUS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - If 1, the receive channel is active. If 0, the receive channel is inactive."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
