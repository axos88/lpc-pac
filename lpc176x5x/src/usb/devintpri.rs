#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVINTPRI {
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
#[doc = "Values that can be written to the field `FRAME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMEW {
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    LP,
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    HP,
}
impl FRAMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAMEW::LP => false,
            FRAMEW::HP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FRAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(FRAMEW::LP)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(FRAMEW::HP)
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
#[doc = "Values that can be written to the field `EP_FAST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_FASTW {
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    LP,
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    HP,
}
impl EP_FASTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EP_FASTW::LP => false,
            EP_FASTW::HP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EP_FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_FASTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_FASTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(EP_FASTW::LP)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(EP_FASTW::HP)
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline(always)]
    pub fn frame(&mut self) -> _FRAMEW {
        _FRAMEW { w: self }
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline(always)]
    pub fn ep_fast(&mut self) -> _EP_FASTW {
        _EP_FASTW { w: self }
    }
}
