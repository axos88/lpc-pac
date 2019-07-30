#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMMAND {
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
pub type TXENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENABLEW<'a> {
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
pub type REGRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _REGRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _REGRESETW<'a> {
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
pub type TXRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRESETW<'a> {
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
pub type RXRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRESETW<'a> {
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
pub type PASSRUNTFRAME_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PASSRUNTFRAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _PASSRUNTFRAMEW<'a> {
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
pub type PASSRXFILTER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PASSRXFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _PASSRXFILTERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TXFLOWCONTROL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXFLOWCONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLOWCONTROLW<'a> {
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
pub type RMII_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RMIIW<'a> {
    w: &'a mut W,
}
impl<'a> _RMIIW<'a> {
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
pub type FULLDUPLEX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FULLDUPLEXW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLDUPLEXW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&self) -> TXENABLE_R {
        TXENABLE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&self) -> REGRESET_R {
        REGRESET_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&self) -> TXRESET_R {
        TXRESET_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&self) -> RXRESET_R {
        RXRESET_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&self) -> PASSRUNTFRAME_R {
        PASSRUNTFRAME_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&self) -> PASSRXFILTER_R {
        PASSRXFILTER_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&self) -> TXFLOWCONTROL_R {
        TXFLOWCONTROL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> _RXENABLEW {
        _RXENABLEW { w: self }
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&mut self) -> _TXENABLEW {
        _TXENABLEW { w: self }
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&mut self) -> _REGRESETW {
        _REGRESETW { w: self }
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&mut self) -> _TXRESETW {
        _TXRESETW { w: self }
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&mut self) -> _RXRESETW {
        _RXRESETW { w: self }
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&mut self) -> _PASSRUNTFRAMEW {
        _PASSRUNTFRAMEW { w: self }
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&mut self) -> _PASSRXFILTERW {
        _PASSRXFILTERW { w: self }
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&mut self) -> _TXFLOWCONTROLW {
        _TXFLOWCONTROLW { w: self }
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&mut self) -> _RMIIW {
        _RMIIW { w: self }
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> _FULLDUPLEXW {
        _FULLDUPLEXW { w: self }
    }
}
