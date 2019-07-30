#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTMODE {
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
#[doc = "Possible values of the field `EXTMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE0R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl crate::ToBits<bool> for EXTMODE0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTMODE0R::LEVEL_SENSITIVE => false,
            EXTMODE0R::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTMODE0_R = crate::FR<bool, EXTMODE0R>;
impl EXTMODE0_R {
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE0R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE0R::EDGE_SENSITIVE
    }
}
#[doc = "Values that can be written to the field `EXTMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE0W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE0W::LEVEL_SENSITIVE => false,
            EXTMODE0W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTMODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0W::EDGE_SENSITIVE)
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
#[doc = "Possible values of the field `EXTMODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE1R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl crate::ToBits<bool> for EXTMODE1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTMODE1R::LEVEL_SENSITIVE => false,
            EXTMODE1R::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTMODE1_R = crate::FR<bool, EXTMODE1R>;
impl EXTMODE1_R {
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE1R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE1R::EDGE_SENSITIVE
    }
}
#[doc = "Values that can be written to the field `EXTMODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE1W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE1W::LEVEL_SENSITIVE => false,
            EXTMODE1W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTMODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1W::EDGE_SENSITIVE)
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
#[doc = "Possible values of the field `EXTMODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE2R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl crate::ToBits<bool> for EXTMODE2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTMODE2R::LEVEL_SENSITIVE => false,
            EXTMODE2R::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTMODE2_R = crate::FR<bool, EXTMODE2R>;
impl EXTMODE2_R {
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE2R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE2R::EDGE_SENSITIVE
    }
}
#[doc = "Values that can be written to the field `EXTMODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE2W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE2W::LEVEL_SENSITIVE => false,
            EXTMODE2W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTMODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2W::EDGE_SENSITIVE)
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
#[doc = "Possible values of the field `EXTMODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE3R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl crate::ToBits<bool> for EXTMODE3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTMODE3R::LEVEL_SENSITIVE => false,
            EXTMODE3R::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTMODE3_R = crate::FR<bool, EXTMODE3R>;
impl EXTMODE3_R {
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE3R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE3R::EDGE_SENSITIVE
    }
}
#[doc = "Values that can be written to the field `EXTMODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE3W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE3W::LEVEL_SENSITIVE => false,
            EXTMODE3W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTMODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3W::EDGE_SENSITIVE)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&self) -> EXTMODE0_R {
        EXTMODE0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&self) -> EXTMODE1_R {
        EXTMODE1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&self) -> EXTMODE2_R {
        EXTMODE2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&self) -> EXTMODE3_R {
        EXTMODE3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&mut self) -> _EXTMODE0W {
        _EXTMODE0W { w: self }
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&mut self) -> _EXTMODE1W {
        _EXTMODE1W { w: self }
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&mut self) -> _EXTMODE2W {
        _EXTMODE2W { w: self }
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&mut self) -> _EXTMODE3W {
        _EXTMODE3W { w: self }
    }
}
