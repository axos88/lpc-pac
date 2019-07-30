#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTIME0 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type SECONDS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MINUTES_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type HOURS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type DOW_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new(((self.bits() >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of week value in the range of 0 to 6"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits() >> 24) & 0x07) as u8)
    }
}
