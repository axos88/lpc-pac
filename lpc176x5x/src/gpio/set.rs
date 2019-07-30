#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SET {
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
pub type PINSET0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET0W<'a> {
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
pub type PINSET1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET1W<'a> {
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
pub type PINSET2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET2W<'a> {
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
pub type PINSET3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET3W<'a> {
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
pub type PINSET4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET4W<'a> {
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
pub type PINSET5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET5W<'a> {
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
pub type PINSET6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET6W<'a> {
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
pub type PINSET7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET7W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET8W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET9W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET10W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET11W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET12W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET13W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET14W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET15W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET16W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET17W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET18W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET19W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET20W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET21W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET22W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET23W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET24W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET25W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET26W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET27W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET28W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET29W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET30W<'a> {
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
#[doc = r"Reader of the field"]
pub type PINSET31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET31W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset0(&self) -> PINSET0_R {
        PINSET0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset1(&self) -> PINSET1_R {
        PINSET1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset2(&self) -> PINSET2_R {
        PINSET2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset3(&self) -> PINSET3_R {
        PINSET3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset4(&self) -> PINSET4_R {
        PINSET4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset5(&self) -> PINSET5_R {
        PINSET5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset6(&self) -> PINSET6_R {
        PINSET6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset7(&self) -> PINSET7_R {
        PINSET7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset8(&self) -> PINSET8_R {
        PINSET8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset9(&self) -> PINSET9_R {
        PINSET9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset10(&self) -> PINSET10_R {
        PINSET10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset11(&self) -> PINSET11_R {
        PINSET11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset12(&self) -> PINSET12_R {
        PINSET12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset13(&self) -> PINSET13_R {
        PINSET13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset14(&self) -> PINSET14_R {
        PINSET14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset15(&self) -> PINSET15_R {
        PINSET15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset16(&self) -> PINSET16_R {
        PINSET16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset17(&self) -> PINSET17_R {
        PINSET17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset18(&self) -> PINSET18_R {
        PINSET18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset19(&self) -> PINSET19_R {
        PINSET19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset20(&self) -> PINSET20_R {
        PINSET20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset21(&self) -> PINSET21_R {
        PINSET21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset22(&self) -> PINSET22_R {
        PINSET22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset23(&self) -> PINSET23_R {
        PINSET23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset24(&self) -> PINSET24_R {
        PINSET24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset25(&self) -> PINSET25_R {
        PINSET25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset26(&self) -> PINSET26_R {
        PINSET26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset27(&self) -> PINSET27_R {
        PINSET27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset28(&self) -> PINSET28_R {
        PINSET28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset29(&self) -> PINSET29_R {
        PINSET29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset30(&self) -> PINSET30_R {
        PINSET30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset31(&self) -> PINSET31_R {
        PINSET31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset0(&mut self) -> _PINSET0W {
        _PINSET0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset1(&mut self) -> _PINSET1W {
        _PINSET1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset2(&mut self) -> _PINSET2W {
        _PINSET2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset3(&mut self) -> _PINSET3W {
        _PINSET3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset4(&mut self) -> _PINSET4W {
        _PINSET4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset5(&mut self) -> _PINSET5W {
        _PINSET5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset6(&mut self) -> _PINSET6W {
        _PINSET6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset7(&mut self) -> _PINSET7W {
        _PINSET7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset8(&mut self) -> _PINSET8W {
        _PINSET8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset9(&mut self) -> _PINSET9W {
        _PINSET9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset10(&mut self) -> _PINSET10W {
        _PINSET10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset11(&mut self) -> _PINSET11W {
        _PINSET11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset12(&mut self) -> _PINSET12W {
        _PINSET12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset13(&mut self) -> _PINSET13W {
        _PINSET13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset14(&mut self) -> _PINSET14W {
        _PINSET14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset15(&mut self) -> _PINSET15W {
        _PINSET15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset16(&mut self) -> _PINSET16W {
        _PINSET16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset17(&mut self) -> _PINSET17W {
        _PINSET17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset18(&mut self) -> _PINSET18W {
        _PINSET18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset19(&mut self) -> _PINSET19W {
        _PINSET19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset20(&mut self) -> _PINSET20W {
        _PINSET20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset21(&mut self) -> _PINSET21W {
        _PINSET21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset22(&mut self) -> _PINSET22W {
        _PINSET22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset23(&mut self) -> _PINSET23W {
        _PINSET23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset24(&mut self) -> _PINSET24W {
        _PINSET24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset25(&mut self) -> _PINSET25W {
        _PINSET25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset26(&mut self) -> _PINSET26W {
        _PINSET26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset27(&mut self) -> _PINSET27W {
        _PINSET27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset28(&mut self) -> _PINSET28W {
        _PINSET28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset29(&mut self) -> _PINSET29W {
        _PINSET29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset30(&mut self) -> _PINSET30W {
        _PINSET30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset31(&mut self) -> _PINSET31W {
        _PINSET31W { w: self }
    }
}
