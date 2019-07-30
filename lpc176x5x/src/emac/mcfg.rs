#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFG {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type SCANINC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCANINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SCANINCW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SUPPPREAMBLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SUPPPREAMBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPPPREAMBLEW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLOCKSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CLOCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RESETMIIMGMT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETMIIMGMTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETMIIMGMTW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&self) -> SCANINC_R {
        SCANINC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&self) -> SUPPPREAMBLE_R {
        SUPPPREAMBLE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&self) -> CLOCKSEL_R {
        CLOCKSEL_R::new(((self.bits() >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&self) -> RESETMIIMGMT_R {
        RESETMIIMGMT_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&mut self) -> _SCANINCW {
        _SCANINCW { w: self }
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&mut self) -> _SUPPPREAMBLEW {
        _SUPPPREAMBLEW { w: self }
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&mut self) -> _CLOCKSELW {
        _CLOCKSELW { w: self }
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&mut self) -> _RESETMIIMGMTW {
        _RESETMIIMGMTW { w: self }
    }
}
