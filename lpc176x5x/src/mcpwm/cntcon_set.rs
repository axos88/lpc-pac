#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CNTCON_SET {
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
pub struct _TC0MCI0_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI0_RE_SETW<'a> {
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
pub struct _TC0MCI0_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI0_FE_SETW<'a> {
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
pub struct _TC0MCI1_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI1_RE_SETW<'a> {
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
pub struct _TC0MCI1_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI1_FE_SETW<'a> {
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
pub struct _TC0MCI2_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI2_RE_SETW<'a> {
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
pub struct _TC0MCI2_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0MCI2_FE_SETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TC1MCI0_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI0_RE_SETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TC1MCI0_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI0_FE_SETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TC1MCI1_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI1_RE_SETW<'a> {
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
pub struct _TC1MCI1_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI1_FE_SETW<'a> {
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
pub struct _TC1MCI2_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI2_RE_SETW<'a> {
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
pub struct _TC1MCI2_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1MCI2_FE_SETW<'a> {
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
pub struct _TC2MCI0_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI0_RE_SETW<'a> {
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
pub struct _TC2MCI0_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI0_FE_SETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TC2MCI1_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI1_RE_SETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TC2MCI1_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI1_FE_SETW<'a> {
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
#[doc = r"Proxy"]
pub struct _TC2MCI2_RE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI2_RE_SETW<'a> {
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
pub struct _TC2MCI2_FE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2MCI2_FE_SETW<'a> {
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
pub struct _CNTR0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTR0_SETW<'a> {
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
pub struct _CNTR1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTR1_SETW<'a> {
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
pub struct _CNTR2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTR2_SETW<'a> {
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_re_set(&mut self) -> _TC0MCI0_RE_SETW {
        _TC0MCI0_RE_SETW { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_fe_set(&mut self) -> _TC0MCI0_FE_SETW {
        _TC0MCI0_FE_SETW { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_re_set(&mut self) -> _TC0MCI1_RE_SETW {
        _TC0MCI1_RE_SETW { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_fe_set(&mut self) -> _TC0MCI1_FE_SETW {
        _TC0MCI1_FE_SETW { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_re_set(&mut self) -> _TC0MCI2_RE_SETW {
        _TC0MCI2_RE_SETW { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_fe_set(&mut self) -> _TC0MCI2_FE_SETW {
        _TC0MCI2_FE_SETW { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_re_set(&mut self) -> _TC1MCI0_RE_SETW {
        _TC1MCI0_RE_SETW { w: self }
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_fe_set(&mut self) -> _TC1MCI0_FE_SETW {
        _TC1MCI0_FE_SETW { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_re_set(&mut self) -> _TC1MCI1_RE_SETW {
        _TC1MCI1_RE_SETW { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_fe_set(&mut self) -> _TC1MCI1_FE_SETW {
        _TC1MCI1_FE_SETW { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_re_set(&mut self) -> _TC1MCI2_RE_SETW {
        _TC1MCI2_RE_SETW { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_fe_set(&mut self) -> _TC1MCI2_FE_SETW {
        _TC1MCI2_FE_SETW { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_re_set(&mut self) -> _TC2MCI0_RE_SETW {
        _TC2MCI0_RE_SETW { w: self }
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_fe_set(&mut self) -> _TC2MCI0_FE_SETW {
        _TC2MCI0_FE_SETW { w: self }
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_re_set(&mut self) -> _TC2MCI1_RE_SETW {
        _TC2MCI1_RE_SETW { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_fe_set(&mut self) -> _TC2MCI1_FE_SETW {
        _TC2MCI1_FE_SETW { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_re_set(&mut self) -> _TC2MCI2_RE_SETW {
        _TC2MCI2_RE_SETW { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_fe_set(&mut self) -> _TC2MCI2_FE_SETW {
        _TC2MCI2_FE_SETW { w: self }
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr0_set(&mut self) -> _CNTR0_SETW {
        _CNTR0_SETW { w: self }
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr1_set(&mut self) -> _CNTR1_SETW {
        _CNTR1_SETW { w: self }
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr2_set(&mut self) -> _CNTR2_SETW {
        _CNTR2_SETW { w: self }
    }
}
