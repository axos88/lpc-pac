#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
#[doc = "Possible values of the field `CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CER {
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    THE_PWM_TIMER_COUNTE,
    #[doc = "The counters are disabled."]
    THE_COUNTERS_ARE_DIS,
}
impl crate::ToBits<bool> for CER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CER::THE_PWM_TIMER_COUNTE => true,
            CER::THE_COUNTERS_ARE_DIS => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CE_R = crate::FR<bool, CER>;
impl CE_R {
    #[doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`"]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == CER::THE_PWM_TIMER_COUNTE
    }
    #[doc = "Checks if the value of the field is `THE_COUNTERS_ARE_DIS`"]
    #[inline(always)]
    pub fn is_the_counters_are_dis(&self) -> bool {
        *self == CER::THE_COUNTERS_ARE_DIS
    }
}
#[doc = "Values that can be written to the field `CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEW {
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    THE_PWM_TIMER_COUNTE,
    #[doc = "The counters are disabled."]
    THE_COUNTERS_ARE_DIS,
}
impl CEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CEW::THE_PWM_TIMER_COUNTE => true,
            CEW::THE_COUNTERS_ARE_DIS => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CEW<'a> {
    w: &'a mut W,
}
impl<'a> _CEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut W {
        self.variant(CEW::THE_PWM_TIMER_COUNTE)
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn the_counters_are_dis(self) -> &'a mut W {
        self.variant(CEW::THE_COUNTERS_ARE_DIS)
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
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    THE_PWM_TIMER_COUNTE,
    #[doc = "Clear reset."]
    CLEAR_RESET_,
}
impl crate::ToBits<bool> for CRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CRR::THE_PWM_TIMER_COUNTE => true,
            CRR::CLEAR_RESET_ => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CR_R = crate::FR<bool, CRR>;
impl CR_R {
    #[doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`"]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == CRR::THE_PWM_TIMER_COUNTE
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET_`"]
    #[inline(always)]
    pub fn is_clear_reset_(&self) -> bool {
        *self == CRR::CLEAR_RESET_
    }
}
#[doc = "Values that can be written to the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRW {
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    THE_PWM_TIMER_COUNTE,
    #[doc = "Clear reset."]
    CLEAR_RESET_,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CRW::THE_PWM_TIMER_COUNTE => true,
            CRW::CLEAR_RESET_ => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut W {
        self.variant(CRW::THE_PWM_TIMER_COUNTE)
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn clear_reset_(self) -> &'a mut W {
        self.variant(CRW::CLEAR_RESET_)
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
#[doc = "Possible values of the field `PWMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENR {
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE_IS_ENABLED_,
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE_IS_ENABLE,
}
impl crate::ToBits<bool> for PWMENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMENR::PWM_MODE_IS_ENABLED_ => true,
            PWMENR::TIMER_MODE_IS_ENABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMEN_R = crate::FR<bool, PWMENR>;
impl PWMEN_R {
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMENR::PWM_MODE_IS_ENABLED_
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_IS_ENABLE`"]
    #[inline(always)]
    pub fn is_timer_mode_is_enable(&self) -> bool {
        *self == PWMENR::TIMER_MODE_IS_ENABLE
    }
}
#[doc = "Values that can be written to the field `PWMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENW {
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE_IS_ENABLED_,
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE_IS_ENABLE,
}
impl PWMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENW::PWM_MODE_IS_ENABLED_ => true,
            PWMENW::TIMER_MODE_IS_ENABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMENW::PWM_MODE_IS_ENABLED_)
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn timer_mode_is_enable(self) -> &'a mut W {
        self.variant(PWMENW::TIMER_MODE_IS_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE_PWM0_IS_,
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE_THE_,
}
impl crate::ToBits<bool> for MDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MDISR::MASTER_USE_PWM0_IS_ => true,
            MDISR::INDIVIDUAL_USE_THE_ => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MDIS_R = crate::FR<bool, MDISR>;
impl MDIS_R {
    #[doc = "Checks if the value of the field is `MASTER_USE_PWM0_IS_`"]
    #[inline(always)]
    pub fn is_master_use_pwm0_is_(&self) -> bool {
        *self == MDISR::MASTER_USE_PWM0_IS_
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_USE_THE_`"]
    #[inline(always)]
    pub fn is_individual_use_the_(&self) -> bool {
        *self == MDISR::INDIVIDUAL_USE_THE_
    }
}
#[doc = "Values that can be written to the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISW {
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE_PWM0_IS_,
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE_THE_,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::MASTER_USE_PWM0_IS_ => true,
            MDISW::INDIVIDUAL_USE_THE_ => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn master_use_pwm0_is_(self) -> &'a mut W {
        self.variant(MDISW::MASTER_USE_PWM0_IS_)
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn individual_use_the_(self) -> &'a mut W {
        self.variant(MDISW::INDIVIDUAL_USE_THE_)
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
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&self) -> PWMEN_R {
        PWMEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&mut self) -> _CEW {
        _CEW { w: self }
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&mut self) -> _PWMENW {
        _PWMENW { w: self }
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
}
