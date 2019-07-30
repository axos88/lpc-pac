#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CP {
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
#[doc = "Possible values of the field `CCPA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA0R {
    #[doc = "MCOA0 passive."]
    MCOA0_PASSIVE_,
    #[doc = "internal MCOA0."]
    INTERNAL_MCOA0_,
}
impl crate::ToBits<bool> for CCPA0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPA0R::MCOA0_PASSIVE_ => false,
            CCPA0R::INTERNAL_MCOA0_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPA0_R = crate::FR<bool, CCPA0R>;
impl CCPA0_R {
    #[doc = "Checks if the value of the field is `MCOA0_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa0_passive_(&self) -> bool {
        *self == CCPA0R::MCOA0_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `INTERNAL_MCOA0_`"]
    #[inline(always)]
    pub fn is_internal_mcoa0_(&self) -> bool {
        *self == CCPA0R::INTERNAL_MCOA0_
    }
}
#[doc = "Values that can be written to the field `CCPA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA0W {
    #[doc = "MCOA0 passive."]
    MCOA0_PASSIVE_,
    #[doc = "internal MCOA0."]
    INTERNAL_MCOA0_,
}
impl CCPA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPA0W::MCOA0_PASSIVE_ => false,
            CCPA0W::INTERNAL_MCOA0_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPA0W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPA0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn mcoa0_passive_(self) -> &'a mut W {
        self.variant(CCPA0W::MCOA0_PASSIVE_)
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0_(self) -> &'a mut W {
        self.variant(CCPA0W::INTERNAL_MCOA0_)
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
#[doc = "Possible values of the field `CCPB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB0R {
    #[doc = "MCOB0 passive."]
    MCOB0_PASSIVE_,
    #[doc = "MCOB0 tracks internal MCOA0."]
    MCOB0_TRACKS_INTERNA,
}
impl crate::ToBits<bool> for CCPB0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPB0R::MCOB0_PASSIVE_ => false,
            CCPB0R::MCOB0_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPB0_R = crate::FR<bool, CCPB0R>;
impl CCPB0_R {
    #[doc = "Checks if the value of the field is `MCOB0_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob0_passive_(&self) -> bool {
        *self == CCPB0R::MCOB0_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB0_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob0_tracks_interna(&self) -> bool {
        *self == CCPB0R::MCOB0_TRACKS_INTERNA
    }
}
#[doc = "Values that can be written to the field `CCPB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB0W {
    #[doc = "MCOB0 passive."]
    MCOB0_PASSIVE_,
    #[doc = "MCOB0 tracks internal MCOA0."]
    MCOB0_TRACKS_INTERNA,
}
impl CCPB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPB0W::MCOB0_PASSIVE_ => false,
            CCPB0W::MCOB0_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPB0W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPB0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn mcob0_passive_(self) -> &'a mut W {
        self.variant(CCPB0W::MCOB0_PASSIVE_)
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob0_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB0W::MCOB0_TRACKS_INTERNA)
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
#[doc = "Possible values of the field `CCPA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA1R {
    #[doc = "MCOA1 passive."]
    MCOA1_PASSIVE_,
    #[doc = "MCOA1 tracks internal MCOA0."]
    MCOA1_TRACKS_INTERNA,
}
impl crate::ToBits<bool> for CCPA1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPA1R::MCOA1_PASSIVE_ => false,
            CCPA1R::MCOA1_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPA1_R = crate::FR<bool, CCPA1R>;
impl CCPA1_R {
    #[doc = "Checks if the value of the field is `MCOA1_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa1_passive_(&self) -> bool {
        *self == CCPA1R::MCOA1_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOA1_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcoa1_tracks_interna(&self) -> bool {
        *self == CCPA1R::MCOA1_TRACKS_INTERNA
    }
}
#[doc = "Values that can be written to the field `CCPA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA1W {
    #[doc = "MCOA1 passive."]
    MCOA1_PASSIVE_,
    #[doc = "MCOA1 tracks internal MCOA0."]
    MCOA1_TRACKS_INTERNA,
}
impl CCPA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPA1W::MCOA1_PASSIVE_ => false,
            CCPA1W::MCOA1_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPA1W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPA1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn mcoa1_passive_(self) -> &'a mut W {
        self.variant(CCPA1W::MCOA1_PASSIVE_)
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa1_tracks_interna(self) -> &'a mut W {
        self.variant(CCPA1W::MCOA1_TRACKS_INTERNA)
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
#[doc = "Possible values of the field `CCPB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB1R {
    #[doc = "MCOB1 passive."]
    MCOB1_PASSIVE_,
    #[doc = "MCOB1 tracks internal MCOA0."]
    MCOB1_TRACKS_INTERNA,
}
impl crate::ToBits<bool> for CCPB1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPB1R::MCOB1_PASSIVE_ => false,
            CCPB1R::MCOB1_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPB1_R = crate::FR<bool, CCPB1R>;
impl CCPB1_R {
    #[doc = "Checks if the value of the field is `MCOB1_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob1_passive_(&self) -> bool {
        *self == CCPB1R::MCOB1_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB1_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob1_tracks_interna(&self) -> bool {
        *self == CCPB1R::MCOB1_TRACKS_INTERNA
    }
}
#[doc = "Values that can be written to the field `CCPB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB1W {
    #[doc = "MCOB1 passive."]
    MCOB1_PASSIVE_,
    #[doc = "MCOB1 tracks internal MCOA0."]
    MCOB1_TRACKS_INTERNA,
}
impl CCPB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPB1W::MCOB1_PASSIVE_ => false,
            CCPB1W::MCOB1_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPB1W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPB1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn mcob1_passive_(self) -> &'a mut W {
        self.variant(CCPB1W::MCOB1_PASSIVE_)
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob1_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB1W::MCOB1_TRACKS_INTERNA)
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
#[doc = "Possible values of the field `CCPA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA2R {
    #[doc = "MCOA2 passive."]
    MCOA2_PASSIVE_,
    #[doc = "MCOA2 tracks internal MCOA0."]
    MCOA2_TRACKS_INTERNA,
}
impl crate::ToBits<bool> for CCPA2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPA2R::MCOA2_PASSIVE_ => false,
            CCPA2R::MCOA2_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPA2_R = crate::FR<bool, CCPA2R>;
impl CCPA2_R {
    #[doc = "Checks if the value of the field is `MCOA2_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa2_passive_(&self) -> bool {
        *self == CCPA2R::MCOA2_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOA2_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcoa2_tracks_interna(&self) -> bool {
        *self == CCPA2R::MCOA2_TRACKS_INTERNA
    }
}
#[doc = "Values that can be written to the field `CCPA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA2W {
    #[doc = "MCOA2 passive."]
    MCOA2_PASSIVE_,
    #[doc = "MCOA2 tracks internal MCOA0."]
    MCOA2_TRACKS_INTERNA,
}
impl CCPA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPA2W::MCOA2_PASSIVE_ => false,
            CCPA2W::MCOA2_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPA2W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPA2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn mcoa2_passive_(self) -> &'a mut W {
        self.variant(CCPA2W::MCOA2_PASSIVE_)
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa2_tracks_interna(self) -> &'a mut W {
        self.variant(CCPA2W::MCOA2_TRACKS_INTERNA)
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
#[doc = "Possible values of the field `CCPB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB2R {
    #[doc = "MCOB2 passive."]
    MCOB2_PASSIVE_,
    #[doc = "MCOB2 tracks internal MCOA0."]
    MCOB2_TRACKS_INTERNA,
}
impl crate::ToBits<bool> for CCPB2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCPB2R::MCOB2_PASSIVE_ => false,
            CCPB2R::MCOB2_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCPB2_R = crate::FR<bool, CCPB2R>;
impl CCPB2_R {
    #[doc = "Checks if the value of the field is `MCOB2_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob2_passive_(&self) -> bool {
        *self == CCPB2R::MCOB2_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB2_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob2_tracks_interna(&self) -> bool {
        *self == CCPB2R::MCOB2_TRACKS_INTERNA
    }
}
#[doc = "Values that can be written to the field `CCPB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB2W {
    #[doc = "MCOB2 passive."]
    MCOB2_PASSIVE_,
    #[doc = "MCOB2 tracks internal MCOA0."]
    MCOB2_TRACKS_INTERNA,
}
impl CCPB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPB2W::MCOB2_PASSIVE_ => false,
            CCPB2W::MCOB2_TRACKS_INTERNA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCPB2W<'a> {
    w: &'a mut W,
}
impl<'a> _CCPB2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn mcob2_passive_(self) -> &'a mut W {
        self.variant(CCPB2W::MCOB2_PASSIVE_)
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob2_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB2W::MCOB2_TRACKS_INTERNA)
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
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&self) -> CCPA0_R {
        CCPA0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&self) -> CCPB0_R {
        CCPB0_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&self) -> CCPA1_R {
        CCPA1_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&self) -> CCPB1_R {
        CCPB1_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&self) -> CCPA2_R {
        CCPA2_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&self) -> CCPB2_R {
        CCPB2_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&mut self) -> _CCPA0W {
        _CCPA0W { w: self }
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&mut self) -> _CCPB0W {
        _CCPB0W { w: self }
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&mut self) -> _CCPA1W {
        _CCPA1W { w: self }
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&mut self) -> _CCPB1W {
        _CCPB1W { w: self }
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&mut self) -> _CCPA2W {
        _CCPA2W { w: self }
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&mut self) -> _CCPB2W {
        _CCPB2W { w: self }
    }
}
