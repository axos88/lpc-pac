#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTPOLAR {
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
#[doc = "Possible values of the field `EXTPOLAR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR0R {
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RISING_EDGE,
}
impl crate::ToBits<bool> for EXTPOLAR0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR0R::FALLING_EDGE => false,
            EXTPOLAR0R::RISING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTPOLAR0_R = crate::FR<bool, EXTPOLAR0R>;
impl EXTPOLAR0_R {
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR0R::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR0R::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `EXTPOLAR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR0W {
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RISING_EDGE,
}
impl EXTPOLAR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR0W::FALLING_EDGE => false,
            EXTPOLAR0W::RISING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTPOLAR0W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTPOLAR0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR0W::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR0W::RISING_EDGE)
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
#[doc = "Possible values of the field `EXTPOLAR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR1R {
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RISING_EDGE,
}
impl crate::ToBits<bool> for EXTPOLAR1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR1R::FALLING_EDGE => false,
            EXTPOLAR1R::RISING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTPOLAR1_R = crate::FR<bool, EXTPOLAR1R>;
impl EXTPOLAR1_R {
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR1R::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR1R::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `EXTPOLAR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR1W {
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RISING_EDGE,
}
impl EXTPOLAR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR1W::FALLING_EDGE => false,
            EXTPOLAR1W::RISING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTPOLAR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTPOLAR1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR1W::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR1W::RISING_EDGE)
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
#[doc = "Possible values of the field `EXTPOLAR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR2R {
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RISING_EDGE,
}
impl crate::ToBits<bool> for EXTPOLAR2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR2R::FALLING_EDGE => false,
            EXTPOLAR2R::RISING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTPOLAR2_R = crate::FR<bool, EXTPOLAR2R>;
impl EXTPOLAR2_R {
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR2R::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR2R::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `EXTPOLAR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR2W {
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RISING_EDGE,
}
impl EXTPOLAR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR2W::FALLING_EDGE => false,
            EXTPOLAR2W::RISING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTPOLAR2W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTPOLAR2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR2W::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR2W::RISING_EDGE)
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
#[doc = "Possible values of the field `EXTPOLAR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR3R {
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RISING_EDGE,
}
impl crate::ToBits<bool> for EXTPOLAR3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR3R::FALLING_EDGE => false,
            EXTPOLAR3R::RISING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTPOLAR3_R = crate::FR<bool, EXTPOLAR3R>;
impl EXTPOLAR3_R {
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR3R::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR3R::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `EXTPOLAR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR3W {
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FALLING_EDGE,
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RISING_EDGE,
}
impl EXTPOLAR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTPOLAR3W::FALLING_EDGE => false,
            EXTPOLAR3W::RISING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTPOLAR3W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTPOLAR3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR3W::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR3W::RISING_EDGE)
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
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&self) -> EXTPOLAR0_R {
        EXTPOLAR0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&self) -> EXTPOLAR1_R {
        EXTPOLAR1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&self) -> EXTPOLAR2_R {
        EXTPOLAR2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&self) -> EXTPOLAR3_R {
        EXTPOLAR3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&mut self) -> _EXTPOLAR0W {
        _EXTPOLAR0W { w: self }
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&mut self) -> _EXTPOLAR1W {
        _EXTPOLAR1W { w: self }
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&mut self) -> _EXTPOLAR2W {
        _EXTPOLAR2W { w: self }
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&mut self) -> _EXTPOLAR3W {
        _EXTPOLAR3W { w: self }
    }
}
