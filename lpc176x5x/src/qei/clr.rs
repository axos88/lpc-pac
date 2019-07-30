#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLR {
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
pub struct _INX_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INX_INTW<'a> {
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
pub struct _TIM_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM_INTW<'a> {
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
pub struct _VELC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _VELC_INTW<'a> {
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
pub struct _DIR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIR_INTW<'a> {
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
pub struct _ERR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERR_INTW<'a> {
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
pub struct _ENCLK_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCLK_INTW<'a> {
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
pub struct _POS0_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS0_INTW<'a> {
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
pub struct _POS1_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS1_INTW<'a> {
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
pub struct _POS2_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS2_INTW<'a> {
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
pub struct _REV0_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV0_INTW<'a> {
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
pub struct _POS0REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS0REV_INTW<'a> {
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
pub struct _POS1REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS1REV_INTW<'a> {
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
pub struct _POS2REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS2REV_INTW<'a> {
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
pub struct _REV1_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV1_INTW<'a> {
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
pub struct _REV2_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV2_INTW<'a> {
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
pub struct _MAXPOS_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXPOS_INTW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Writing a 1 clears the INX_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn inx_int(&mut self) -> _INX_INTW {
        _INX_INTW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the TIN_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn tim_int(&mut self) -> _TIM_INTW {
        _TIM_INTW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the VELC_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn velc_int(&mut self) -> _VELC_INTW {
        _VELC_INTW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the DIR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn dir_int(&mut self) -> _DIR_INTW {
        _DIR_INTW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the ERR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn err_int(&mut self) -> _ERR_INTW {
        _ERR_INTW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the ENCLK_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn enclk_int(&mut self) -> _ENCLK_INTW {
        _ENCLK_INTW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the POS0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0_int(&mut self) -> _POS0_INTW {
        _POS0_INTW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the POS1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1_int(&mut self) -> _POS1_INTW {
        _POS1_INTW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 clears the POS2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2_int(&mut self) -> _POS2_INTW {
        _POS2_INTW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 clears the REV0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev0_int(&mut self) -> _REV0_INTW {
        _REV0_INTW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 clears the POS0REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
        _POS0REV_INTW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 clears the POS1REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
        _POS1REV_INTW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the POS2REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
        _POS2REV_INTW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the REV1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev1_int(&mut self) -> _REV1_INTW {
        _REV1_INTW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 clears the REV2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev2_int(&mut self) -> _REV2_INTW {
        _REV2_INTW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 clears the MAXPOS_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
        _MAXPOS_INTW { w: self }
    }
}
