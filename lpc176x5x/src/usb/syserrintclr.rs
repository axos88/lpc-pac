#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSERRINTCLR {
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
pub struct _EPERRINTCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR0W<'a> {
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
pub struct _EPERRINTCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR1W<'a> {
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
pub struct _EPERRINTCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR2W<'a> {
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
pub struct _EPERRINTCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR3W<'a> {
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
pub struct _EPERRINTCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR4W<'a> {
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
pub struct _EPERRINTCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR5W<'a> {
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
pub struct _EPERRINTCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR6W<'a> {
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
pub struct _EPERRINTCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR7W<'a> {
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
pub struct _EPERRINTCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR8W<'a> {
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
pub struct _EPERRINTCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR9W<'a> {
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
pub struct _EPERRINTCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR10W<'a> {
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
pub struct _EPERRINTCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR11W<'a> {
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
pub struct _EPERRINTCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR12W<'a> {
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
pub struct _EPERRINTCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR13W<'a> {
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
pub struct _EPERRINTCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR14W<'a> {
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
pub struct _EPERRINTCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR15W<'a> {
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
pub struct _EPERRINTCLR16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR16W<'a> {
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
pub struct _EPERRINTCLR17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR17W<'a> {
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
pub struct _EPERRINTCLR18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR18W<'a> {
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
pub struct _EPERRINTCLR19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR19W<'a> {
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
pub struct _EPERRINTCLR20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR20W<'a> {
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
pub struct _EPERRINTCLR21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR21W<'a> {
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
pub struct _EPERRINTCLR22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR22W<'a> {
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
pub struct _EPERRINTCLR23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR23W<'a> {
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
pub struct _EPERRINTCLR24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR24W<'a> {
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
pub struct _EPERRINTCLR25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR25W<'a> {
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
pub struct _EPERRINTCLR26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR26W<'a> {
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
pub struct _EPERRINTCLR27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR27W<'a> {
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
pub struct _EPERRINTCLR28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR28W<'a> {
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
pub struct _EPERRINTCLR29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR29W<'a> {
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
pub struct _EPERRINTCLR30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR30W<'a> {
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
pub struct _EPERRINTCLR31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPERRINTCLR31W<'a> {
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr0(&mut self) -> _EPERRINTCLR0W {
        _EPERRINTCLR0W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr1(&mut self) -> _EPERRINTCLR1W {
        _EPERRINTCLR1W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr2(&mut self) -> _EPERRINTCLR2W {
        _EPERRINTCLR2W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr3(&mut self) -> _EPERRINTCLR3W {
        _EPERRINTCLR3W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr4(&mut self) -> _EPERRINTCLR4W {
        _EPERRINTCLR4W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr5(&mut self) -> _EPERRINTCLR5W {
        _EPERRINTCLR5W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr6(&mut self) -> _EPERRINTCLR6W {
        _EPERRINTCLR6W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr7(&mut self) -> _EPERRINTCLR7W {
        _EPERRINTCLR7W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr8(&mut self) -> _EPERRINTCLR8W {
        _EPERRINTCLR8W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr9(&mut self) -> _EPERRINTCLR9W {
        _EPERRINTCLR9W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr10(&mut self) -> _EPERRINTCLR10W {
        _EPERRINTCLR10W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr11(&mut self) -> _EPERRINTCLR11W {
        _EPERRINTCLR11W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr12(&mut self) -> _EPERRINTCLR12W {
        _EPERRINTCLR12W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr13(&mut self) -> _EPERRINTCLR13W {
        _EPERRINTCLR13W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr14(&mut self) -> _EPERRINTCLR14W {
        _EPERRINTCLR14W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr15(&mut self) -> _EPERRINTCLR15W {
        _EPERRINTCLR15W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr16(&mut self) -> _EPERRINTCLR16W {
        _EPERRINTCLR16W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr17(&mut self) -> _EPERRINTCLR17W {
        _EPERRINTCLR17W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr18(&mut self) -> _EPERRINTCLR18W {
        _EPERRINTCLR18W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr19(&mut self) -> _EPERRINTCLR19W {
        _EPERRINTCLR19W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr20(&mut self) -> _EPERRINTCLR20W {
        _EPERRINTCLR20W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr21(&mut self) -> _EPERRINTCLR21W {
        _EPERRINTCLR21W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr22(&mut self) -> _EPERRINTCLR22W {
        _EPERRINTCLR22W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr23(&mut self) -> _EPERRINTCLR23W {
        _EPERRINTCLR23W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr24(&mut self) -> _EPERRINTCLR24W {
        _EPERRINTCLR24W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr25(&mut self) -> _EPERRINTCLR25W {
        _EPERRINTCLR25W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr26(&mut self) -> _EPERRINTCLR26W {
        _EPERRINTCLR26W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr27(&mut self) -> _EPERRINTCLR27W {
        _EPERRINTCLR27W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr28(&mut self) -> _EPERRINTCLR28W {
        _EPERRINTCLR28W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr29(&mut self) -> _EPERRINTCLR29W {
        _EPERRINTCLR29W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr30(&mut self) -> _EPERRINTCLR30W {
        _EPERRINTCLR30W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr31(&mut self) -> _EPERRINTCLR31W {
        _EPERRINTCLR31W { w: self }
    }
}
