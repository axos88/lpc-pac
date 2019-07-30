#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REEP {
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
        0x03
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type EPR0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR2W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR3W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR4W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR5W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR6W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR7W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR8W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR9W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR10W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR11W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR12W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR13W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR14W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR15W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR16W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR17W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR18W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR19W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR20W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR21W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR22W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR23W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR24W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR25W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR26W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR27W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR28W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR29W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR30W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type EPR31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPR31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPR31W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&self) -> EPR0_R {
        EPR0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&self) -> EPR1_R {
        EPR1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&self) -> EPR2_R {
        EPR2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&self) -> EPR3_R {
        EPR3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&self) -> EPR4_R {
        EPR4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&self) -> EPR5_R {
        EPR5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&self) -> EPR6_R {
        EPR6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&self) -> EPR7_R {
        EPR7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&self) -> EPR8_R {
        EPR8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&self) -> EPR9_R {
        EPR9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&self) -> EPR10_R {
        EPR10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&self) -> EPR11_R {
        EPR11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&self) -> EPR12_R {
        EPR12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&self) -> EPR13_R {
        EPR13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&self) -> EPR14_R {
        EPR14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&self) -> EPR15_R {
        EPR15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&self) -> EPR16_R {
        EPR16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&self) -> EPR17_R {
        EPR17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&self) -> EPR18_R {
        EPR18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&self) -> EPR19_R {
        EPR19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&self) -> EPR20_R {
        EPR20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&self) -> EPR21_R {
        EPR21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&self) -> EPR22_R {
        EPR22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&self) -> EPR23_R {
        EPR23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&self) -> EPR24_R {
        EPR24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&self) -> EPR25_R {
        EPR25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&self) -> EPR26_R {
        EPR26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&self) -> EPR27_R {
        EPR27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&self) -> EPR28_R {
        EPR28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&self) -> EPR29_R {
        EPR29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&self) -> EPR30_R {
        EPR30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&self) -> EPR31_R {
        EPR31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&mut self) -> _EPR0W {
        _EPR0W { w: self }
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&mut self) -> _EPR1W {
        _EPR1W { w: self }
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&mut self) -> _EPR2W {
        _EPR2W { w: self }
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&mut self) -> _EPR3W {
        _EPR3W { w: self }
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&mut self) -> _EPR4W {
        _EPR4W { w: self }
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&mut self) -> _EPR5W {
        _EPR5W { w: self }
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&mut self) -> _EPR6W {
        _EPR6W { w: self }
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&mut self) -> _EPR7W {
        _EPR7W { w: self }
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&mut self) -> _EPR8W {
        _EPR8W { w: self }
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&mut self) -> _EPR9W {
        _EPR9W { w: self }
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&mut self) -> _EPR10W {
        _EPR10W { w: self }
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&mut self) -> _EPR11W {
        _EPR11W { w: self }
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&mut self) -> _EPR12W {
        _EPR12W { w: self }
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&mut self) -> _EPR13W {
        _EPR13W { w: self }
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&mut self) -> _EPR14W {
        _EPR14W { w: self }
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&mut self) -> _EPR15W {
        _EPR15W { w: self }
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&mut self) -> _EPR16W {
        _EPR16W { w: self }
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&mut self) -> _EPR17W {
        _EPR17W { w: self }
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&mut self) -> _EPR18W {
        _EPR18W { w: self }
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&mut self) -> _EPR19W {
        _EPR19W { w: self }
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&mut self) -> _EPR20W {
        _EPR20W { w: self }
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&mut self) -> _EPR21W {
        _EPR21W { w: self }
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&mut self) -> _EPR22W {
        _EPR22W { w: self }
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&mut self) -> _EPR23W {
        _EPR23W { w: self }
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&mut self) -> _EPR24W {
        _EPR24W { w: self }
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&mut self) -> _EPR25W {
        _EPR25W { w: self }
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&mut self) -> _EPR26W {
        _EPR26W { w: self }
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&mut self) -> _EPR27W {
        _EPR27W { w: self }
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&mut self) -> _EPR28W {
        _EPR28W { w: self }
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&mut self) -> _EPR29W {
        _EPR29W { w: self }
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&mut self) -> _EPR30W {
        _EPR30W { w: self }
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&mut self) -> _EPR31W {
        _EPR31W { w: self }
    }
}
