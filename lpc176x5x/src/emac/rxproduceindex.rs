#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXPRODUCEINDEX {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXPRODUCEIX_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be filled next by the receive datapath."]
    #[inline(always)]
    pub fn rxproduceix(&self) -> RXPRODUCEIX_R {
        RXPRODUCEIX_R::new((self.bits() & 0xffff) as u16)
    }
}
