#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKSRCSEL {
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
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    SELECTS_THE_RTC_OSCI,
}
impl crate::ToBits<u8> for CLKSRCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKSRCR::SELECTS_THE_INTERNAL => 0,
            CLKSRCR::SELECTS_THE_MAIN_OSC => 1,
            CLKSRCR::SELECTS_THE_RTC_OSCI => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKSRC_R = crate::FR<u8, CLKSRCR>;
impl CLKSRC_R {
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_RTC_OSCI
    }
}
#[doc = "Values that can be written to the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCW {
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    SELECTS_THE_RTC_OSCI,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSRCW::SELECTS_THE_INTERNAL => 0,
            CLKSRCW::SELECTS_THE_MAIN_OSC => 1,
            CLKSRCW::SELECTS_THE_RTC_OSCI => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
}
