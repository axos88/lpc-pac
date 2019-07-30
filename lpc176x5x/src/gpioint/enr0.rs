#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENR0 {
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
pub type P0_0ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_0ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_0ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_1ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_1ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_1ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_2ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_2ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_2ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_3ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_3ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_3ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_4ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_4ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_4ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_5ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_5ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_5ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_6ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_6ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_6ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_7ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_7ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_7ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_8ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_8ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_8ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_9ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_9ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_9ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_10ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_10ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_11ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_11ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_12ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_12ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_12ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_13ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_13ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_13ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_14ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_14ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_14ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_15ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_15ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_16ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_16ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_17ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_17ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_18ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_18ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_19ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_19ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_20ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_20ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_21ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_21ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_22ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_22ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_23ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_23ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_24ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_24ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_25ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_25ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_26ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_26ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_27ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_27ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_27ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_28ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_28ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_28ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_29ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_29ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_29ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type P0_30ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P0_30ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_30ERW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P0\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_0er(&self) -> P0_0ER_R {
        P0_0ER_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P0\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_1er(&self) -> P0_1ER_R {
        P0_1ER_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P0\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_2er(&self) -> P0_2ER_R {
        P0_2ER_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P0\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_3er(&self) -> P0_3ER_R {
        P0_3ER_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P0\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_4er(&self) -> P0_4ER_R {
        P0_4ER_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P0\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_5er(&self) -> P0_5ER_R {
        P0_5ER_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P0\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_6er(&self) -> P0_6ER_R {
        P0_6ER_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P0\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_7er(&self) -> P0_7ER_R {
        P0_7ER_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P0\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_8er(&self) -> P0_8ER_R {
        P0_8ER_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P0\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_9er(&self) -> P0_9ER_R {
        P0_9ER_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P0\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_10er(&self) -> P0_10ER_R {
        P0_10ER_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P0\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_11er(&self) -> P0_11ER_R {
        P0_11ER_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P0\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_12er(&self) -> P0_12ER_R {
        P0_12ER_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P0\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_13er(&self) -> P0_13ER_R {
        P0_13ER_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P0\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_14er(&self) -> P0_14ER_R {
        P0_14ER_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P0\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_15er(&self) -> P0_15ER_R {
        P0_15ER_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P0\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_16er(&self) -> P0_16ER_R {
        P0_16ER_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P0\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_17er(&self) -> P0_17ER_R {
        P0_17ER_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P0\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_18er(&self) -> P0_18ER_R {
        P0_18ER_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P0\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_19er(&self) -> P0_19ER_R {
        P0_19ER_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P0\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_20er(&self) -> P0_20ER_R {
        P0_20ER_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P0\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_21er(&self) -> P0_21ER_R {
        P0_21ER_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P0\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_22er(&self) -> P0_22ER_R {
        P0_22ER_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P0\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_23er(&self) -> P0_23ER_R {
        P0_23ER_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P0\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_24er(&self) -> P0_24ER_R {
        P0_24ER_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P0\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_25er(&self) -> P0_25ER_R {
        P0_25ER_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P0\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_26er(&self) -> P0_26ER_R {
        P0_26ER_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P0\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_27er(&self) -> P0_27ER_R {
        P0_27ER_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P0\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_28er(&self) -> P0_28ER_R {
        P0_28ER_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P0\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_29er(&self) -> P0_29ER_R {
        P0_29ER_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P0\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_30er(&self) -> P0_30ER_R {
        P0_30ER_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable rising edge interrupt for P0\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_0er(&mut self) -> _P0_0ERW {
        _P0_0ERW { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P0\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_1er(&mut self) -> _P0_1ERW {
        _P0_1ERW { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P0\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_2er(&mut self) -> _P0_2ERW {
        _P0_2ERW { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P0\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_3er(&mut self) -> _P0_3ERW {
        _P0_3ERW { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P0\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_4er(&mut self) -> _P0_4ERW {
        _P0_4ERW { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P0\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_5er(&mut self) -> _P0_5ERW {
        _P0_5ERW { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P0\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_6er(&mut self) -> _P0_6ERW {
        _P0_6ERW { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P0\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_7er(&mut self) -> _P0_7ERW {
        _P0_7ERW { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P0\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_8er(&mut self) -> _P0_8ERW {
        _P0_8ERW { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P0\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_9er(&mut self) -> _P0_9ERW {
        _P0_9ERW { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P0\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_10er(&mut self) -> _P0_10ERW {
        _P0_10ERW { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P0\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_11er(&mut self) -> _P0_11ERW {
        _P0_11ERW { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P0\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_12er(&mut self) -> _P0_12ERW {
        _P0_12ERW { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P0\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_13er(&mut self) -> _P0_13ERW {
        _P0_13ERW { w: self }
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P0\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_14er(&mut self) -> _P0_14ERW {
        _P0_14ERW { w: self }
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P0\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_15er(&mut self) -> _P0_15ERW {
        _P0_15ERW { w: self }
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P0\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_16er(&mut self) -> _P0_16ERW {
        _P0_16ERW { w: self }
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P0\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_17er(&mut self) -> _P0_17ERW {
        _P0_17ERW { w: self }
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P0\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_18er(&mut self) -> _P0_18ERW {
        _P0_18ERW { w: self }
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P0\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_19er(&mut self) -> _P0_19ERW {
        _P0_19ERW { w: self }
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P0\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_20er(&mut self) -> _P0_20ERW {
        _P0_20ERW { w: self }
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P0\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_21er(&mut self) -> _P0_21ERW {
        _P0_21ERW { w: self }
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P0\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_22er(&mut self) -> _P0_22ERW {
        _P0_22ERW { w: self }
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P0\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_23er(&mut self) -> _P0_23ERW {
        _P0_23ERW { w: self }
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P0\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_24er(&mut self) -> _P0_24ERW {
        _P0_24ERW { w: self }
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P0\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_25er(&mut self) -> _P0_25ERW {
        _P0_25ERW { w: self }
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P0\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_26er(&mut self) -> _P0_26ERW {
        _P0_26ERW { w: self }
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P0\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_27er(&mut self) -> _P0_27ERW {
        _P0_27ERW { w: self }
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P0\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_28er(&mut self) -> _P0_28ERW {
        _P0_28ERW { w: self }
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P0\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_29er(&mut self) -> _P0_29ERW {
        _P0_29ERW { w: self }
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P0\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p0_30er(&mut self) -> _P0_30ERW {
        _P0_30ERW { w: self }
    }
}
