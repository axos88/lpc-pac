#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STCTRL {
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
pub type PORT_FUNC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PORT_FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PORT_FUNCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TMR_SCALE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TMR_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_SCALEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TMR_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TMR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_MODEW<'a> {
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
pub type TMR_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TMR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_ENW<'a> {
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
#[doc = r"Reader of the field"]
pub type TMR_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TMR_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_RSTW<'a> {
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
#[doc = r"Reader of the field"]
pub type B_HNP_TRACK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _B_HNP_TRACKW<'a> {
    w: &'a mut W,
}
impl<'a> _B_HNP_TRACKW<'a> {
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
pub type A_HNP_TRACK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _A_HNP_TRACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A_HNP_TRACKW<'a> {
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
pub type PU_REMOVED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PU_REMOVEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PU_REMOVEDW<'a> {
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
pub type TMR_CNT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TMR_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_CNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&self) -> PORT_FUNC_R {
        PORT_FUNC_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&self) -> TMR_SCALE_R {
        TMR_SCALE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&self) -> TMR_RST_R {
        TMR_RST_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&self) -> B_HNP_TRACK_R {
        B_HNP_TRACK_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&self) -> A_HNP_TRACK_R {
        A_HNP_TRACK_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&self) -> PU_REMOVED_R {
        PU_REMOVED_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TMR_CNT_R {
        TMR_CNT_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&mut self) -> _PORT_FUNCW {
        _PORT_FUNCW { w: self }
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&mut self) -> _TMR_SCALEW {
        _TMR_SCALEW { w: self }
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&mut self) -> _TMR_MODEW {
        _TMR_MODEW { w: self }
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> _TMR_ENW {
        _TMR_ENW { w: self }
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&mut self) -> _TMR_RSTW {
        _TMR_RSTW { w: self }
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&mut self) -> _B_HNP_TRACKW {
        _B_HNP_TRACKW { w: self }
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&mut self) -> _A_HNP_TRACKW {
        _A_HNP_TRACKW { w: self }
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&mut self) -> _PU_REMOVEDW {
        _PU_REMOVEDW { w: self }
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&mut self) -> _TMR_CNTW {
        _TMR_CNTW { w: self }
    }
}
