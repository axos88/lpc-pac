#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EOTINTSET {
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
pub struct _EPTXINTSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET0W<'a> {
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
pub struct _EPTXINTSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET1W<'a> {
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
pub struct _EPTXINTSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET2W<'a> {
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
pub struct _EPTXINTSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET3W<'a> {
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
pub struct _EPTXINTSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET4W<'a> {
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
pub struct _EPTXINTSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET5W<'a> {
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
pub struct _EPTXINTSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET6W<'a> {
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
pub struct _EPTXINTSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET7W<'a> {
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
pub struct _EPTXINTSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET8W<'a> {
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
pub struct _EPTXINTSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET9W<'a> {
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
pub struct _EPTXINTSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET10W<'a> {
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
pub struct _EPTXINTSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET11W<'a> {
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
pub struct _EPTXINTSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET12W<'a> {
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
pub struct _EPTXINTSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET13W<'a> {
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
pub struct _EPTXINTSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET14W<'a> {
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
pub struct _EPTXINTSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET15W<'a> {
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
pub struct _EPTXINTSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET16W<'a> {
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
pub struct _EPTXINTSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET17W<'a> {
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
pub struct _EPTXINTSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET18W<'a> {
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
pub struct _EPTXINTSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET19W<'a> {
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
pub struct _EPTXINTSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET20W<'a> {
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
pub struct _EPTXINTSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET21W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET22W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET23W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET24W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET25W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET26W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET27W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET28W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EPTXINTSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET29W<'a> {
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
pub struct _EPTXINTSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET30W<'a> {
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
pub struct _EPTXINTSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTSET31W<'a> {
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
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset0(&mut self) -> _EPTXINTSET0W {
        _EPTXINTSET0W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset1(&mut self) -> _EPTXINTSET1W {
        _EPTXINTSET1W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset2(&mut self) -> _EPTXINTSET2W {
        _EPTXINTSET2W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset3(&mut self) -> _EPTXINTSET3W {
        _EPTXINTSET3W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset4(&mut self) -> _EPTXINTSET4W {
        _EPTXINTSET4W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset5(&mut self) -> _EPTXINTSET5W {
        _EPTXINTSET5W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset6(&mut self) -> _EPTXINTSET6W {
        _EPTXINTSET6W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset7(&mut self) -> _EPTXINTSET7W {
        _EPTXINTSET7W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset8(&mut self) -> _EPTXINTSET8W {
        _EPTXINTSET8W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset9(&mut self) -> _EPTXINTSET9W {
        _EPTXINTSET9W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset10(&mut self) -> _EPTXINTSET10W {
        _EPTXINTSET10W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset11(&mut self) -> _EPTXINTSET11W {
        _EPTXINTSET11W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset12(&mut self) -> _EPTXINTSET12W {
        _EPTXINTSET12W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset13(&mut self) -> _EPTXINTSET13W {
        _EPTXINTSET13W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset14(&mut self) -> _EPTXINTSET14W {
        _EPTXINTSET14W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset15(&mut self) -> _EPTXINTSET15W {
        _EPTXINTSET15W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset16(&mut self) -> _EPTXINTSET16W {
        _EPTXINTSET16W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset17(&mut self) -> _EPTXINTSET17W {
        _EPTXINTSET17W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset18(&mut self) -> _EPTXINTSET18W {
        _EPTXINTSET18W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset19(&mut self) -> _EPTXINTSET19W {
        _EPTXINTSET19W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset20(&mut self) -> _EPTXINTSET20W {
        _EPTXINTSET20W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset21(&mut self) -> _EPTXINTSET21W {
        _EPTXINTSET21W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset22(&mut self) -> _EPTXINTSET22W {
        _EPTXINTSET22W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset23(&mut self) -> _EPTXINTSET23W {
        _EPTXINTSET23W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset24(&mut self) -> _EPTXINTSET24W {
        _EPTXINTSET24W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset25(&mut self) -> _EPTXINTSET25W {
        _EPTXINTSET25W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset26(&mut self) -> _EPTXINTSET26W {
        _EPTXINTSET26W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset27(&mut self) -> _EPTXINTSET27W {
        _EPTXINTSET27W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset28(&mut self) -> _EPTXINTSET28W {
        _EPTXINTSET28W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset29(&mut self) -> _EPTXINTSET29W {
        _EPTXINTSET29W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset30(&mut self) -> _EPTXINTSET30W {
        _EPTXINTSET30W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset31(&mut self) -> _EPTXINTSET31W {
        _EPTXINTSET31W { w: self }
    }
}
