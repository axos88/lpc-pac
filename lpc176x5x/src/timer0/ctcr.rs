#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTCR {
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
#[doc = "Possible values of the field `CTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMODER {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE,
}
impl crate::ToBits<u8> for CTMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CTMODER::TIMER_MODE_EVERY_RI => 0,
            CTMODER::RISING => 1,
            CTMODER::FALLING => 2,
            CTMODER::DUALEDGE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CTMODE_R = crate::FR<u8, CTMODER>;
impl CTMODE_R {
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline(always)]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        *self == CTMODER::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CTMODER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CTMODER::FALLING
    }
    #[doc = "Checks if the value of the field is `DUALEDGE`"]
    #[inline(always)]
    pub fn is_dualedge(&self) -> bool {
        *self == CTMODER::DUALEDGE
    }
}
#[doc = "Values that can be written to the field `CTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMODEW {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE,
}
impl CTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMODEW::TIMER_MODE_EVERY_RI => 0,
            CTMODEW::RISING => 1,
            CTMODEW::FALLING => 2,
            CTMODEW::DUALEDGE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTMODEW::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTMODEW::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CTMODEW::FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dualedge(self) -> &'a mut W {
        self.variant(CTMODEW::DUALEDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSELR {
    #[doc = "CAPn.0 for TIMERn"]
    CAPN_0_FOR_TIMERN,
    #[doc = "CAPn.1 for TIMERn"]
    CAPN_1_FOR_TIMERN,
}
impl crate::ToBits<u8> for CINSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CINSELR::CAPN_0_FOR_TIMERN => 0,
            CINSELR::CAPN_1_FOR_TIMERN => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CINSEL_R = crate::FR<u8, CINSELR>;
impl CINSEL_R {
    #[doc = "Checks if the value of the field is `CAPN_0_FOR_TIMERN`"]
    #[inline(always)]
    pub fn is_capn_0_for_timern(&self) -> bool {
        *self == CINSELR::CAPN_0_FOR_TIMERN
    }
    #[doc = "Checks if the value of the field is `CAPN_1_FOR_TIMERN`"]
    #[inline(always)]
    pub fn is_capn_1_for_timern(&self) -> bool {
        *self == CINSELR::CAPN_1_FOR_TIMERN
    }
}
#[doc = "Values that can be written to the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSELW {
    #[doc = "CAPn.0 for TIMERn"]
    CAPN_0_FOR_TIMERN,
    #[doc = "CAPn.1 for TIMERn"]
    CAPN_1_FOR_TIMERN,
}
impl CINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CINSELW::CAPN_0_FOR_TIMERN => 0,
            CINSELW::CAPN_1_FOR_TIMERN => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn capn_0_for_timern(self) -> &'a mut W {
        self.variant(CINSELW::CAPN_0_FOR_TIMERN)
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn capn_1_for_timern(self) -> &'a mut W {
        self.variant(CINSELW::CAPN_1_FOR_TIMERN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&mut self) -> _CTMODEW {
        _CTMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&mut self) -> _CINSELW {
        _CINSELW { w: self }
    }
}
