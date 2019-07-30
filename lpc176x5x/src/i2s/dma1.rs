#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA1 {
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
pub type RX_DMA1_ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RX_DMA1_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DMA1_ENABLEW<'a> {
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
pub type TX_DMA1_ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TX_DMA1_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DMA1_ENABLEW<'a> {
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
pub type RX_DEPTH_DMA1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RX_DEPTH_DMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DEPTH_DMA1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TX_DEPTH_DMA1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TX_DEPTH_DMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DEPTH_DMA1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&self) -> RX_DMA1_ENABLE_R {
        RX_DMA1_ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&self) -> TX_DMA1_ENABLE_R {
        TX_DMA1_ENABLE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&self) -> RX_DEPTH_DMA1_R {
        RX_DEPTH_DMA1_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&self) -> TX_DEPTH_DMA1_R {
        TX_DEPTH_DMA1_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&mut self) -> _RX_DMA1_ENABLEW {
        _RX_DMA1_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&mut self) -> _TX_DMA1_ENABLEW {
        _TX_DMA1_ENABLEW { w: self }
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&mut self) -> _RX_DEPTH_DMA1W {
        _RX_DEPTH_DMA1W { w: self }
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&mut self) -> _TX_DEPTH_DMA1W {
        _TX_DEPTH_DMA1W { w: self }
    }
}
