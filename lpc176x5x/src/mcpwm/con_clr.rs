#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON_CLR {
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
pub struct _RUN0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN0_CLRW<'a> {
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
pub struct _CENTER0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER0_CLRW<'a> {
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
pub struct _POLA0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA0_CLRW<'a> {
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
pub struct _DTE0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE0_CLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _DISUP0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP0_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RUN1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN1_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _CENTER1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER1_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _POLA1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA1_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DTE1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE1_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DISUP1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP1_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RUN2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN2_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _CENTER2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER2_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _POLA2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA2_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DTE2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE2_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DISUP2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP2_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _INVBDC_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _INVBDC_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ACMOD_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMOD_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DCMODE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMODE_CLRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_clr(&mut self) -> _RUN0_CLRW {
        _RUN0_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_clr(&mut self) -> _CENTER0_CLRW {
        _CENTER0_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_clr(&mut self) -> _POLA0_CLRW {
        _POLA0_CLRW { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_clr(&mut self) -> _DTE0_CLRW {
        _DTE0_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_clr(&mut self) -> _DISUP0_CLRW {
        _DISUP0_CLRW { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_clr(&mut self) -> _RUN1_CLRW {
        _RUN1_CLRW { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_clr(&mut self) -> _CENTER1_CLRW {
        _CENTER1_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_clr(&mut self) -> _POLA1_CLRW {
        _POLA1_CLRW { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_clr(&mut self) -> _DTE1_CLRW {
        _DTE1_CLRW { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_clr(&mut self) -> _DISUP1_CLRW {
        _DISUP1_CLRW { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_clr(&mut self) -> _RUN2_CLRW {
        _RUN2_CLRW { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_clr(&mut self) -> _CENTER2_CLRW {
        _CENTER2_CLRW { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_clr(&mut self) -> _POLA2_CLRW {
        _POLA2_CLRW { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_clr(&mut self) -> _DTE2_CLRW {
        _DTE2_CLRW { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_clr(&mut self) -> _DISUP2_CLRW {
        _DISUP2_CLRW { w: self }
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_clr(&mut self) -> _INVBDC_CLRW {
        _INVBDC_CLRW { w: self }
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmod_clr(&mut self) -> _ACMOD_CLRW {
        _ACMOD_CLRW { w: self }
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_clr(&mut self) -> _DCMODE_CLRW {
        _DCMODE_CLRW { w: self }
    }
}
