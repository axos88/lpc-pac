#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AMR {
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
pub type AMRSEC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRSECW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRSECW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRMIN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRMINW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRMINW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRHOUR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRHOURW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRHOURW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRDOM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRDOMW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRDOMW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRDOW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRDOWW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRDOWW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRDOY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRDOYW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRDOYW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRMON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRMONW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRMONW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub type AMRYEAR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMRYEARW<'a> {
    w: &'a mut W,
}
impl<'a> _AMRYEARW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&self) -> AMRSEC_R {
        AMRSEC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&self) -> AMRMIN_R {
        AMRMIN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&self) -> AMRHOUR_R {
        AMRHOUR_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&self) -> AMRDOM_R {
        AMRDOM_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&self) -> AMRDOW_R {
        AMRDOW_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&self) -> AMRDOY_R {
        AMRDOY_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&self) -> AMRMON_R {
        AMRMON_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&self) -> AMRYEAR_R {
        AMRYEAR_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&mut self) -> _AMRSECW {
        _AMRSECW { w: self }
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&mut self) -> _AMRMINW {
        _AMRMINW { w: self }
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&mut self) -> _AMRHOURW {
        _AMRHOURW { w: self }
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&mut self) -> _AMRDOMW {
        _AMRDOMW { w: self }
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&mut self) -> _AMRDOWW {
        _AMRDOWW { w: self }
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&mut self) -> _AMRDOYW {
        _AMRDOYW { w: self }
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&mut self) -> _AMRMONW {
        _AMRMONW { w: self }
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&mut self) -> _AMRYEARW {
        _AMRYEARW { w: self }
    }
}
