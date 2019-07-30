#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD4 {
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
#[doc = "Possible values of the field `P4_28OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28ODR {
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for P4_28ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P4_28ODR::NORMAL => false,
            P4_28ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P4_28OD_R = crate::FR<bool, P4_28ODR>;
impl P4_28OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P4_28ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P4_28ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P4_28OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28ODW {
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P4_28ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            P4_28ODW::NORMAL => false,
            P4_28ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P4_28ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_28ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P4.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P4_28ODW::NORMAL)
    }
    #[doc = "Open-drain. P4.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P4_28ODW::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P4_29OD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P4_29ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_29ODW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&self) -> P4_28OD_R {
        P4_28OD_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&self) -> P4_29OD_R {
        P4_29OD_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&mut self) -> _P4_28ODW {
        _P4_28ODW { w: self }
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&mut self) -> _P4_29ODW {
        _P4_29ODW { w: self }
    }
}
