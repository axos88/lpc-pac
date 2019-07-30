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
#[doc = "Possible values of the field `CAP0_R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_RR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH,
}
impl crate::ToBits<bool> for CAP0_RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0_RR::DISABLED_THIS_FEATU => false,
            CAP0_RR::RISING_EDGE_A_SYNCH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0_R_R = crate::FR<bool, CAP0_RR>;
impl CAP0_R_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_RR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`"]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == CAP0_RR::RISING_EDGE_A_SYNCH
    }
}
#[doc = "Values that can be written to the field `CAP0_R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_RW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH,
}
impl CAP0_RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0_RW::DISABLED_THIS_FEATU => false,
            CAP0_RW::RISING_EDGE_A_SYNCH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0_RW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0_RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_RW::DISABLED_THIS_FEATU)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut W {
        self.variant(CAP0_RW::RISING_EDGE_A_SYNCH)
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
#[doc = "Possible values of the field `CAP0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_FR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC,
}
impl crate::ToBits<bool> for CAP0_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0_FR::DISABLED_THIS_FEATU => false,
            CAP0_FR::FALLING_EDGE_A_SYNC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0_F_R = crate::FR<bool, CAP0_FR>;
impl CAP0_F_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_FR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`"]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == CAP0_FR::FALLING_EDGE_A_SYNC
    }
}
#[doc = "Values that can be written to the field `CAP0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_FW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC,
}
impl CAP0_FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0_FW::DISABLED_THIS_FEATU => false,
            CAP0_FW::FALLING_EDGE_A_SYNC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0_FW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0_FW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_FW::DISABLED_THIS_FEATU)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut W {
        self.variant(CAP0_FW::FALLING_EDGE_A_SYNC)
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
#[doc = "Possible values of the field `CAP0_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    INTERRUPT_A_CR0_LOA,
}
impl crate::ToBits<bool> for CAP0_IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0_IR::DISABLED_THIS_FEATU => false,
            CAP0_IR::INTERRUPT_A_CR0_LOA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0_I_R = crate::FR<bool, CAP0_IR>;
impl CAP0_I_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_IR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_A_CR0_LOA`"]
    #[inline(always)]
    pub fn is_interrupt_a_cr0_loa(&self) -> bool {
        *self == CAP0_IR::INTERRUPT_A_CR0_LOA
    }
}
#[doc = "Values that can be written to the field `CAP0_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    INTERRUPT_A_CR0_LOA,
}
impl CAP0_IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0_IW::DISABLED_THIS_FEATU => false,
            CAP0_IW::INTERRUPT_A_CR0_LOA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0_IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0_IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_IW::DISABLED_THIS_FEATU)
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr0_loa(self) -> &'a mut W {
        self.variant(CAP0_IW::INTERRUPT_A_CR0_LOA)
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
#[doc = "Possible values of the field `CAP1_R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_RR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH,
}
impl crate::ToBits<bool> for CAP1_RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1_RR::DISABLED_THIS_FEATU => false,
            CAP1_RR::RISING_EDGE_A_SYNCH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1_R_R = crate::FR<bool, CAP1_RR>;
impl CAP1_R_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_RR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`"]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == CAP1_RR::RISING_EDGE_A_SYNCH
    }
}
#[doc = "Values that can be written to the field `CAP1_R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_RW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH,
}
impl CAP1_RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1_RW::DISABLED_THIS_FEATU => false,
            CAP1_RW::RISING_EDGE_A_SYNCH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1_RW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1_RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_RW::DISABLED_THIS_FEATU)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut W {
        self.variant(CAP1_RW::RISING_EDGE_A_SYNCH)
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
#[doc = "Possible values of the field `CAP1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_FR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC,
}
impl crate::ToBits<bool> for CAP1_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1_FR::DISABLED_THIS_FEATU => false,
            CAP1_FR::FALLING_EDGE_A_SYNC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1_F_R = crate::FR<bool, CAP1_FR>;
impl CAP1_F_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_FR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`"]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == CAP1_FR::FALLING_EDGE_A_SYNC
    }
}
#[doc = "Values that can be written to the field `CAP1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_FW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC,
}
impl CAP1_FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1_FW::DISABLED_THIS_FEATU => false,
            CAP1_FW::FALLING_EDGE_A_SYNC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1_FW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1_FW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_FW::DISABLED_THIS_FEATU)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut W {
        self.variant(CAP1_FW::FALLING_EDGE_A_SYNC)
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
#[doc = "Possible values of the field `CAP1_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_IR {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    INTERRUPT_A_CR1_LOA,
}
impl crate::ToBits<bool> for CAP1_IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1_IR::DISABLED_THIS_FEATU => false,
            CAP1_IR::INTERRUPT_A_CR1_LOA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1_I_R = crate::FR<bool, CAP1_IR>;
impl CAP1_I_R {
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_IR::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_A_CR1_LOA`"]
    #[inline(always)]
    pub fn is_interrupt_a_cr1_loa(&self) -> bool {
        *self == CAP1_IR::INTERRUPT_A_CR1_LOA
    }
}
#[doc = "Values that can be written to the field `CAP1_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_IW {
    #[doc = "Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU,
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    INTERRUPT_A_CR1_LOA,
}
impl CAP1_IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1_IW::DISABLED_THIS_FEATU => false,
            CAP1_IW::INTERRUPT_A_CR1_LOA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1_IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1_IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_IW::DISABLED_THIS_FEATU)
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr1_loa(self) -> &'a mut W {
        self.variant(CAP1_IW::INTERRUPT_A_CR1_LOA)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&self) -> CAP0_R_R {
        CAP0_R_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&self) -> CAP0_F_R {
        CAP0_F_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&self) -> CAP0_I_R {
        CAP0_I_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&self) -> CAP1_R_R {
        CAP1_R_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&self) -> CAP1_F_R {
        CAP1_F_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&self) -> CAP1_I_R {
        CAP1_I_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&mut self) -> _CAP0_RW {
        _CAP0_RW { w: self }
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&mut self) -> _CAP0_FW {
        _CAP0_FW { w: self }
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&mut self) -> _CAP0_IW {
        _CAP0_IW { w: self }
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&mut self) -> _CAP1_RW {
        _CAP1_RW { w: self }
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&mut self) -> _CAP1_FW {
        _CAP1_FW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&mut self) -> _CAP1_IW {
        _CAP1_IW { w: self }
    }
}
