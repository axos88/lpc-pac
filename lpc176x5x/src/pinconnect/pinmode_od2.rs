#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD2 {
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
#[doc = "Possible values of the field `P2_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00ODR {
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_00ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_00ODR::NORMAL => false,
            P2_00ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_00OD_R = crate::FR<bool, P2_00ODR>;
impl P2_00OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_00ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_00ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00ODW {
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_00ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_00ODW::NORMAL => false,
            P2_00ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_00ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_00ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_00ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_00ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_00ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01ODR {
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_01ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_01ODR::NORMAL => false,
            P2_01ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_01OD_R = crate::FR<bool, P2_01ODR>;
impl P2_01OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_01ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_01ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01ODW {
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_01ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_01ODW::NORMAL => false,
            P2_01ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_01ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_01ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_01ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_01ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_01ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_02OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02ODR {
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_02ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_02ODR::NORMAL => false,
            P2_02ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_02OD_R = crate::FR<bool, P2_02ODR>;
impl P2_02OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_02ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_02ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_02OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02ODW {
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_02ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_02ODW::NORMAL => false,
            P2_02ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_02ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_02ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_02ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_02ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_02ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_03OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03ODR {
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_03ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_03ODR::NORMAL => false,
            P2_03ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_03OD_R = crate::FR<bool, P2_03ODR>;
impl P2_03OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_03ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_03ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_03OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03ODW {
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_03ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_03ODW::NORMAL => false,
            P2_03ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_03ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_03ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_03ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_03ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_03ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04ODR {
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_04ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_04ODR::NORMAL => false,
            P2_04ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_04OD_R = crate::FR<bool, P2_04ODR>;
impl P2_04OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_04ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_04ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04ODW {
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_04ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_04ODW::NORMAL => false,
            P2_04ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_04ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_04ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_04ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_04ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_04ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_05OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05ODR {
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_05ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_05ODR::NORMAL => false,
            P2_05ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_05OD_R = crate::FR<bool, P2_05ODR>;
impl P2_05OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_05ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_05ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_05OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05ODW {
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_05ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_05ODW::NORMAL => false,
            P2_05ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_05ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_05ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_05ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_05ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_05ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_06OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06ODR {
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_06ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_06ODR::NORMAL => false,
            P2_06ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_06OD_R = crate::FR<bool, P2_06ODR>;
impl P2_06OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_06ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_06ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_06OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06ODW {
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_06ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_06ODW::NORMAL => false,
            P2_06ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_06ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_06ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_06ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_06ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_06ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_07OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07ODR {
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_07ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_07ODR::NORMAL => false,
            P2_07ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_07OD_R = crate::FR<bool, P2_07ODR>;
impl P2_07OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_07ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_07ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_07OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07ODW {
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_07ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_07ODW::NORMAL => false,
            P2_07ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_07ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_07ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_07ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_07ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_07ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08ODR {
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_08ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_08ODR::NORMAL => false,
            P2_08ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_08OD_R = crate::FR<bool, P2_08ODR>;
impl P2_08OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_08ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_08ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08ODW {
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_08ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_08ODW::NORMAL => false,
            P2_08ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_08ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_08ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_08ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_08ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_08ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09ODR {
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_09ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_09ODR::NORMAL => false,
            P2_09ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_09OD_R = crate::FR<bool, P2_09ODR>;
impl P2_09OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_09ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_09ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09ODW {
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_09ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_09ODW::NORMAL => false,
            P2_09ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_09ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_09ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_09ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_09ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_09ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10ODR {
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_10ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_10ODR::NORMAL => false,
            P2_10ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_10OD_R = crate::FR<bool, P2_10ODR>;
impl P2_10OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_10ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_10ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10ODW {
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_10ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_10ODW::NORMAL => false,
            P2_10ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_10ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_10ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_10ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_10ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_11OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11ODR {
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_11ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_11ODR::NORMAL => false,
            P2_11ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_11OD_R = crate::FR<bool, P2_11ODR>;
impl P2_11OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_11ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_11ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_11OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11ODW {
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_11ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_11ODW::NORMAL => false,
            P2_11ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_11ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_11ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_11ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_11ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_12OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12ODR {
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_12ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_12ODR::NORMAL => false,
            P2_12ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_12OD_R = crate::FR<bool, P2_12ODR>;
impl P2_12OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_12ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_12ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_12OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12ODW {
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_12ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_12ODW::NORMAL => false,
            P2_12ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_12ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_12ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_12ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_12ODW::OPEN_DRAIN)
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
#[doc = "Possible values of the field `P2_13OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13ODR {
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P2_13ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2_13ODR::NORMAL => false,
            P2_13ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_13OD_R = crate::FR<bool, P2_13ODR>;
impl P2_13OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_13ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_13ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_13OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13ODW {
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_13ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_13ODW::NORMAL => false,
            P2_13ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_13ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_13ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_13ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_13ODW::OPEN_DRAIN)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&self) -> P2_00OD_R {
        P2_00OD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&self) -> P2_01OD_R {
        P2_01OD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&self) -> P2_02OD_R {
        P2_02OD_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&self) -> P2_03OD_R {
        P2_03OD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&self) -> P2_04OD_R {
        P2_04OD_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&self) -> P2_05OD_R {
        P2_05OD_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&self) -> P2_06OD_R {
        P2_06OD_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&self) -> P2_07OD_R {
        P2_07OD_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&self) -> P2_08OD_R {
        P2_08OD_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&self) -> P2_09OD_R {
        P2_09OD_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&self) -> P2_10OD_R {
        P2_10OD_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&self) -> P2_11OD_R {
        P2_11OD_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&self) -> P2_12OD_R {
        P2_12OD_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&self) -> P2_13OD_R {
        P2_13OD_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&mut self) -> _P2_00ODW {
        _P2_00ODW { w: self }
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&mut self) -> _P2_01ODW {
        _P2_01ODW { w: self }
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&mut self) -> _P2_02ODW {
        _P2_02ODW { w: self }
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&mut self) -> _P2_03ODW {
        _P2_03ODW { w: self }
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&mut self) -> _P2_04ODW {
        _P2_04ODW { w: self }
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&mut self) -> _P2_05ODW {
        _P2_05ODW { w: self }
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&mut self) -> _P2_06ODW {
        _P2_06ODW { w: self }
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&mut self) -> _P2_07ODW {
        _P2_07ODW { w: self }
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&mut self) -> _P2_08ODW {
        _P2_08ODW { w: self }
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&mut self) -> _P2_09ODW {
        _P2_09ODW { w: self }
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&mut self) -> _P2_10ODW {
        _P2_10ODW { w: self }
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&mut self) -> _P2_11ODW {
        _P2_11ODW { w: self }
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&mut self) -> _P2_12ODW {
        _P2_12ODW { w: self }
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&mut self) -> _P2_13ODW {
        _P2_13ODW { w: self }
    }
}
