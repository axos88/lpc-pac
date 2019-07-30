#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTCFG {
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
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    SELECTS_THE_CPU_CLOC,
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the USB clock as the CLKOUT source."]
    SELECTS_THE_USB_CLOC,
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    SELECTS_THE_RTC_OSCI,
}
impl crate::ToBits<u8> for CLKOUTSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::SELECTS_THE_CPU_CLOC => 0,
            CLKOUTSELR::SELECTS_THE_MAIN_OSC => 1,
            CLKOUTSELR::SELECTS_THE_INTERNAL => 2,
            CLKOUTSELR::SELECTS_THE_USB_CLOC => 3,
            CLKOUTSELR::SELECTS_THE_RTC_OSCI => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKOUTSEL_R = crate::FR<u8, CLKOUTSELR>;
impl CLKOUTSEL_R {
    #[doc = "Checks if the value of the field is `SELECTS_THE_CPU_CLOC`"]
    #[inline(always)]
    pub fn is_selects_the_cpu_cloc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_CPU_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_USB_CLOC`"]
    #[inline(always)]
    pub fn is_selects_the_usb_cloc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_USB_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_RTC_OSCI
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELW {
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    SELECTS_THE_CPU_CLOC,
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the USB clock as the CLKOUT source."]
    SELECTS_THE_USB_CLOC,
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    SELECTS_THE_RTC_OSCI,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::SELECTS_THE_CPU_CLOC => 0,
            CLKOUTSELW::SELECTS_THE_MAIN_OSC => 1,
            CLKOUTSELW::SELECTS_THE_INTERNAL => 2,
            CLKOUTSELW::SELECTS_THE_USB_CLOC => 3,
            CLKOUTSELW::SELECTS_THE_RTC_OSCI => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_cpu_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_CPU_CLOC)
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_usb_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_USB_CLOC)
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLKOUTDIV_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CLKOUTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLKOUT_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLKOUT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT_ENW<'a> {
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
pub type CLKOUT_ACT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLKOUT_ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT_ACTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&self) -> CLKOUT_EN_R {
        CLKOUT_EN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&self) -> CLKOUT_ACT_R {
        CLKOUT_ACT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> _CLKOUTDIVW {
        _CLKOUTDIVW { w: self }
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&mut self) -> _CLKOUT_ENW {
        _CLKOUT_ENW { w: self }
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&mut self) -> _CLKOUT_ACTW {
        _CLKOUT_ACTW { w: self }
    }
}
