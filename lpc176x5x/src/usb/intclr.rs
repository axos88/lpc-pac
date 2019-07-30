#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTCLR {
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
#[doc = r"Proxy"]
pub struct _TMR_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_CLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _REMOVE_PU_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _REMOVE_PU_CLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _HNP_FAILURE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_FAILURE_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HNP_SUCCES_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_SUCCES_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_clr(&mut self) -> _TMR_CLRW {
        _TMR_CLRW { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_clr(&mut self) -> _REMOVE_PU_CLRW {
        _REMOVE_PU_CLRW { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_clr(&mut self) -> _HNP_FAILURE_CLRW {
        _HNP_FAILURE_CLRW { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_clr(&mut self) -> _HNP_SUCCES_CLRW {
        _HNP_SUCCES_CLRW { w: self }
    }
}
