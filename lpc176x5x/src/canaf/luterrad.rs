#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LUTERRAD {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type LUTERRAD_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:10 - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
    #[inline(always)]
    pub fn luterrad(&self) -> LUTERRAD_R {
        LUTERRAD_R::new(((self.bits() >> 2) & 0x01ff) as u16)
    }
}
