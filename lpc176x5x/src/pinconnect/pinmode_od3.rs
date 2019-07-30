#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD3 {
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
#[doc = "Possible values of the field `P3_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25ODR {
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P3_25ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P3_25ODR::NORMAL => false,
            P3_25ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P3_25OD_R = crate::FR<bool, P3_25ODR>;
impl P3_25OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P3_25ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P3_25ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P3_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25ODW {
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P3_25ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P3_25ODW::NORMAL => false,
            P3_25ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P3_25ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P3_25ODW::NORMAL)
    }
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P3_25ODW::OPEN_DRAIN)
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
#[doc = r"Reader of the field"]
pub type P3_26OD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P3_26ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26ODW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&self) -> P3_25OD_R {
        P3_25OD_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&self) -> P3_26OD_R {
        P3_26OD_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&mut self) -> _P3_25ODW {
        _P3_25ODW { w: self }
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&mut self) -> _P3_26ODW {
        _P3_26ODW { w: self }
    }
}
