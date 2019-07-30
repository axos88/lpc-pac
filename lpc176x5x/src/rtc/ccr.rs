#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "The time counters are enabled."]
    THE_TIME_COUNTERS_ARE_ENABLED,
    #[doc = "The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_ARE_DISABLED,
}
impl crate::ToBits<bool> for CLKENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CLKENR::THE_TIME_COUNTERS_ARE_ENABLED => true,
            CLKENR::THE_TIME_COUNTERS_ARE_DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKEN_R = crate::FR<bool, CLKENR>;
impl CLKEN_R {
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_ARE_ENABLED`"]
    #[inline(always)]
    pub fn is_the_time_counters_are_enabled(&self) -> bool {
        *self == CLKENR::THE_TIME_COUNTERS_ARE_ENABLED
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_ARE_DISABLED`"]
    #[inline(always)]
    pub fn is_the_time_counters_are_disabled(&self) -> bool {
        *self == CLKENR::THE_TIME_COUNTERS_ARE_DISABLED
    }
}
#[doc = "Values that can be written to the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENW {
    #[doc = "The time counters are enabled."]
    THE_TIME_COUNTERS_ARE_ENABLED,
    #[doc = "The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_ARE_DISABLED,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::THE_TIME_COUNTERS_ARE_ENABLED => true,
            CLKENW::THE_TIME_COUNTERS_ARE_DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn the_time_counters_are_enabled(self) -> &'a mut W {
        self.variant(CLKENW::THE_TIME_COUNTERS_ARE_ENABLED)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn the_time_counters_are_disabled(self) -> &'a mut W {
        self.variant(CLKENW::THE_TIME_COUNTERS_ARE_DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `CTCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCRSTR {
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET,
    #[doc = "No effect."]
    NO_EFFECT_,
}
impl crate::ToBits<bool> for CTCRSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CTCRSTR::RESET => true,
            CTCRSTR::NO_EFFECT_ => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CTCRST_R = crate::FR<bool, CTCRSTR>;
impl CTCRST_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTCRSTR::RESET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline(always)]
    pub fn is_no_effect_(&self) -> bool {
        *self == CTCRSTR::NO_EFFECT_
    }
}
#[doc = "Values that can be written to the field `CTCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCRSTW {
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET,
    #[doc = "No effect."]
    NO_EFFECT_,
}
impl CTCRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCRSTW::RESET => true,
            CTCRSTW::NO_EFFECT_ => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CTCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTCRSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTCRSTW::RESET)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(CTCRSTW::NO_EFFECT_)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `CCALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCALENR {
    #[doc = "The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUNTER_IS_DISABLED,
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUNTER_IS_ENABLED,
}
impl crate::ToBits<bool> for CCALENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCALENR::THE_CALIBRATION_COUNTER_IS_DISABLED => true,
            CCALENR::THE_CALIBRATION_COUNTER_IS_ENABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCALEN_R = crate::FR<bool, CCALENR>;
impl CCALEN_R {
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUNTER_IS_DISABLED`"]
    #[inline(always)]
    pub fn is_the_calibration_counter_is_disabled(&self) -> bool {
        *self == CCALENR::THE_CALIBRATION_COUNTER_IS_DISABLED
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUNTER_IS_ENABLED`"]
    #[inline(always)]
    pub fn is_the_calibration_counter_is_enabled(&self) -> bool {
        *self == CCALENR::THE_CALIBRATION_COUNTER_IS_ENABLED
    }
}
#[doc = "Values that can be written to the field `CCALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCALENW {
    #[doc = "The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUNTER_IS_DISABLED,
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUNTER_IS_ENABLED,
}
impl CCALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCALENW::THE_CALIBRATION_COUNTER_IS_DISABLED => true,
            CCALENW::THE_CALIBRATION_COUNTER_IS_ENABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCALENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCALENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCALENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn the_calibration_counter_is_disabled(self) -> &'a mut W {
        self.variant(CCALENW::THE_CALIBRATION_COUNTER_IS_DISABLED)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn the_calibration_counter_is_enabled(self) -> &'a mut W {
        self.variant(CCALENW::THE_CALIBRATION_COUNTER_IS_ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&self) -> CCALEN_R {
        CCALEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> _CTCRSTW {
        _CTCRSTW { w: self }
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&mut self) -> _CCALENW {
        _CCALENW { w: self }
    }
}
