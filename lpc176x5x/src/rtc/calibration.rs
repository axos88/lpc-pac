#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALIBRATION {
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
pub type CALVAL_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _CALVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CALVALW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Possible values of the field `CALDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALDIRR {
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION,
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_,
}
impl crate::ToBits<bool> for CALDIRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CALDIRR::BACKWARD_CALIBRATION => true,
            CALDIRR::FORWARD_CALIBRATION_ => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CALDIR_R = crate::FR<bool, CALDIRR>;
impl CALDIR_R {
    #[doc = "Checks if the value of the field is `BACKWARD_CALIBRATION`"]
    #[inline(always)]
    pub fn is_backward_calibration(&self) -> bool {
        *self == CALDIRR::BACKWARD_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `FORWARD_CALIBRATION_`"]
    #[inline(always)]
    pub fn is_forward_calibration_(&self) -> bool {
        *self == CALDIRR::FORWARD_CALIBRATION_
    }
}
#[doc = "Values that can be written to the field `CALDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALDIRW {
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION,
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_,
}
impl CALDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CALDIRW::BACKWARD_CALIBRATION => true,
            CALDIRW::FORWARD_CALIBRATION_ => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CALDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CALDIRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn backward_calibration(self) -> &'a mut W {
        self.variant(CALDIRW::BACKWARD_CALIBRATION)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn forward_calibration_(self) -> &'a mut W {
        self.variant(CALDIRW::FORWARD_CALIBRATION_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits() & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&mut self) -> _CALVALW {
        _CALVALW { w: self }
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&mut self) -> _CALDIRW {
        _CALDIRW { w: self }
    }
}
