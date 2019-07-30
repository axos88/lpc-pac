#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIN {
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
pub type PINVAL0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL0W<'a> {
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
pub type PINVAL1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL1W<'a> {
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
pub type PINVAL2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL2W<'a> {
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
pub type PINVAL3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL3W<'a> {
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
pub type PINVAL4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL4W<'a> {
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
pub type PINVAL5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL5W<'a> {
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
pub type PINVAL6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL6W<'a> {
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
pub type PINVAL7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL7W<'a> {
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
pub type PINVAL8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL8W<'a> {
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
pub type PINVAL9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL9W<'a> {
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
pub type PINVAL10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL10W<'a> {
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
pub type PINVAL11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL11W<'a> {
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
pub type PINVAL12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL12W<'a> {
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
pub type PINVAL13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL13W<'a> {
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
pub type PINVAL14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL14W<'a> {
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
pub type PINVAL15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL15W<'a> {
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
pub type PINVAL16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL16W<'a> {
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
pub type PINVAL17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL17W<'a> {
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
pub type PINVAL18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL18W<'a> {
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
pub type PINVAL19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL19W<'a> {
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
pub type PINVAL20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL20W<'a> {
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
pub type PINVAL21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL21W<'a> {
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
pub type PINVAL22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL22W<'a> {
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
pub type PINVAL23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL23W<'a> {
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
pub type PINVAL24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL24W<'a> {
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
pub type PINVAL25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL25W<'a> {
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
pub type PINVAL26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL26W<'a> {
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
pub type PINVAL27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL27W<'a> {
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
pub type PINVAL28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL28W<'a> {
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
pub type PINVAL29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL29W<'a> {
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
pub type PINVAL30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL30W<'a> {
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
pub type PINVAL31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINVAL31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL31W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval0(&self) -> PINVAL0_R {
        PINVAL0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval1(&self) -> PINVAL1_R {
        PINVAL1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval2(&self) -> PINVAL2_R {
        PINVAL2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval3(&self) -> PINVAL3_R {
        PINVAL3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval4(&self) -> PINVAL4_R {
        PINVAL4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval5(&self) -> PINVAL5_R {
        PINVAL5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval6(&self) -> PINVAL6_R {
        PINVAL6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval7(&self) -> PINVAL7_R {
        PINVAL7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval8(&self) -> PINVAL8_R {
        PINVAL8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval9(&self) -> PINVAL9_R {
        PINVAL9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval10(&self) -> PINVAL10_R {
        PINVAL10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval11(&self) -> PINVAL11_R {
        PINVAL11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval12(&self) -> PINVAL12_R {
        PINVAL12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval13(&self) -> PINVAL13_R {
        PINVAL13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval14(&self) -> PINVAL14_R {
        PINVAL14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval15(&self) -> PINVAL15_R {
        PINVAL15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval16(&self) -> PINVAL16_R {
        PINVAL16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval17(&self) -> PINVAL17_R {
        PINVAL17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval18(&self) -> PINVAL18_R {
        PINVAL18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval19(&self) -> PINVAL19_R {
        PINVAL19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval20(&self) -> PINVAL20_R {
        PINVAL20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval21(&self) -> PINVAL21_R {
        PINVAL21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval22(&self) -> PINVAL22_R {
        PINVAL22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval23(&self) -> PINVAL23_R {
        PINVAL23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval24(&self) -> PINVAL24_R {
        PINVAL24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval25(&self) -> PINVAL25_R {
        PINVAL25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval26(&self) -> PINVAL26_R {
        PINVAL26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval27(&self) -> PINVAL27_R {
        PINVAL27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval28(&self) -> PINVAL28_R {
        PINVAL28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval29(&self) -> PINVAL29_R {
        PINVAL29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval30(&self) -> PINVAL30_R {
        PINVAL30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval31(&self) -> PINVAL31_R {
        PINVAL31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval0(&mut self) -> _PINVAL0W {
        _PINVAL0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval1(&mut self) -> _PINVAL1W {
        _PINVAL1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval2(&mut self) -> _PINVAL2W {
        _PINVAL2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval3(&mut self) -> _PINVAL3W {
        _PINVAL3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval4(&mut self) -> _PINVAL4W {
        _PINVAL4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval5(&mut self) -> _PINVAL5W {
        _PINVAL5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval6(&mut self) -> _PINVAL6W {
        _PINVAL6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval7(&mut self) -> _PINVAL7W {
        _PINVAL7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval8(&mut self) -> _PINVAL8W {
        _PINVAL8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval9(&mut self) -> _PINVAL9W {
        _PINVAL9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval10(&mut self) -> _PINVAL10W {
        _PINVAL10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval11(&mut self) -> _PINVAL11W {
        _PINVAL11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval12(&mut self) -> _PINVAL12W {
        _PINVAL12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval13(&mut self) -> _PINVAL13W {
        _PINVAL13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval14(&mut self) -> _PINVAL14W {
        _PINVAL14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval15(&mut self) -> _PINVAL15W {
        _PINVAL15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval16(&mut self) -> _PINVAL16W {
        _PINVAL16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval17(&mut self) -> _PINVAL17W {
        _PINVAL17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval18(&mut self) -> _PINVAL18W {
        _PINVAL18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval19(&mut self) -> _PINVAL19W {
        _PINVAL19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval20(&mut self) -> _PINVAL20W {
        _PINVAL20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval21(&mut self) -> _PINVAL21W {
        _PINVAL21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval22(&mut self) -> _PINVAL22W {
        _PINVAL22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval23(&mut self) -> _PINVAL23W {
        _PINVAL23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval24(&mut self) -> _PINVAL24W {
        _PINVAL24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval25(&mut self) -> _PINVAL25W {
        _PINVAL25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval26(&mut self) -> _PINVAL26W {
        _PINVAL26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval27(&mut self) -> _PINVAL27W {
        _PINVAL27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval28(&mut self) -> _PINVAL28W {
        _PINVAL28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval29(&mut self) -> _PINVAL29W {
        _PINVAL29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval30(&mut self) -> _PINVAL30W {
        _PINVAL30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval31(&mut self) -> _PINVAL31W {
        _PINVAL31W { w: self }
    }
}
