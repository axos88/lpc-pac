#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CIIR {
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
pub type IMSEC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMSECW<'a> {
    w: &'a mut W,
}
impl<'a> _IMSECW<'a> {
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
pub type IMMIN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMMINW<'a> {
    w: &'a mut W,
}
impl<'a> _IMMINW<'a> {
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
pub type IMHOUR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMHOURW<'a> {
    w: &'a mut W,
}
impl<'a> _IMHOURW<'a> {
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
#[doc = r"Reader of the field"]
pub type IMDOM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMDOMW<'a> {
    w: &'a mut W,
}
impl<'a> _IMDOMW<'a> {
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
#[doc = r"Reader of the field"]
pub type IMDOW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMDOWW<'a> {
    w: &'a mut W,
}
impl<'a> _IMDOWW<'a> {
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
#[doc = r"Reader of the field"]
pub type IMDOY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMDOYW<'a> {
    w: &'a mut W,
}
impl<'a> _IMDOYW<'a> {
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
#[doc = r"Reader of the field"]
pub type IMMON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMMONW<'a> {
    w: &'a mut W,
}
impl<'a> _IMMONW<'a> {
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
#[doc = r"Reader of the field"]
pub type IMYEAR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IMYEARW<'a> {
    w: &'a mut W,
}
impl<'a> _IMYEARW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&self) -> IMSEC_R {
        IMSEC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&self) -> IMMIN_R {
        IMMIN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&self) -> IMHOUR_R {
        IMHOUR_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&self) -> IMDOM_R {
        IMDOM_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&self) -> IMDOW_R {
        IMDOW_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&self) -> IMDOY_R {
        IMDOY_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&self) -> IMMON_R {
        IMMON_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&self) -> IMYEAR_R {
        IMYEAR_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&mut self) -> _IMSECW {
        _IMSECW { w: self }
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&mut self) -> _IMMINW {
        _IMMINW { w: self }
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&mut self) -> _IMHOURW {
        _IMHOURW { w: self }
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&mut self) -> _IMDOMW {
        _IMDOMW { w: self }
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&mut self) -> _IMDOWW {
        _IMDOWW { w: self }
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&mut self) -> _IMDOYW {
        _IMDOYW { w: self }
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&mut self) -> _IMMONW {
        _IMMONW { w: self }
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&mut self) -> _IMYEARW {
        _IMYEARW { w: self }
    }
}
