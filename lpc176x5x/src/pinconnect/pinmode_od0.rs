#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD0 {
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
#[doc = "Possible values of the field `P0_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00ODR {
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_00ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_00ODR::NORMAL => false,
            P0_00ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_00OD_R = crate::FR<bool, P0_00ODR>;
impl P0_00OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_00ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_00ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00ODW {
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_00ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_00ODW::NORMAL => false,
            P0_00ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_00ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_00ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_00ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_00ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_00ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01ODR {
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_01ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_01ODR::NORMAL => false,
            P0_01ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_01OD_R = crate::FR<bool, P0_01ODR>;
impl P0_01OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_01ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_01ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01ODW {
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_01ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_01ODW::NORMAL => false,
            P0_01ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_01ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_01ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_01ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_01ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_01ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_02OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02ODR {
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_02ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_02ODR::NORMAL => false,
            P0_02ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_02OD_R = crate::FR<bool, P0_02ODR>;
impl P0_02OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_02ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_02ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_02OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02ODW {
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_02ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_02ODW::NORMAL => false,
            P0_02ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_02ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_02ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_02ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_02ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_02ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_03OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03ODR {
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_03ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_03ODR::NORMAL => false,
            P0_03ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_03OD_R = crate::FR<bool, P0_03ODR>;
impl P0_03OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_03ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_03ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_03OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03ODW {
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_03ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_03ODW::NORMAL => false,
            P0_03ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_03ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_03ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_03ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_03ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_03ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04ODR {
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_04ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_04ODR::NORMAL => false,
            P0_04ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_04OD_R = crate::FR<bool, P0_04ODR>;
impl P0_04OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_04ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_04ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04ODW {
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_04ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_04ODW::NORMAL => false,
            P0_04ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_04ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_04ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_04ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_04ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_04ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_05OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05ODR {
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_05ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_05ODR::NORMAL => false,
            P0_05ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_05OD_R = crate::FR<bool, P0_05ODR>;
impl P0_05OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_05ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_05ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_05OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05ODW {
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_05ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_05ODW::NORMAL => false,
            P0_05ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_05ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_05ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_05ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_05ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_05ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_06OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06ODR {
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_06ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_06ODR::NORMAL => false,
            P0_06ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_06OD_R = crate::FR<bool, P0_06ODR>;
impl P0_06OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_06ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_06ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_06OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06ODW {
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_06ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_06ODW::NORMAL => false,
            P0_06ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_06ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_06ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_06ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_06ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_06ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_07OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07ODR {
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_07ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_07ODR::NORMAL => false,
            P0_07ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_07OD_R = crate::FR<bool, P0_07ODR>;
impl P0_07OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_07ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_07ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_07OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07ODW {
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_07ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_07ODW::NORMAL => false,
            P0_07ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_07ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_07ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_07ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_07ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_07ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08ODR {
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_08ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_08ODR::NORMAL => false,
            P0_08ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_08OD_R = crate::FR<bool, P0_08ODR>;
impl P0_08OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_08ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_08ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08ODW {
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_08ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_08ODW::NORMAL => false,
            P0_08ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_08ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_08ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_08ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_08ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_08ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09ODR {
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_09ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_09ODR::NORMAL => false,
            P0_09ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_09OD_R = crate::FR<bool, P0_09ODR>;
impl P0_09OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_09ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_09ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09ODW {
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_09ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_09ODW::NORMAL => false,
            P0_09ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_09ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_09ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_09ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_09ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_09ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10ODR {
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_10ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_10ODR::NORMAL => false,
            P0_10ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_10OD_R = crate::FR<bool, P0_10ODR>;
impl P0_10OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_10ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_10ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10ODW {
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_10ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_10ODW::NORMAL => false,
            P0_10ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_10ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_10ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_10ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_11OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11ODR {
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_11ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_11ODR::NORMAL => false,
            P0_11ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_11OD_R = crate::FR<bool, P0_11ODR>;
impl P0_11OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_11ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_11ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_11OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11ODW {
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_11ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_11ODW::NORMAL => false,
            P0_11ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_11ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_11ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_11ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_15OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15ODR {
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_15ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_15ODR::NORMAL => false,
            P0_15ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_15OD_R = crate::FR<bool, P0_15ODR>;
impl P0_15OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_15ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_15ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_15OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15ODW {
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_15ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_15ODW::NORMAL => false,
            P0_15ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_15ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_15ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_15ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_16OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16ODR {
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_16ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_16ODR::NORMAL => false,
            P0_16ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_16OD_R = crate::FR<bool, P0_16ODR>;
impl P0_16OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_16ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_16ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_16OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16ODW {
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_16ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_16ODW::NORMAL => false,
            P0_16ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_16ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_16ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_16ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_16ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_17OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17ODR {
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_17ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_17ODR::NORMAL => false,
            P0_17ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_17OD_R = crate::FR<bool, P0_17ODR>;
impl P0_17OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_17ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_17ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_17OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17ODW {
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_17ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_17ODW::NORMAL => false,
            P0_17ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_17ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_17ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_17ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_17ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_18OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18ODR {
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_18ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_18ODR::NORMAL => false,
            P0_18ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_18OD_R = crate::FR<bool, P0_18ODR>;
impl P0_18OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_18ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_18ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_18OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18ODW {
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_18ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_18ODW::NORMAL => false,
            P0_18ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_18ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_18ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_18ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_18ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_19OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19ODR {
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_19ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_19ODR::NORMAL => false,
            P0_19ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_19OD_R = crate::FR<bool, P0_19ODR>;
impl P0_19OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_19ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_19ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_19OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19ODW {
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_19ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_19ODW::NORMAL => false,
            P0_19ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_19ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_19ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_19ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_19ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_20OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20ODR {
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_20ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_20ODR::NORMAL => false,
            P0_20ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_20OD_R = crate::FR<bool, P0_20ODR>;
impl P0_20OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_20ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_20ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_20OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20ODW {
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_20ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_20ODW::NORMAL => false,
            P0_20ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_20ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_20ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_20ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_20ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P0_21OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21ODR {
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_21ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_21ODR::NORMAL => false,
            P0_21ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_21OD_R = crate::FR<bool, P0_21ODR>;
impl P0_21OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_21ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_21ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_21OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21ODW {
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_21ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_21ODW::NORMAL => false,
            P0_21ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_21ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_21ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_21ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_21ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `P0_22OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22ODR {
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_22ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_22ODR::NORMAL => false,
            P0_22ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_22OD_R = crate::FR<bool, P0_22ODR>;
impl P0_22OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_22ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_22ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_22OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22ODW {
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_22ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_22ODW::NORMAL => false,
            P0_22ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_22ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_22ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_22ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_22ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P0_23OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23ODR {
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_23ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_23ODR::NORMAL => false,
            P0_23ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_23OD_R = crate::FR<bool, P0_23ODR>;
impl P0_23OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_23ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_23ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_23OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23ODW {
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_23ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_23ODW::NORMAL => false,
            P0_23ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_23ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_23ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_23ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_23ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `P0_24OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24ODR {
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_24ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_24ODR::NORMAL => false,
            P0_24ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_24OD_R = crate::FR<bool, P0_24ODR>;
impl P0_24OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_24ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_24ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_24OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24ODW {
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_24ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_24ODW::NORMAL => false,
            P0_24ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_24ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_24ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_24ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_24ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P0_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25ODR {
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_25ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_25ODR::NORMAL => false,
            P0_25ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_25OD_R = crate::FR<bool, P0_25ODR>;
impl P0_25OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_25ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_25ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25ODW {
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_25ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_25ODW::NORMAL => false,
            P0_25ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_25ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_25ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_25ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_25ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `P0_26OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26ODR {
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_26ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_26ODR::NORMAL => false,
            P0_26ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_26OD_R = crate::FR<bool, P0_26ODR>;
impl P0_26OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_26ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_26ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_26OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26ODW {
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_26ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_26ODW::NORMAL => false,
            P0_26ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_26ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_26ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_26ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_26ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `P0_29OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_29ODR {
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_29ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_29ODR::NORMAL => false,
            P0_29ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_29OD_R = crate::FR<bool, P0_29ODR>;
impl P0_29OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_29ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_29ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_29OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_29ODW {
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_29ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_29ODW::NORMAL => false,
            P0_29ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_29ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_29ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_29ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_29ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_29ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Possible values of the field `P0_30OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_30ODR {
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P0_30ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0_30ODR::NORMAL => false,
            P0_30ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_30OD_R = crate::FR<bool, P0_30ODR>;
impl P0_30OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_30ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_30ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P0_30OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_30ODW {
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P0_30ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P0_30ODW::NORMAL => false,
            P0_30ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_30ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_30ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_30ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_30ODW::NORMAL)
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_30ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&self) -> P0_00OD_R {
        P0_00OD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&self) -> P0_01OD_R {
        P0_01OD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&self) -> P0_02OD_R {
        P0_02OD_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&self) -> P0_03OD_R {
        P0_03OD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&self) -> P0_04OD_R {
        P0_04OD_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&self) -> P0_05OD_R {
        P0_05OD_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&self) -> P0_06OD_R {
        P0_06OD_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&self) -> P0_07OD_R {
        P0_07OD_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&self) -> P0_08OD_R {
        P0_08OD_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&self) -> P0_09OD_R {
        P0_09OD_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&self) -> P0_10OD_R {
        P0_10OD_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&self) -> P0_11OD_R {
        P0_11OD_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&self) -> P0_15OD_R {
        P0_15OD_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&self) -> P0_16OD_R {
        P0_16OD_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&self) -> P0_17OD_R {
        P0_17OD_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&self) -> P0_18OD_R {
        P0_18OD_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&self) -> P0_19OD_R {
        P0_19OD_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&self) -> P0_20OD_R {
        P0_20OD_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&self) -> P0_21OD_R {
        P0_21OD_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&self) -> P0_22OD_R {
        P0_22OD_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&self) -> P0_23OD_R {
        P0_23OD_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&self) -> P0_24OD_R {
        P0_24OD_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&self) -> P0_25OD_R {
        P0_25OD_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&self) -> P0_26OD_R {
        P0_26OD_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&self) -> P0_29OD_R {
        P0_29OD_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&self) -> P0_30OD_R {
        P0_30OD_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&mut self) -> _P0_00ODW {
        _P0_00ODW { w: self }
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&mut self) -> _P0_01ODW {
        _P0_01ODW { w: self }
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&mut self) -> _P0_02ODW {
        _P0_02ODW { w: self }
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&mut self) -> _P0_03ODW {
        _P0_03ODW { w: self }
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&mut self) -> _P0_04ODW {
        _P0_04ODW { w: self }
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&mut self) -> _P0_05ODW {
        _P0_05ODW { w: self }
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&mut self) -> _P0_06ODW {
        _P0_06ODW { w: self }
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&mut self) -> _P0_07ODW {
        _P0_07ODW { w: self }
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&mut self) -> _P0_08ODW {
        _P0_08ODW { w: self }
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&mut self) -> _P0_09ODW {
        _P0_09ODW { w: self }
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&mut self) -> _P0_10ODW {
        _P0_10ODW { w: self }
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&mut self) -> _P0_11ODW {
        _P0_11ODW { w: self }
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&mut self) -> _P0_15ODW {
        _P0_15ODW { w: self }
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&mut self) -> _P0_16ODW {
        _P0_16ODW { w: self }
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&mut self) -> _P0_17ODW {
        _P0_17ODW { w: self }
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&mut self) -> _P0_18ODW {
        _P0_18ODW { w: self }
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&mut self) -> _P0_19ODW {
        _P0_19ODW { w: self }
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&mut self) -> _P0_20ODW {
        _P0_20ODW { w: self }
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&mut self) -> _P0_21ODW {
        _P0_21ODW { w: self }
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&mut self) -> _P0_22ODW {
        _P0_22ODW { w: self }
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&mut self) -> _P0_23ODW {
        _P0_23ODW { w: self }
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&mut self) -> _P0_24ODW {
        _P0_24ODW { w: self }
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&mut self) -> _P0_25ODW {
        _P0_25ODW { w: self }
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&mut self) -> _P0_26ODW {
        _P0_26ODW { w: self }
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&mut self) -> _P0_29ODW {
        _P0_29ODW { w: self }
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&mut self) -> _P0_30ODW {
        _P0_30ODW { w: self }
    }
}
