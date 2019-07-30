#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CMDDATA {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CMD_RDATA_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Command Read Data."]
    #[inline(always)]
    pub fn cmd_rdata(&self) -> CMD_RDATA_R {
        CMD_RDATA_R::new((self.bits() & 0xff) as u8)
    }
}
