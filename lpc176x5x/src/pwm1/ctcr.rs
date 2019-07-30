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
#[doc = "Possible values of the field `MOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODR {
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TIMER_MODE_THE_TC_I,
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RISING_EDGE_COUNTER_,
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FALLING_EDGE_COUNTER,
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DUAL_EDGE_COUNTER_MO,
}
impl crate::ToBits<u8> for MODR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODR::TIMER_MODE_THE_TC_I => 0,
            MODR::RISING_EDGE_COUNTER_ => 1,
            MODR::FALLING_EDGE_COUNTER => 2,
            MODR::DUAL_EDGE_COUNTER_MO => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MOD_R = crate::FR<u8, MODR>;
impl MOD_R {
    #[doc = "Checks if the value of the field is `TIMER_MODE_THE_TC_I`"]
    #[inline(always)]
    pub fn is_timer_mode_the_tc_i(&self) -> bool {
        *self == MODR::TIMER_MODE_THE_TC_I
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_COUNTER_`"]
    #[inline(always)]
    pub fn is_rising_edge_counter_(&self) -> bool {
        *self == MODR::RISING_EDGE_COUNTER_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_falling_edge_counter(&self) -> bool {
        *self == MODR::FALLING_EDGE_COUNTER
    }
    #[doc = "Checks if the value of the field is `DUAL_EDGE_COUNTER_MO`"]
    #[inline(always)]
    pub fn is_dual_edge_counter_mo(&self) -> bool {
        *self == MODR::DUAL_EDGE_COUNTER_MO
    }
}
#[doc = "Values that can be written to the field `MOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODW {
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TIMER_MODE_THE_TC_I,
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RISING_EDGE_COUNTER_,
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FALLING_EDGE_COUNTER,
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DUAL_EDGE_COUNTER_MO,
}
impl MODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODW::TIMER_MODE_THE_TC_I => 0,
            MODW::RISING_EDGE_COUNTER_ => 1,
            MODW::FALLING_EDGE_COUNTER => 2,
            MODW::DUAL_EDGE_COUNTER_MO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODW<'a> {
    w: &'a mut W,
}
impl<'a> _MODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer_mode_the_tc_i(self) -> &'a mut W {
        self.variant(MODW::TIMER_MODE_THE_TC_I)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising_edge_counter_(self) -> &'a mut W {
        self.variant(MODW::RISING_EDGE_COUNTER_)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling_edge_counter(self) -> &'a mut W {
        self.variant(MODW::FALLING_EDGE_COUNTER)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dual_edge_counter_mo(self) -> &'a mut W {
        self.variant(MODW::DUAL_EDGE_COUNTER_MO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISR {
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    FOR_PWM0_00_EQ_PWM0_,
}
impl crate::ToBits<u8> for CISR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CISR::FOR_PWM0_00_EQ_PWM0_ => 0,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CIS_R = crate::FR<u8, CISR>;
impl CIS_R {
    #[doc = "Checks if the value of the field is `FOR_PWM0_00_EQ_PWM0_`"]
    #[inline(always)]
    pub fn is_for_pwm0_00_eq_pwm0_(&self) -> bool {
        *self == CISR::FOR_PWM0_00_EQ_PWM0_
    }
}
#[doc = "Values that can be written to the field `CIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISW {
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    FOR_PWM0_00_EQ_PWM0_,
}
impl CISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CISW::FOR_PWM0_00_EQ_PWM0_ => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CISW<'a> {
    w: &'a mut W,
}
impl<'a> _CISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn for_pwm0_00_eq_pwm0_(self) -> &'a mut W {
        self.variant(CISW::FOR_PWM0_00_EQ_PWM0_)
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
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&mut self) -> _MODW {
        _MODW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&mut self) -> _CISW {
        _CISW { w: self }
    }
}
