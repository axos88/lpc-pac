#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCS {
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
#[doc = "Possible values of the field `OSCRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRANGER {
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW,
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH,
}
impl crate::ToBits<bool> for OSCRANGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OSCRANGER::LOW => false,
            OSCRANGER::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSCRANGE_R = crate::FR<bool, OSCRANGER>;
impl OSCRANGE_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OSCRANGER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OSCRANGER::HIGH
    }
}
#[doc = "Values that can be written to the field `OSCRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRANGEW {
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW,
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH,
}
impl OSCRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCRANGEW::LOW => false,
            OSCRANGEW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSCRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCRANGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCRANGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OSCRANGEW::LOW)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OSCRANGEW::HIGH)
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
#[doc = "Possible values of the field `OSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCENR {
    #[doc = "Disabled. The main oscillator is disabled."]
    DISABLED,
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl crate::ToBits<bool> for OSCENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OSCENR::DISABLED => false,
            OSCENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSCEN_R = crate::FR<bool, OSCENR>;
impl OSCEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSCENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `OSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCENW {
    #[doc = "Disabled. The main oscillator is disabled."]
    DISABLED,
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl OSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCENW::DISABLED => false,
            OSCENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCENW::DISABLED)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSCENW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `OSCSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTATR {
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl crate::ToBits<bool> for OSCSTATR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OSCSTATR::NOT_READY => false,
            OSCSTATR::READY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSCSTAT_R = crate::FR<bool, OSCSTATR>;
impl OSCSTAT_R {
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTATR::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTATR::READY
    }
}
#[doc = "Values that can be written to the field `OSCSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTATW {
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl OSCSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSTATW::NOT_READY => false,
            OSCSTATW::READY => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSCSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSTATW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(OSCSTATW::NOT_READY)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(OSCSTATW::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&self) -> OSCRANGE_R {
        OSCRANGE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OSCSTAT_R {
        OSCSTAT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&mut self) -> _OSCRANGEW {
        _OSCRANGEW { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> _OSCENW {
        _OSCENW { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> _OSCSTATW {
        _OSCSTATW { w: self }
    }
}
