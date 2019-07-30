#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBINTST {
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
        0x8000_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type USB_INT_REQ_LP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_INT_REQ_LPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_INT_REQ_LPW<'a> {
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
pub type USB_INT_REQ_HP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_INT_REQ_HPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_INT_REQ_HPW<'a> {
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
pub type USB_INT_REQ_DMA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_INT_REQ_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_INT_REQ_DMAW<'a> {
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
pub type USB_HOST_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_HOST_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_HOST_INTW<'a> {
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
pub type USB_ATX_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_ATX_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ATX_INTW<'a> {
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
pub type USB_OTG_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_OTG_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG_INTW<'a> {
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
pub type USB_I2C_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_I2C_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_I2C_INTW<'a> {
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
pub type USB_NEED_CLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB_NEED_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_NEED_CLKW<'a> {
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
pub type EN_USB_INTS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EN_USB_INTSW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_USB_INTSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&self) -> USB_INT_REQ_LP_R {
        USB_INT_REQ_LP_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&self) -> USB_INT_REQ_HP_R {
        USB_INT_REQ_HP_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&self) -> USB_INT_REQ_DMA_R {
        USB_INT_REQ_DMA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&self) -> USB_HOST_INT_R {
        USB_HOST_INT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&self) -> USB_ATX_INT_R {
        USB_ATX_INT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&self) -> USB_OTG_INT_R {
        USB_OTG_INT_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&self) -> USB_I2C_INT_R {
        USB_I2C_INT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&self) -> USB_NEED_CLK_R {
        USB_NEED_CLK_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&self) -> EN_USB_INTS_R {
        EN_USB_INTS_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&mut self) -> _USB_INT_REQ_LPW {
        _USB_INT_REQ_LPW { w: self }
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&mut self) -> _USB_INT_REQ_HPW {
        _USB_INT_REQ_HPW { w: self }
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&mut self) -> _USB_INT_REQ_DMAW {
        _USB_INT_REQ_DMAW { w: self }
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&mut self) -> _USB_HOST_INTW {
        _USB_HOST_INTW { w: self }
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&mut self) -> _USB_ATX_INTW {
        _USB_ATX_INTW { w: self }
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&mut self) -> _USB_OTG_INTW {
        _USB_OTG_INTW { w: self }
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&mut self) -> _USB_I2C_INTW {
        _USB_I2C_INTW { w: self }
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&mut self) -> _USB_NEED_CLKW {
        _USB_NEED_CLKW { w: self }
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&mut self) -> _EN_USB_INTSW {
        _EN_USB_INTSW { w: self }
    }
}
