#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
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
#[doc = "Values that can be written to the field `FIFOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOENW {
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    DISABLED,
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    ENABLED,
}
impl FIFOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOENW::DISABLED => false,
            FIFOENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FIFOENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOENW::DISABLED)
    }
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOENW::ENABLED)
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
#[doc = "Values that can be written to the field `RXFIFORES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFORESW {
    #[doc = "No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl RXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFORESW::NO_IMPACT => false,
            RXFIFORESW::CLEAR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFORESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(RXFIFORESW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFORESW::CLEAR)
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
#[doc = "Values that can be written to the field `TXFIFORES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFORESW {
    #[doc = "No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl TXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFIFORESW::NO_IMPACT => false,
            TXFIFORESW::CLEAR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFORESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(TXFIFORESW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFORESW::CLEAR)
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
#[doc = "Values that can be written to the field `RXTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTLW {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C,
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C,
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C,
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_,
}
impl RXTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTLW::TRIGGER_LEVEL_0_1_C => 0,
            RXTLW::TRIGGER_LEVEL_1_4_C => 1,
            RXTLW::TRIGGER_LEVEL_2_8_C => 2,
            RXTLW::TRIGGER_LEVEL_3_14_ => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_3_14_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
    #[doc = "Bit 0 - FIFO enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> _FIFOENW {
        _FIFOENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    pub fn rxfifores(&mut self) -> _RXFIFORESW {
        _RXFIFORESW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    pub fn txfifores(&mut self) -> _TXFIFORESW {
        _TXFIFORESW { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtl(&mut self) -> _RXTLW {
        _RXTLW { w: self }
    }
}
