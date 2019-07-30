#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NDDRINTCLR {
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
pub struct _EPNDDINTCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR0W<'a> {
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
pub struct _EPNDDINTCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR1W<'a> {
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
pub struct _EPNDDINTCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR2W<'a> {
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
pub struct _EPNDDINTCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR3W<'a> {
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
pub struct _EPNDDINTCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR4W<'a> {
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
pub struct _EPNDDINTCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR5W<'a> {
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
pub struct _EPNDDINTCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR6W<'a> {
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
pub struct _EPNDDINTCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR7W<'a> {
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
pub struct _EPNDDINTCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR8W<'a> {
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
pub struct _EPNDDINTCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR9W<'a> {
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
pub struct _EPNDDINTCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR10W<'a> {
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
pub struct _EPNDDINTCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR11W<'a> {
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
pub struct _EPNDDINTCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR12W<'a> {
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
pub struct _EPNDDINTCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR13W<'a> {
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
pub struct _EPNDDINTCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR14W<'a> {
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
pub struct _EPNDDINTCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR15W<'a> {
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
pub struct _EPNDDINTCLR16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR16W<'a> {
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
pub struct _EPNDDINTCLR17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR17W<'a> {
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
pub struct _EPNDDINTCLR18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR18W<'a> {
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
pub struct _EPNDDINTCLR19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR19W<'a> {
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
pub struct _EPNDDINTCLR20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR20W<'a> {
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
pub struct _EPNDDINTCLR21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR21W<'a> {
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
pub struct _EPNDDINTCLR22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR22W<'a> {
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
pub struct _EPNDDINTCLR23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR23W<'a> {
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
pub struct _EPNDDINTCLR24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR24W<'a> {
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
pub struct _EPNDDINTCLR25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR25W<'a> {
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
pub struct _EPNDDINTCLR26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR26W<'a> {
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
pub struct _EPNDDINTCLR27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR27W<'a> {
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
pub struct _EPNDDINTCLR28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR28W<'a> {
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
pub struct _EPNDDINTCLR29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR29W<'a> {
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
pub struct _EPNDDINTCLR30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR30W<'a> {
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
pub struct _EPNDDINTCLR31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTCLR31W<'a> {
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr0(&mut self) -> _EPNDDINTCLR0W {
        _EPNDDINTCLR0W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr1(&mut self) -> _EPNDDINTCLR1W {
        _EPNDDINTCLR1W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr2(&mut self) -> _EPNDDINTCLR2W {
        _EPNDDINTCLR2W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr3(&mut self) -> _EPNDDINTCLR3W {
        _EPNDDINTCLR3W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr4(&mut self) -> _EPNDDINTCLR4W {
        _EPNDDINTCLR4W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr5(&mut self) -> _EPNDDINTCLR5W {
        _EPNDDINTCLR5W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr6(&mut self) -> _EPNDDINTCLR6W {
        _EPNDDINTCLR6W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr7(&mut self) -> _EPNDDINTCLR7W {
        _EPNDDINTCLR7W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr8(&mut self) -> _EPNDDINTCLR8W {
        _EPNDDINTCLR8W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr9(&mut self) -> _EPNDDINTCLR9W {
        _EPNDDINTCLR9W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr10(&mut self) -> _EPNDDINTCLR10W {
        _EPNDDINTCLR10W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr11(&mut self) -> _EPNDDINTCLR11W {
        _EPNDDINTCLR11W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr12(&mut self) -> _EPNDDINTCLR12W {
        _EPNDDINTCLR12W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr13(&mut self) -> _EPNDDINTCLR13W {
        _EPNDDINTCLR13W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr14(&mut self) -> _EPNDDINTCLR14W {
        _EPNDDINTCLR14W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr15(&mut self) -> _EPNDDINTCLR15W {
        _EPNDDINTCLR15W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr16(&mut self) -> _EPNDDINTCLR16W {
        _EPNDDINTCLR16W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr17(&mut self) -> _EPNDDINTCLR17W {
        _EPNDDINTCLR17W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr18(&mut self) -> _EPNDDINTCLR18W {
        _EPNDDINTCLR18W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr19(&mut self) -> _EPNDDINTCLR19W {
        _EPNDDINTCLR19W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr20(&mut self) -> _EPNDDINTCLR20W {
        _EPNDDINTCLR20W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr21(&mut self) -> _EPNDDINTCLR21W {
        _EPNDDINTCLR21W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr22(&mut self) -> _EPNDDINTCLR22W {
        _EPNDDINTCLR22W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr23(&mut self) -> _EPNDDINTCLR23W {
        _EPNDDINTCLR23W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr24(&mut self) -> _EPNDDINTCLR24W {
        _EPNDDINTCLR24W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr25(&mut self) -> _EPNDDINTCLR25W {
        _EPNDDINTCLR25W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr26(&mut self) -> _EPNDDINTCLR26W {
        _EPNDDINTCLR26W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr27(&mut self) -> _EPNDDINTCLR27W {
        _EPNDDINTCLR27W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr28(&mut self) -> _EPNDDINTCLR28W {
        _EPNDDINTCLR28W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr29(&mut self) -> _EPNDDINTCLR29W {
        _EPNDDINTCLR29W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr30(&mut self) -> _EPNDDINTCLR30W {
        _EPNDDINTCLR30W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr31(&mut self) -> _EPNDDINTCLR31W {
        _EPNDDINTCLR31W { w: self }
    }
}
