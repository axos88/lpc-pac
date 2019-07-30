#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC1 {
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
        0x8000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type RXENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENABLEW<'a> {
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
pub type PARF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PARFW<'a> {
    w: &'a mut W,
}
impl<'a> _PARFW<'a> {
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
pub type RXFLOWCTRL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXFLOWCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFLOWCTRLW<'a> {
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
pub type TXFLOWCTRL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXFLOWCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLOWCTRLW<'a> {
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
#[doc = r"Reader of the field"]
pub type LOOPBACK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOOPBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPBACKW<'a> {
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
#[doc = r"Reader of the field"]
pub type RESETTX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETTXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETTXW<'a> {
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
#[doc = r"Reader of the field"]
pub type RESETMCSTX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETMCSTXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETMCSTXW<'a> {
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
#[doc = r"Reader of the field"]
pub type RESETRX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETRXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETRXW<'a> {
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
#[doc = r"Reader of the field"]
pub type RESETMCSRX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETMCSRXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETMCSRXW<'a> {
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
#[doc = r"Reader of the field"]
pub type SIMRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SIMRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SIMRESETW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTRESETW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&self) -> RXFLOWCTRL_R {
        RXFLOWCTRL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&self) -> TXFLOWCTRL_R {
        TXFLOWCTRL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&self) -> RESETTX_R {
        RESETTX_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&self) -> RESETMCSTX_R {
        RESETMCSTX_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&self) -> RESETRX_R {
        RESETRX_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&self) -> RESETMCSRX_R {
        RESETMCSRX_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&self) -> SIMRESET_R {
        SIMRESET_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> _RXENABLEW {
        _RXENABLEW { w: self }
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&mut self) -> _PARFW {
        _PARFW { w: self }
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&mut self) -> _RXFLOWCTRLW {
        _RXFLOWCTRLW { w: self }
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&mut self) -> _TXFLOWCTRLW {
        _TXFLOWCTRLW { w: self }
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&mut self) -> _LOOPBACKW {
        _LOOPBACKW { w: self }
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&mut self) -> _RESETTXW {
        _RESETTXW { w: self }
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&mut self) -> _RESETMCSTXW {
        _RESETMCSTXW { w: self }
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&mut self) -> _RESETRXW {
        _RESETRXW { w: self }
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&mut self) -> _RESETMCSRXW {
        _RESETMCSRXW { w: self }
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&mut self) -> _SIMRESETW {
        _SIMRESETW { w: self }
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&mut self) -> _SOFTRESETW {
        _SOFTRESETW { w: self }
    }
}
