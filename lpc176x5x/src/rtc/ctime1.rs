#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTIME1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DOM_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MONTH_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type YEAR_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits() >> 16) & 0x0fff) as u16)
    }
}
