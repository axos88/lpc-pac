#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIR {
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
pub type PINDIR0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR0W<'a> {
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
pub type PINDIR1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR1W<'a> {
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
pub type PINDIR2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR2W<'a> {
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
pub type PINDIR3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR3W<'a> {
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
pub type PINDIR4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR4W<'a> {
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
pub type PINDIR5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR5W<'a> {
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
pub type PINDIR6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR6W<'a> {
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
pub type PINDIR7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR7W<'a> {
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
pub type PINDIR8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR8W<'a> {
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
pub type PINDIR9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR9W<'a> {
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
pub type PINDIR10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR10W<'a> {
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
pub type PINDIR11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR11W<'a> {
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
pub type PINDIR12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR12W<'a> {
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
pub type PINDIR13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR13W<'a> {
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
pub type PINDIR14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR14W<'a> {
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
pub type PINDIR15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR15W<'a> {
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
pub type PINDIR16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR16W<'a> {
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
pub type PINDIR17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR17W<'a> {
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
pub type PINDIR18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR18W<'a> {
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
pub type PINDIR19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR19W<'a> {
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
pub type PINDIR20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR20W<'a> {
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
pub type PINDIR21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR21W<'a> {
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
pub type PINDIR22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR22W<'a> {
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
pub type PINDIR23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR23W<'a> {
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
pub type PINDIR24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR24W<'a> {
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
pub type PINDIR25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR25W<'a> {
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
pub type PINDIR26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR26W<'a> {
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
pub type PINDIR27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR27W<'a> {
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
pub type PINDIR28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR28W<'a> {
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
pub type PINDIR29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR29W<'a> {
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
pub type PINDIR30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR30W<'a> {
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
pub type PINDIR31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINDIR31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR31W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir0(&self) -> PINDIR0_R {
        PINDIR0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir1(&self) -> PINDIR1_R {
        PINDIR1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir2(&self) -> PINDIR2_R {
        PINDIR2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir3(&self) -> PINDIR3_R {
        PINDIR3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir4(&self) -> PINDIR4_R {
        PINDIR4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir5(&self) -> PINDIR5_R {
        PINDIR5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir6(&self) -> PINDIR6_R {
        PINDIR6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir7(&self) -> PINDIR7_R {
        PINDIR7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir8(&self) -> PINDIR8_R {
        PINDIR8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir9(&self) -> PINDIR9_R {
        PINDIR9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir10(&self) -> PINDIR10_R {
        PINDIR10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir11(&self) -> PINDIR11_R {
        PINDIR11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir12(&self) -> PINDIR12_R {
        PINDIR12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir13(&self) -> PINDIR13_R {
        PINDIR13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir14(&self) -> PINDIR14_R {
        PINDIR14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir15(&self) -> PINDIR15_R {
        PINDIR15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir16(&self) -> PINDIR16_R {
        PINDIR16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir17(&self) -> PINDIR17_R {
        PINDIR17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir18(&self) -> PINDIR18_R {
        PINDIR18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir19(&self) -> PINDIR19_R {
        PINDIR19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir20(&self) -> PINDIR20_R {
        PINDIR20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir21(&self) -> PINDIR21_R {
        PINDIR21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir22(&self) -> PINDIR22_R {
        PINDIR22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir23(&self) -> PINDIR23_R {
        PINDIR23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir24(&self) -> PINDIR24_R {
        PINDIR24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir25(&self) -> PINDIR25_R {
        PINDIR25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir26(&self) -> PINDIR26_R {
        PINDIR26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir27(&self) -> PINDIR27_R {
        PINDIR27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir28(&self) -> PINDIR28_R {
        PINDIR28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir29(&self) -> PINDIR29_R {
        PINDIR29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir30(&self) -> PINDIR30_R {
        PINDIR30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir31(&self) -> PINDIR31_R {
        PINDIR31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir0(&mut self) -> _PINDIR0W {
        _PINDIR0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir1(&mut self) -> _PINDIR1W {
        _PINDIR1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir2(&mut self) -> _PINDIR2W {
        _PINDIR2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir3(&mut self) -> _PINDIR3W {
        _PINDIR3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir4(&mut self) -> _PINDIR4W {
        _PINDIR4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir5(&mut self) -> _PINDIR5W {
        _PINDIR5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir6(&mut self) -> _PINDIR6W {
        _PINDIR6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir7(&mut self) -> _PINDIR7W {
        _PINDIR7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir8(&mut self) -> _PINDIR8W {
        _PINDIR8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir9(&mut self) -> _PINDIR9W {
        _PINDIR9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir10(&mut self) -> _PINDIR10W {
        _PINDIR10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir11(&mut self) -> _PINDIR11W {
        _PINDIR11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir12(&mut self) -> _PINDIR12W {
        _PINDIR12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir13(&mut self) -> _PINDIR13W {
        _PINDIR13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir14(&mut self) -> _PINDIR14W {
        _PINDIR14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir15(&mut self) -> _PINDIR15W {
        _PINDIR15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir16(&mut self) -> _PINDIR16W {
        _PINDIR16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir17(&mut self) -> _PINDIR17W {
        _PINDIR17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir18(&mut self) -> _PINDIR18W {
        _PINDIR18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir19(&mut self) -> _PINDIR19W {
        _PINDIR19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir20(&mut self) -> _PINDIR20W {
        _PINDIR20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir21(&mut self) -> _PINDIR21W {
        _PINDIR21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir22(&mut self) -> _PINDIR22W {
        _PINDIR22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir23(&mut self) -> _PINDIR23W {
        _PINDIR23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir24(&mut self) -> _PINDIR24W {
        _PINDIR24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir25(&mut self) -> _PINDIR25W {
        _PINDIR25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir26(&mut self) -> _PINDIR26W {
        _PINDIR26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir27(&mut self) -> _PINDIR27W {
        _PINDIR27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir28(&mut self) -> _PINDIR28W {
        _PINDIR28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir29(&mut self) -> _PINDIR29W {
        _PINDIR29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir30(&mut self) -> _PINDIR30W {
        _PINDIR30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir31(&mut self) -> _PINDIR31W {
        _PINDIR31W { w: self }
    }
}
