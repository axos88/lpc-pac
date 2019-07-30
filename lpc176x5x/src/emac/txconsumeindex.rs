#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TXCONSUMEINDEX {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXCI_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
    #[inline(always)]
    pub fn txci(&self) -> TXCI_R {
        TXCI_R::new((self.bits() & 0xffff) as u16)
    }
}
