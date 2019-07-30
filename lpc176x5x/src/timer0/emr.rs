#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMR {
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
#[doc = r"Reader of the field"]
pub type EM0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EM0W<'a> {
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
#[doc = r"Reader of the field"]
pub type EM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EM1W<'a> {
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
#[doc = r"Reader of the field"]
pub type EM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM2W<'a> {
    w: &'a mut W,
}
impl<'a> _EM2W<'a> {
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
#[doc = r"Reader of the field"]
pub type EM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EM3W<'a> {
    w: &'a mut W,
}
impl<'a> _EM3W<'a> {
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
#[doc = "Possible values of the field `EMC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC0R {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl crate::ToBits<u8> for EMC0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC0R::DO_NOTHING_ => 0,
            EMC0R::CLEAR_THE_CORRESPOND => 1,
            EMC0R::SET_THE_CORRESPONDIN => 2,
            EMC0R::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC0_R = crate::FR<u8, EMC0R>;
impl EMC0_R {
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC0R::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC0R::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC0R::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC0R::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Values that can be written to the field `EMC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC0W {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl EMC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC0W::DO_NOTHING_ => 0,
            EMC0W::CLEAR_THE_CORRESPOND => 1,
            EMC0W::SET_THE_CORRESPONDIN => 2,
            EMC0W::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC0W::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC0W::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC0W::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC0W::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `EMC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC1R {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl crate::ToBits<u8> for EMC1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC1R::DO_NOTHING_ => 0,
            EMC1R::CLEAR_THE_CORRESPOND => 1,
            EMC1R::SET_THE_CORRESPONDIN => 2,
            EMC1R::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC1_R = crate::FR<u8, EMC1R>;
impl EMC1_R {
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC1R::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC1R::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC1R::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC1R::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Values that can be written to the field `EMC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC1W {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl EMC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC1W::DO_NOTHING_ => 0,
            EMC1W::CLEAR_THE_CORRESPOND => 1,
            EMC1W::SET_THE_CORRESPONDIN => 2,
            EMC1W::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC1W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC1W::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC1W::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC1W::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC1W::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `EMC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC2R {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl crate::ToBits<u8> for EMC2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC2R::DO_NOTHING_ => 0,
            EMC2R::CLEAR_THE_CORRESPOND => 1,
            EMC2R::SET_THE_CORRESPONDIN => 2,
            EMC2R::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC2_R = crate::FR<u8, EMC2R>;
impl EMC2_R {
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC2R::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC2R::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC2R::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC2R::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Values that can be written to the field `EMC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC2W {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl EMC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC2W::DO_NOTHING_ => 0,
            EMC2W::CLEAR_THE_CORRESPOND => 1,
            EMC2W::SET_THE_CORRESPONDIN => 2,
            EMC2W::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC2W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC2W::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC2W::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC2W::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC2W::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `EMC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC3R {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl crate::ToBits<u8> for EMC3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EMC3R::DO_NOTHING_ => 0,
            EMC3R::CLEAR_THE_CORRESPOND => 1,
            EMC3R::SET_THE_CORRESPONDIN => 2,
            EMC3R::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EMC3_R = crate::FR<u8, EMC3R>;
impl EMC3_R {
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC3R::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC3R::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC3R::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC3R::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Values that can be written to the field `EMC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC3W {
    #[doc = "Do Nothing."]
    DO_NOTHING_,
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND,
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN,
    #[doc = "Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON,
}
impl EMC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMC3W::DO_NOTHING_ => 0,
            EMC3W::CLEAR_THE_CORRESPOND => 1,
            EMC3W::SET_THE_CORRESPONDIN => 2,
            EMC3W::TOGGLE_THE_CORRESPON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMC3W<'a> {
    w: &'a mut W,
}
impl<'a> _EMC3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC3W::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC3W::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC3W::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC3W::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&mut self) -> _EM0W {
        _EM0W { w: self }
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&mut self) -> _EM1W {
        _EM1W { w: self }
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&mut self) -> _EM2W {
        _EM2W { w: self }
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&mut self) -> _EM3W {
        _EM3W { w: self }
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&mut self) -> _EMC0W {
        _EMC0W { w: self }
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&mut self) -> _EMC1W {
        _EMC1W { w: self }
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&mut self) -> _EMC2W {
        _EMC2W { w: self }
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&mut self) -> _EMC3W {
        _EMC3W { w: self }
    }
}
