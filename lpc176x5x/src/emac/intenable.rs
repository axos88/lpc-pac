#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENABLE {
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
pub type RXOVERRUNINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXOVERRUNINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNINTENW<'a> {
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
pub type RXERRORINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXERRORINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRORINTENW<'a> {
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
pub type RXFINISHEDINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXFINISHEDINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFINISHEDINTENW<'a> {
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
pub type RXDONEINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXDONEINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDONEINTENW<'a> {
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
pub type TXUNDERRUNINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXUNDERRUNINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNINTENW<'a> {
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
pub type TXERRORINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXERRORINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRORINTENW<'a> {
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
pub type TXFINISHEDINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXFINISHEDINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFINISHEDINTENW<'a> {
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
pub type TXDONEINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXDONEINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDONEINTENW<'a> {
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
pub type SOFTINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTINTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WAKEUPINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAKEUPINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPINTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&self) -> RXOVERRUNINTEN_R {
        RXOVERRUNINTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&self) -> RXERRORINTEN_R {
        RXERRORINTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&self) -> RXFINISHEDINTEN_R {
        RXFINISHEDINTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&self) -> RXDONEINTEN_R {
        RXDONEINTEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&self) -> TXUNDERRUNINTEN_R {
        TXUNDERRUNINTEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&self) -> TXERRORINTEN_R {
        TXERRORINTEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&self) -> TXFINISHEDINTEN_R {
        TXFINISHEDINTEN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&self) -> TXDONEINTEN_R {
        TXDONEINTEN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&self) -> SOFTINTEN_R {
        SOFTINTEN_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&self) -> WAKEUPINTEN_R {
        WAKEUPINTEN_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&mut self) -> _RXOVERRUNINTENW {
        _RXOVERRUNINTENW { w: self }
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&mut self) -> _RXERRORINTENW {
        _RXERRORINTENW { w: self }
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&mut self) -> _RXFINISHEDINTENW {
        _RXFINISHEDINTENW { w: self }
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&mut self) -> _RXDONEINTENW {
        _RXDONEINTENW { w: self }
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&mut self) -> _TXUNDERRUNINTENW {
        _TXUNDERRUNINTENW { w: self }
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&mut self) -> _TXERRORINTENW {
        _TXERRORINTENW { w: self }
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&mut self) -> _TXFINISHEDINTENW {
        _TXFINISHEDINTENW { w: self }
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&mut self) -> _TXDONEINTENW {
        _TXDONEINTENW { w: self }
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&mut self) -> _SOFTINTENW {
        _SOFTINTENW { w: self }
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&mut self) -> _WAKEUPINTENW {
        _WAKEUPINTENW { w: self }
    }
}
