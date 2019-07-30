#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PLL1STAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSEL1_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PSEL1_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PLLE1_STAT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PLLC1_STAT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PLOCK1_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn msel1(&self) -> MSEL1_R {
        MSEL1_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL1 Divider value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn psel1(&self) -> PSEL1_R {
        PSEL1_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle1_stat(&self) -> PLLE1_STAT_R {
        PLLE1_STAT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn pllc1_stat(&self) -> PLLC1_STAT_R {
        PLLC1_STAT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock1(&self) -> PLOCK1_R {
        PLOCK1_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
