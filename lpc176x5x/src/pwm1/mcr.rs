#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `PWMMR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR0,
}
impl crate::ToBits<bool> for PWMMR0IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR0IR::DISABLED_ => false,
            PWMMR0IR::INTERRUPT_ON_PWMMR0 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR0I_R = crate::FR<bool, PWMMR0IR>;
impl PWMMR0I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR0IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR0`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr0(&self) -> bool {
        *self == PWMMR0IR::INTERRUPT_ON_PWMMR0
    }
}
#[doc = "Values that can be written to the field `PWMMR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR0,
}
impl PWMMR0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0IW::DISABLED_ => false,
            PWMMR0IW::INTERRUPT_ON_PWMMR0 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR0IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR0IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr0(self) -> &'a mut W {
        self.variant(PWMMR0IW::INTERRUPT_ON_PWMMR0)
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
#[doc = "Possible values of the field `PWMMR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET_ON_PWMMR0_THE,
}
impl crate::ToBits<bool> for PWMMR0RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR0RR::DISABLED_ => false,
            PWMMR0RR::RESET_ON_PWMMR0_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR0R_R = crate::FR<bool, PWMMR0RR>;
impl PWMMR0R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR0RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR0_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr0_the(&self) -> bool {
        *self == PWMMR0RR::RESET_ON_PWMMR0_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET_ON_PWMMR0_THE,
}
impl PWMMR0RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0RW::DISABLED_ => false,
            PWMMR0RW::RESET_ON_PWMMR0_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR0RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR0RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr0_the(self) -> &'a mut W {
        self.variant(PWMMR0RW::RESET_ON_PWMMR0_THE)
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
#[doc = "Possible values of the field `PWMMR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR0_THE_,
}
impl crate::ToBits<bool> for PWMMR0SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR0SR::DISABLED => false,
            PWMMR0SR::STOP_ON_PWMMR0_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR0S_R = crate::FR<bool, PWMMR0SR>;
impl PWMMR0S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR0_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr0_the_(&self) -> bool {
        *self == PWMMR0SR::STOP_ON_PWMMR0_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR0_THE_,
}
impl PWMMR0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0SW::DISABLED => false,
            PWMMR0SW::STOP_ON_PWMMR0_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR0SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR0SW::DISABLED)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr0_the_(self) -> &'a mut W {
        self.variant(PWMMR0SW::STOP_ON_PWMMR0_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR1,
}
impl crate::ToBits<bool> for PWMMR1IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR1IR::DISABLED_ => false,
            PWMMR1IR::INTERRUPT_ON_PWMMR1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR1I_R = crate::FR<bool, PWMMR1IR>;
impl PWMMR1I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR1IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR1`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr1(&self) -> bool {
        *self == PWMMR1IR::INTERRUPT_ON_PWMMR1
    }
}
#[doc = "Values that can be written to the field `PWMMR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR1,
}
impl PWMMR1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1IW::DISABLED_ => false,
            PWMMR1IW::INTERRUPT_ON_PWMMR1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR1IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR1IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr1(self) -> &'a mut W {
        self.variant(PWMMR1IW::INTERRUPT_ON_PWMMR1)
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
#[doc = "Possible values of the field `PWMMR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET_ON_PWMMR1_THE,
}
impl crate::ToBits<bool> for PWMMR1RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR1RR::DISABLED_ => false,
            PWMMR1RR::RESET_ON_PWMMR1_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR1R_R = crate::FR<bool, PWMMR1RR>;
impl PWMMR1R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR1RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR1_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr1_the(&self) -> bool {
        *self == PWMMR1RR::RESET_ON_PWMMR1_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET_ON_PWMMR1_THE,
}
impl PWMMR1RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1RW::DISABLED_ => false,
            PWMMR1RW::RESET_ON_PWMMR1_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR1RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR1RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr1_the(self) -> &'a mut W {
        self.variant(PWMMR1RW::RESET_ON_PWMMR1_THE)
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
#[doc = "Possible values of the field `PWMMR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP_ON_PWMMR1_THE_,
}
impl crate::ToBits<bool> for PWMMR1SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR1SR::DISABLED => false,
            PWMMR1SR::STOP_ON_PWMMR1_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR1S_R = crate::FR<bool, PWMMR1SR>;
impl PWMMR1S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR1_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr1_the_(&self) -> bool {
        *self == PWMMR1SR::STOP_ON_PWMMR1_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP_ON_PWMMR1_THE_,
}
impl PWMMR1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1SW::DISABLED => false,
            PWMMR1SW::STOP_ON_PWMMR1_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR1SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR1SW::DISABLED)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr1_the_(self) -> &'a mut W {
        self.variant(PWMMR1SW::STOP_ON_PWMMR1_THE_)
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
#[doc = "Possible values of the field `PWMMR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR2,
}
impl crate::ToBits<bool> for PWMMR2IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR2IR::DISABLED_ => false,
            PWMMR2IR::INTERRUPT_ON_PWMMR2 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR2I_R = crate::FR<bool, PWMMR2IR>;
impl PWMMR2I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR2IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR2`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr2(&self) -> bool {
        *self == PWMMR2IR::INTERRUPT_ON_PWMMR2
    }
}
#[doc = "Values that can be written to the field `PWMMR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR2,
}
impl PWMMR2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2IW::DISABLED_ => false,
            PWMMR2IW::INTERRUPT_ON_PWMMR2 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR2IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR2IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr2(self) -> &'a mut W {
        self.variant(PWMMR2IW::INTERRUPT_ON_PWMMR2)
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
#[doc = "Possible values of the field `PWMMR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET_ON_PWMMR2_THE,
}
impl crate::ToBits<bool> for PWMMR2RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR2RR::DISABLED_ => false,
            PWMMR2RR::RESET_ON_PWMMR2_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR2R_R = crate::FR<bool, PWMMR2RR>;
impl PWMMR2R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR2RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR2_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr2_the(&self) -> bool {
        *self == PWMMR2RR::RESET_ON_PWMMR2_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET_ON_PWMMR2_THE,
}
impl PWMMR2RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2RW::DISABLED_ => false,
            PWMMR2RW::RESET_ON_PWMMR2_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR2RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr2_the(self) -> &'a mut W {
        self.variant(PWMMR2RW::RESET_ON_PWMMR2_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR2_THE_,
}
impl crate::ToBits<bool> for PWMMR2SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR2SR::DISABLED => false,
            PWMMR2SR::STOP_ON_PWMMR2_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR2S_R = crate::FR<bool, PWMMR2SR>;
impl PWMMR2S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR2_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr2_the_(&self) -> bool {
        *self == PWMMR2SR::STOP_ON_PWMMR2_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR2_THE_,
}
impl PWMMR2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2SW::DISABLED => false,
            PWMMR2SW::STOP_ON_PWMMR2_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR2SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR2SW::DISABLED)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr2_the_(self) -> &'a mut W {
        self.variant(PWMMR2SW::STOP_ON_PWMMR2_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR3,
}
impl crate::ToBits<bool> for PWMMR3IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR3IR::DISABLED_ => false,
            PWMMR3IR::INTERRUPT_ON_PWMMR3 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR3I_R = crate::FR<bool, PWMMR3IR>;
impl PWMMR3I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR3IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR3`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr3(&self) -> bool {
        *self == PWMMR3IR::INTERRUPT_ON_PWMMR3
    }
}
#[doc = "Values that can be written to the field `PWMMR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR3,
}
impl PWMMR3IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3IW::DISABLED_ => false,
            PWMMR3IW::INTERRUPT_ON_PWMMR3 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR3IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR3IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr3(self) -> &'a mut W {
        self.variant(PWMMR3IW::INTERRUPT_ON_PWMMR3)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET_ON_PWMMR3_THE,
}
impl crate::ToBits<bool> for PWMMR3RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR3RR::DISABLED_ => false,
            PWMMR3RR::RESET_ON_PWMMR3_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR3R_R = crate::FR<bool, PWMMR3RR>;
impl PWMMR3R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR3RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR3_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr3_the(&self) -> bool {
        *self == PWMMR3RR::RESET_ON_PWMMR3_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET_ON_PWMMR3_THE,
}
impl PWMMR3RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3RW::DISABLED_ => false,
            PWMMR3RW::RESET_ON_PWMMR3_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR3RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR3RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr3_the(self) -> &'a mut W {
        self.variant(PWMMR3RW::RESET_ON_PWMMR3_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR3_THE_,
}
impl crate::ToBits<bool> for PWMMR3SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR3SR::DISABLED => false,
            PWMMR3SR::STOP_ON_PWMMR3_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR3S_R = crate::FR<bool, PWMMR3SR>;
impl PWMMR3S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR3_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr3_the_(&self) -> bool {
        *self == PWMMR3SR::STOP_ON_PWMMR3_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR3_THE_,
}
impl PWMMR3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3SW::DISABLED => false,
            PWMMR3SW::STOP_ON_PWMMR3_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR3SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR3SW::DISABLED)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr3_the_(self) -> &'a mut W {
        self.variant(PWMMR3SW::STOP_ON_PWMMR3_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR4I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR4,
}
impl crate::ToBits<bool> for PWMMR4IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR4IR::DISABLED_ => false,
            PWMMR4IR::INTERRUPT_ON_PWMMR4 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR4I_R = crate::FR<bool, PWMMR4IR>;
impl PWMMR4I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR4IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR4`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr4(&self) -> bool {
        *self == PWMMR4IR::INTERRUPT_ON_PWMMR4
    }
}
#[doc = "Values that can be written to the field `PWMMR4I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR4,
}
impl PWMMR4IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4IW::DISABLED_ => false,
            PWMMR4IW::INTERRUPT_ON_PWMMR4 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR4IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR4IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr4(self) -> &'a mut W {
        self.variant(PWMMR4IW::INTERRUPT_ON_PWMMR4)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR4R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET_ON_PWMMR4_THE,
}
impl crate::ToBits<bool> for PWMMR4RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR4RR::DISABLED_ => false,
            PWMMR4RR::RESET_ON_PWMMR4_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR4R_R = crate::FR<bool, PWMMR4RR>;
impl PWMMR4R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR4RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR4_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr4_the(&self) -> bool {
        *self == PWMMR4RR::RESET_ON_PWMMR4_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR4R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET_ON_PWMMR4_THE,
}
impl PWMMR4RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4RW::DISABLED_ => false,
            PWMMR4RW::RESET_ON_PWMMR4_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR4RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR4RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr4_the(self) -> &'a mut W {
        self.variant(PWMMR4RW::RESET_ON_PWMMR4_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP_ON_PWMMR4_THE_,
}
impl crate::ToBits<bool> for PWMMR4SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR4SR::DISABLED => false,
            PWMMR4SR::STOP_ON_PWMMR4_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR4S_R = crate::FR<bool, PWMMR4SR>;
impl PWMMR4S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR4_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr4_the_(&self) -> bool {
        *self == PWMMR4SR::STOP_ON_PWMMR4_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP_ON_PWMMR4_THE_,
}
impl PWMMR4SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4SW::DISABLED => false,
            PWMMR4SW::STOP_ON_PWMMR4_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR4SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR4SW::DISABLED)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr4_the_(self) -> &'a mut W {
        self.variant(PWMMR4SW::STOP_ON_PWMMR4_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR5,
}
impl crate::ToBits<bool> for PWMMR5IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR5IR::DISABLED_ => false,
            PWMMR5IR::INTERRUPT_ON_PWMMR5 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR5I_R = crate::FR<bool, PWMMR5IR>;
impl PWMMR5I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR5IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR5`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr5(&self) -> bool {
        *self == PWMMR5IR::INTERRUPT_ON_PWMMR5
    }
}
#[doc = "Values that can be written to the field `PWMMR5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR5,
}
impl PWMMR5IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5IW::DISABLED_ => false,
            PWMMR5IW::INTERRUPT_ON_PWMMR5 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR5IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR5IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr5(self) -> &'a mut W {
        self.variant(PWMMR5IW::INTERRUPT_ON_PWMMR5)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR5R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET_ON_PWMMR5_THE,
}
impl crate::ToBits<bool> for PWMMR5RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR5RR::DISABLED_ => false,
            PWMMR5RR::RESET_ON_PWMMR5_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR5R_R = crate::FR<bool, PWMMR5RR>;
impl PWMMR5R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR5RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR5_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr5_the(&self) -> bool {
        *self == PWMMR5RR::RESET_ON_PWMMR5_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR5R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET_ON_PWMMR5_THE,
}
impl PWMMR5RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5RW::DISABLED_ => false,
            PWMMR5RW::RESET_ON_PWMMR5_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR5RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR5RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr5_the(self) -> &'a mut W {
        self.variant(PWMMR5RW::RESET_ON_PWMMR5_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR5S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP_ON_PWMMR5_THE_,
}
impl crate::ToBits<bool> for PWMMR5SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR5SR::DISABLED => false,
            PWMMR5SR::STOP_ON_PWMMR5_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR5S_R = crate::FR<bool, PWMMR5SR>;
impl PWMMR5S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR5_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr5_the_(&self) -> bool {
        *self == PWMMR5SR::STOP_ON_PWMMR5_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR5S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP_ON_PWMMR5_THE_,
}
impl PWMMR5SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5SW::DISABLED => false,
            PWMMR5SW::STOP_ON_PWMMR5_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR5SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR5SW::DISABLED)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr5_the_(self) -> &'a mut W {
        self.variant(PWMMR5SW::STOP_ON_PWMMR5_THE_)
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
#[doc = "Possible values of the field `PWMMR6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6IR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR6,
}
impl crate::ToBits<bool> for PWMMR6IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR6IR::DISABLED_ => false,
            PWMMR6IR::INTERRUPT_ON_PWMMR6 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR6I_R = crate::FR<bool, PWMMR6IR>;
impl PWMMR6I_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR6IR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR6`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr6(&self) -> bool {
        *self == PWMMR6IR::INTERRUPT_ON_PWMMR6
    }
}
#[doc = "Values that can be written to the field `PWMMR6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6IW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR6,
}
impl PWMMR6IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6IW::DISABLED_ => false,
            PWMMR6IW::INTERRUPT_ON_PWMMR6 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR6IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR6IW::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr6(self) -> &'a mut W {
        self.variant(PWMMR6IW::INTERRUPT_ON_PWMMR6)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR6R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6RR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET_ON_PWMMR6_THE,
}
impl crate::ToBits<bool> for PWMMR6RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR6RR::DISABLED_ => false,
            PWMMR6RR::RESET_ON_PWMMR6_THE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR6R_R = crate::FR<bool, PWMMR6RR>;
impl PWMMR6R_R {
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR6RR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR6_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr6_the(&self) -> bool {
        *self == PWMMR6RR::RESET_ON_PWMMR6_THE
    }
}
#[doc = "Values that can be written to the field `PWMMR6R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6RW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET_ON_PWMMR6_THE,
}
impl PWMMR6RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6RW::DISABLED_ => false,
            PWMMR6RW::RESET_ON_PWMMR6_THE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR6RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR6RW::DISABLED_)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr6_the(self) -> &'a mut W {
        self.variant(PWMMR6RW::RESET_ON_PWMMR6_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `PWMMR6S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP_ON_PWMMR6_THE_,
}
impl crate::ToBits<bool> for PWMMR6SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMMR6SR::DISABLED => false,
            PWMMR6SR::STOP_ON_PWMMR6_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR6S_R = crate::FR<bool, PWMMR6SR>;
impl PWMMR6S_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR6_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr6_the_(&self) -> bool {
        *self == PWMMR6SR::STOP_ON_PWMMR6_THE_
    }
}
#[doc = "Values that can be written to the field `PWMMR6S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP_ON_PWMMR6_THE_,
}
impl PWMMR6SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6SW::DISABLED => false,
            PWMMR6SW::STOP_ON_PWMMR6_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMMR6SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR6SW::DISABLED)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr6_the_(self) -> &'a mut W {
        self.variant(PWMMR6SW::STOP_ON_PWMMR6_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&self) -> PWMMR0I_R {
        PWMMR0I_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&self) -> PWMMR0R_R {
        PWMMR0R_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&self) -> PWMMR0S_R {
        PWMMR0S_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&self) -> PWMMR1I_R {
        PWMMR1I_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&self) -> PWMMR1R_R {
        PWMMR1R_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&self) -> PWMMR1S_R {
        PWMMR1S_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&self) -> PWMMR2I_R {
        PWMMR2I_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&self) -> PWMMR2R_R {
        PWMMR2R_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&self) -> PWMMR2S_R {
        PWMMR2S_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&self) -> PWMMR3I_R {
        PWMMR3I_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&self) -> PWMMR3R_R {
        PWMMR3R_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&self) -> PWMMR3S_R {
        PWMMR3S_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&self) -> PWMMR4I_R {
        PWMMR4I_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&self) -> PWMMR4R_R {
        PWMMR4R_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&self) -> PWMMR4S_R {
        PWMMR4S_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&self) -> PWMMR5I_R {
        PWMMR5I_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&self) -> PWMMR5R_R {
        PWMMR5R_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&self) -> PWMMR5S_R {
        PWMMR5S_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&self) -> PWMMR6I_R {
        PWMMR6I_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&self) -> PWMMR6R_R {
        PWMMR6R_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&self) -> PWMMR6S_R {
        PWMMR6S_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&mut self) -> _PWMMR0IW {
        _PWMMR0IW { w: self }
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&mut self) -> _PWMMR0RW {
        _PWMMR0RW { w: self }
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&mut self) -> _PWMMR0SW {
        _PWMMR0SW { w: self }
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&mut self) -> _PWMMR1IW {
        _PWMMR1IW { w: self }
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&mut self) -> _PWMMR1RW {
        _PWMMR1RW { w: self }
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&mut self) -> _PWMMR1SW {
        _PWMMR1SW { w: self }
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&mut self) -> _PWMMR2IW {
        _PWMMR2IW { w: self }
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&mut self) -> _PWMMR2RW {
        _PWMMR2RW { w: self }
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&mut self) -> _PWMMR2SW {
        _PWMMR2SW { w: self }
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&mut self) -> _PWMMR3IW {
        _PWMMR3IW { w: self }
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&mut self) -> _PWMMR3RW {
        _PWMMR3RW { w: self }
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&mut self) -> _PWMMR3SW {
        _PWMMR3SW { w: self }
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&mut self) -> _PWMMR4IW {
        _PWMMR4IW { w: self }
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&mut self) -> _PWMMR4RW {
        _PWMMR4RW { w: self }
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&mut self) -> _PWMMR4SW {
        _PWMMR4SW { w: self }
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&mut self) -> _PWMMR5IW {
        _PWMMR5IW { w: self }
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&mut self) -> _PWMMR5RW {
        _PWMMR5RW { w: self }
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&mut self) -> _PWMMR5SW {
        _PWMMR5SW { w: self }
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&mut self) -> _PWMMR6IW {
        _PWMMR6IW { w: self }
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&mut self) -> _PWMMR6RW {
        _PWMMR6RW { w: self }
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&mut self) -> _PWMMR6SW {
        _PWMMR6SW { w: self }
    }
}
