#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MRDD {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type READDATA_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
    #[inline(always)]
    pub fn readdata(&self) -> READDATA_R {
        READDATA_R::new((self.bits() & 0xffff) as u16)
    }
}
