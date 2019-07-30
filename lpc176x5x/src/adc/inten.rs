#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
        0x0100
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `ADINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN0R {
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN0R::DISABLE => false,
            ADINTEN0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN0_R = crate::FR<bool, ADINTEN0R>;
impl ADINTEN0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN0W {
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN0W::DISABLE => false,
            ADINTEN0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN0W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN0W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN1R {
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN1R::DISABLE => false,
            ADINTEN1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN1_R = crate::FR<bool, ADINTEN1R>;
impl ADINTEN1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN1W {
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN1W::DISABLE => false,
            ADINTEN1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN1W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN1W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN2R {
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN2R::DISABLE => false,
            ADINTEN2R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN2_R = crate::FR<bool, ADINTEN2R>;
impl ADINTEN2_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN2R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN2W {
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN2W::DISABLE => false,
            ADINTEN2W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN2W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN2W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN3R {
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN3R::DISABLE => false,
            ADINTEN3R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN3_R = crate::FR<bool, ADINTEN3R>;
impl ADINTEN3_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN3R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN3W {
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN3W::DISABLE => false,
            ADINTEN3W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN3W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN3W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN4R {
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN4R::DISABLE => false,
            ADINTEN4R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN4_R = crate::FR<bool, ADINTEN4R>;
impl ADINTEN4_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN4R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN4W {
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN4W::DISABLE => false,
            ADINTEN4W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN4W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN4W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN5R {
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN5R::DISABLE => false,
            ADINTEN5R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN5_R = crate::FR<bool, ADINTEN5R>;
impl ADINTEN5_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN5R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN5W {
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN5W::DISABLE => false,
            ADINTEN5W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN5W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN5W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN6R {
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN6R::DISABLE => false,
            ADINTEN6R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN6_R = crate::FR<bool, ADINTEN6R>;
impl ADINTEN6_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN6R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN6W {
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN6W::DISABLE => false,
            ADINTEN6W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN6W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN6W::ENABLE)
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
#[doc = "Possible values of the field `ADINTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN7R {
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE,
}
impl crate::ToBits<bool> for ADINTEN7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADINTEN7R::DISABLE => false,
            ADINTEN7R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADINTEN7_R = crate::FR<bool, ADINTEN7R>;
impl ADINTEN7_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN7R::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADINTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN7W {
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN7W::DISABLE => false,
            ADINTEN7W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN7W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN7W::ENABLE)
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
#[doc = "Possible values of the field `ADGINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADGINTENR {
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS,
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL,
}
impl crate::ToBits<bool> for ADGINTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADGINTENR::CHANNELS => false,
            ADGINTENR::GLOBAL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADGINTEN_R = crate::FR<bool, ADGINTENR>;
impl ADGINTEN_R {
    #[doc = "Checks if the value of the field is `CHANNELS`"]
    #[inline(always)]
    pub fn is_channels(&self) -> bool {
        *self == ADGINTENR::CHANNELS
    }
    #[doc = "Checks if the value of the field is `GLOBAL`"]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == ADGINTENR::GLOBAL
    }
}
#[doc = "Values that can be written to the field `ADGINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADGINTENW {
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS,
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL,
}
impl ADGINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADGINTENW::CHANNELS => false,
            ADGINTENW::GLOBAL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADGINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADGINTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADGINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn channels(self) -> &'a mut W {
        self.variant(ADGINTENW::CHANNELS)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn global(self) -> &'a mut W {
        self.variant(ADGINTENW::GLOBAL)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&self) -> ADINTEN0_R {
        ADINTEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&self) -> ADINTEN1_R {
        ADINTEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&self) -> ADINTEN2_R {
        ADINTEN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&self) -> ADINTEN3_R {
        ADINTEN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&self) -> ADINTEN4_R {
        ADINTEN4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&self) -> ADINTEN5_R {
        ADINTEN5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&self) -> ADINTEN6_R {
        ADINTEN6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&self) -> ADINTEN7_R {
        ADINTEN7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&self) -> ADGINTEN_R {
        ADGINTEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&mut self) -> _ADINTEN0W {
        _ADINTEN0W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&mut self) -> _ADINTEN1W {
        _ADINTEN1W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&mut self) -> _ADINTEN2W {
        _ADINTEN2W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&mut self) -> _ADINTEN3W {
        _ADINTEN3W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&mut self) -> _ADINTEN4W {
        _ADINTEN4W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&mut self) -> _ADINTEN5W {
        _ADINTEN5W { w: self }
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&mut self) -> _ADINTEN6W {
        _ADINTEN6W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&mut self) -> _ADINTEN7W {
        _ADINTEN7W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&mut self) -> _ADGINTENW {
        _ADGINTENW { w: self }
    }
}
