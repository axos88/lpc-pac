#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENF0 {
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
pub type P0_0EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_0EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_0EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_1EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_1EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_1EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_2EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_2EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_2EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_3EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_3EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_3EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_4EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_4EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_4EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_5EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_5EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_5EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_6EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_6EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_6EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_7EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_7EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_7EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_8EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_8EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_8EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_9EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_9EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_9EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_10EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_10EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_11EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_11EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_12EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_12EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_12EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_13EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_13EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_13EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_14EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_14EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_14EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_15EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_15EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_16EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_16EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_17EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_17EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_18EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_18EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_19EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_19EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_20EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_20EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_21EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_21EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_22EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_22EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_23EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_23EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_24EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_24EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_25EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_25EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_26EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_26EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_27EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_27EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_27EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_28EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_28EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_28EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_29EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_29EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_29EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_30EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_30EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_30EFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable falling edge interrupt for P0\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_0ef(&self) -> P0_0EF_R {
        P0_0EF_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P0\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_1ef(&self) -> P0_1EF_R {
        P0_1EF_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P0\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_2ef(&self) -> P0_2EF_R {
        P0_2EF_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P0\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_3ef(&self) -> P0_3EF_R {
        P0_3EF_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P0\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_4ef(&self) -> P0_4EF_R {
        P0_4EF_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P0\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_5ef(&self) -> P0_5EF_R {
        P0_5EF_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P0\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_6ef(&self) -> P0_6EF_R {
        P0_6EF_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P0\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_7ef(&self) -> P0_7EF_R {
        P0_7EF_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P0\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_8ef(&self) -> P0_8EF_R {
        P0_8EF_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P0\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_9ef(&self) -> P0_9EF_R {
        P0_9EF_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P0\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_10ef(&self) -> P0_10EF_R {
        P0_10EF_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P0\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_11ef(&self) -> P0_11EF_R {
        P0_11EF_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P0\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_12ef(&self) -> P0_12EF_R {
        P0_12EF_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P0\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_13ef(&self) -> P0_13EF_R {
        P0_13EF_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P0\\[14\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_14ef(&self) -> P0_14EF_R {
        P0_14EF_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P0\\[15\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_15ef(&self) -> P0_15EF_R {
        P0_15EF_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P0\\[16\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_16ef(&self) -> P0_16EF_R {
        P0_16EF_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P0\\[17\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_17ef(&self) -> P0_17EF_R {
        P0_17EF_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P0\\[18\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_18ef(&self) -> P0_18EF_R {
        P0_18EF_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P0\\[19\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_19ef(&self) -> P0_19EF_R {
        P0_19EF_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P0\\[20\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_20ef(&self) -> P0_20EF_R {
        P0_20EF_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P0\\[21\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_21ef(&self) -> P0_21EF_R {
        P0_21EF_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P0\\[22\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_22ef(&self) -> P0_22EF_R {
        P0_22EF_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P0\\[23\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_23ef(&self) -> P0_23EF_R {
        P0_23EF_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P0\\[24\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_24ef(&self) -> P0_24EF_R {
        P0_24EF_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P0\\[25\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_25ef(&self) -> P0_25EF_R {
        P0_25EF_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P0\\[26\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_26ef(&self) -> P0_26EF_R {
        P0_26EF_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P0\\[27\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_27ef(&self) -> P0_27EF_R {
        P0_27EF_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P0\\[28\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_28ef(&self) -> P0_28EF_R {
        P0_28EF_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P0\\[29\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_29ef(&self) -> P0_29EF_R {
        P0_29EF_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P0\\[30\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_30ef(&self) -> P0_30EF_R {
        P0_30EF_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable falling edge interrupt for P0\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_0ef(&mut self) -> _P0_0EFW {
        _P0_0EFW { w: self }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P0\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_1ef(&mut self) -> _P0_1EFW {
        _P0_1EFW { w: self }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P0\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_2ef(&mut self) -> _P0_2EFW {
        _P0_2EFW { w: self }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P0\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_3ef(&mut self) -> _P0_3EFW {
        _P0_3EFW { w: self }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P0\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_4ef(&mut self) -> _P0_4EFW {
        _P0_4EFW { w: self }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P0\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_5ef(&mut self) -> _P0_5EFW {
        _P0_5EFW { w: self }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P0\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_6ef(&mut self) -> _P0_6EFW {
        _P0_6EFW { w: self }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P0\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_7ef(&mut self) -> _P0_7EFW {
        _P0_7EFW { w: self }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P0\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_8ef(&mut self) -> _P0_8EFW {
        _P0_8EFW { w: self }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P0\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_9ef(&mut self) -> _P0_9EFW {
        _P0_9EFW { w: self }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P0\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_10ef(&mut self) -> _P0_10EFW {
        _P0_10EFW { w: self }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P0\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_11ef(&mut self) -> _P0_11EFW {
        _P0_11EFW { w: self }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P0\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_12ef(&mut self) -> _P0_12EFW {
        _P0_12EFW { w: self }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P0\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_13ef(&mut self) -> _P0_13EFW {
        _P0_13EFW { w: self }
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P0\\[14\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_14ef(&mut self) -> _P0_14EFW {
        _P0_14EFW { w: self }
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P0\\[15\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_15ef(&mut self) -> _P0_15EFW {
        _P0_15EFW { w: self }
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P0\\[16\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_16ef(&mut self) -> _P0_16EFW {
        _P0_16EFW { w: self }
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P0\\[17\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_17ef(&mut self) -> _P0_17EFW {
        _P0_17EFW { w: self }
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P0\\[18\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_18ef(&mut self) -> _P0_18EFW {
        _P0_18EFW { w: self }
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P0\\[19\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_19ef(&mut self) -> _P0_19EFW {
        _P0_19EFW { w: self }
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P0\\[20\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_20ef(&mut self) -> _P0_20EFW {
        _P0_20EFW { w: self }
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P0\\[21\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_21ef(&mut self) -> _P0_21EFW {
        _P0_21EFW { w: self }
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P0\\[22\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_22ef(&mut self) -> _P0_22EFW {
        _P0_22EFW { w: self }
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P0\\[23\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_23ef(&mut self) -> _P0_23EFW {
        _P0_23EFW { w: self }
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P0\\[24\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_24ef(&mut self) -> _P0_24EFW {
        _P0_24EFW { w: self }
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P0\\[25\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_25ef(&mut self) -> _P0_25EFW {
        _P0_25EFW { w: self }
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P0\\[26\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_26ef(&mut self) -> _P0_26EFW {
        _P0_26EFW { w: self }
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P0\\[27\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_27ef(&mut self) -> _P0_27EFW {
        _P0_27EFW { w: self }
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P0\\[28\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_28ef(&mut self) -> _P0_28EFW {
        _P0_28EFW { w: self }
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P0\\[29\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_29ef(&mut self) -> _P0_29EFW {
        _P0_29EFW { w: self }
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P0\\[30\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p0_30ef(&mut self) -> _P0_30EFW {
        _P0_30EFW { w: self }
    }
}
