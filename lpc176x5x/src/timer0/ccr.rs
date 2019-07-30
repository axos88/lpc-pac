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
#[doc = "Possible values of the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RER {
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP0RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0RER::ENABLE => true,
            CAP0RER::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0RE_R = crate::FR<bool, CAP0RER>;
impl CAP0RE_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0RER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0RER::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0REW {
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0REW::ENABLE => true,
            CAP0REW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0REW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0REW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0REW::DISABLE)
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
#[doc = "Possible values of the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FER {
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP0FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0FER::ENABLE => true,
            CAP0FER::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0FE_R = crate::FR<bool, CAP0FER>;
impl CAP0FE_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0FER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0FER::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FEW {
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0FEW::ENABLE => true,
            CAP0FEW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0FEW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0FEW::DISABLE)
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
#[doc = "Possible values of the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IR {
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP0IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0IR::ENABLE => true,
            CAP0IR::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0I_R = crate::FR<bool, CAP0IR>;
impl CAP0I_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0IR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0IR::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IW {
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0IW::ENABLE => true,
            CAP0IW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0IW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0IW::DISABLE)
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
#[doc = "Possible values of the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RER {
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP1RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1RER::ENABLE => true,
            CAP1RER::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1RE_R = crate::FR<bool, CAP1RER>;
impl CAP1RE_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1RER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1RER::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1REW {
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1REW::ENABLE => true,
            CAP1REW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1REW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1REW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1REW::DISABLE)
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
#[doc = "Possible values of the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FER {
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP1FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1FER::ENABLE => true,
            CAP1FER::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1FE_R = crate::FR<bool, CAP1FER>;
impl CAP1FE_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1FER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1FER::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FEW {
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1FEW::ENABLE => true,
            CAP1FEW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1FEW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1FEW::DISABLE)
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
#[doc = "Possible values of the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IR {
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for CAP1IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1IR::ENABLE => true,
            CAP1IR::DISABLE => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1I_R = crate::FR<bool, CAP1IR>;
impl CAP1I_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1IR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1IR::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IW {
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1IW::ENABLE => true,
            CAP1IW::DISABLE => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1IW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1IW::DISABLE)
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&mut self) -> _CAP0REW {
        _CAP0REW { w: self }
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> _CAP0FEW {
        _CAP0FEW { w: self }
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&mut self) -> _CAP0IW {
        _CAP0IW { w: self }
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&mut self) -> _CAP1REW {
        _CAP1REW { w: self }
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&mut self) -> _CAP1FEW {
        _CAP1FEW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&mut self) -> _CAP1IW {
        _CAP1IW { w: self }
    }
}
