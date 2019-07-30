#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSERRINTSET {
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
pub struct _EPERRINTSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET0W<'a> {
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
pub struct _EPERRINTSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET1W<'a> {
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
pub struct _EPERRINTSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET2W<'a> {
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
pub struct _EPERRINTSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET3W<'a> {
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
pub struct _EPERRINTSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET4W<'a> {
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
pub struct _EPERRINTSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET5W<'a> {
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
pub struct _EPERRINTSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET6W<'a> {
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
pub struct _EPERRINTSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET7W<'a> {
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
pub struct _EPERRINTSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET8W<'a> {
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
pub struct _EPERRINTSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET9W<'a> {
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
pub struct _EPERRINTSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET10W<'a> {
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
pub struct _EPERRINTSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET11W<'a> {
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
pub struct _EPERRINTSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET12W<'a> {
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
pub struct _EPERRINTSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET13W<'a> {
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
pub struct _EPERRINTSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET14W<'a> {
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
pub struct _EPERRINTSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET15W<'a> {
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
pub struct _EPERRINTSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET16W<'a> {
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
pub struct _EPERRINTSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET17W<'a> {
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
pub struct _EPERRINTSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET18W<'a> {
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
pub struct _EPERRINTSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET19W<'a> {
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
pub struct _EPERRINTSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET20W<'a> {
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
pub struct _EPERRINTSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET21W<'a> {
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
pub struct _EPERRINTSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET22W<'a> {
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
pub struct _EPERRINTSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET23W<'a> {
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
pub struct _EPERRINTSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET24W<'a> {
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
pub struct _EPERRINTSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET25W<'a> {
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
pub struct _EPERRINTSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET26W<'a> {
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
pub struct _EPERRINTSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET27W<'a> {
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
pub struct _EPERRINTSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET28W<'a> {
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
pub struct _EPERRINTSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET29W<'a> {
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
pub struct _EPERRINTSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET30W<'a> {
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
pub struct _EPERRINTSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTSET31W<'a> {
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
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset0(&mut self) -> _EPERRINTSET0W {
        _EPERRINTSET0W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset1(&mut self) -> _EPERRINTSET1W {
        _EPERRINTSET1W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset2(&mut self) -> _EPERRINTSET2W {
        _EPERRINTSET2W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset3(&mut self) -> _EPERRINTSET3W {
        _EPERRINTSET3W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset4(&mut self) -> _EPERRINTSET4W {
        _EPERRINTSET4W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset5(&mut self) -> _EPERRINTSET5W {
        _EPERRINTSET5W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset6(&mut self) -> _EPERRINTSET6W {
        _EPERRINTSET6W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset7(&mut self) -> _EPERRINTSET7W {
        _EPERRINTSET7W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset8(&mut self) -> _EPERRINTSET8W {
        _EPERRINTSET8W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset9(&mut self) -> _EPERRINTSET9W {
        _EPERRINTSET9W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset10(&mut self) -> _EPERRINTSET10W {
        _EPERRINTSET10W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset11(&mut self) -> _EPERRINTSET11W {
        _EPERRINTSET11W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset12(&mut self) -> _EPERRINTSET12W {
        _EPERRINTSET12W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset13(&mut self) -> _EPERRINTSET13W {
        _EPERRINTSET13W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset14(&mut self) -> _EPERRINTSET14W {
        _EPERRINTSET14W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset15(&mut self) -> _EPERRINTSET15W {
        _EPERRINTSET15W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset16(&mut self) -> _EPERRINTSET16W {
        _EPERRINTSET16W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset17(&mut self) -> _EPERRINTSET17W {
        _EPERRINTSET17W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset18(&mut self) -> _EPERRINTSET18W {
        _EPERRINTSET18W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset19(&mut self) -> _EPERRINTSET19W {
        _EPERRINTSET19W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset20(&mut self) -> _EPERRINTSET20W {
        _EPERRINTSET20W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset21(&mut self) -> _EPERRINTSET21W {
        _EPERRINTSET21W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset22(&mut self) -> _EPERRINTSET22W {
        _EPERRINTSET22W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset23(&mut self) -> _EPERRINTSET23W {
        _EPERRINTSET23W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset24(&mut self) -> _EPERRINTSET24W {
        _EPERRINTSET24W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset25(&mut self) -> _EPERRINTSET25W {
        _EPERRINTSET25W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset26(&mut self) -> _EPERRINTSET26W {
        _EPERRINTSET26W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset27(&mut self) -> _EPERRINTSET27W {
        _EPERRINTSET27W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset28(&mut self) -> _EPERRINTSET28W {
        _EPERRINTSET28W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset29(&mut self) -> _EPERRINTSET29W {
        _EPERRINTSET29W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset30(&mut self) -> _EPERRINTSET30W {
        _EPERRINTSET30W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset31(&mut self) -> _EPERRINTSET31W {
        _EPERRINTSET31W { w: self }
    }
}
