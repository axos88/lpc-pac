#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTCLEAR {
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
#[doc = r"Proxy"]
pub struct _RXOVERRUNINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _RXERRORINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRORINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _RXFINISHEDINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFINISHEDINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _RXDONEINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDONEINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _TXUNDERRUNINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _TXERRORINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRORINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _TXFINISHEDINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFINISHEDINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _TXDONEINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDONEINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _SOFTINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTINTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _WAKEUPINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPINTCLRW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxoverrunintclr(&mut self) -> _RXOVERRUNINTCLRW {
        _RXOVERRUNINTCLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxerrorintclr(&mut self) -> _RXERRORINTCLRW {
        _RXERRORINTCLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxfinishedintclr(&mut self) -> _RXFINISHEDINTCLRW {
        _RXFINISHEDINTCLRW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn rxdoneintclr(&mut self) -> _RXDONEINTCLRW {
        _RXDONEINTCLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txunderrunintclr(&mut self) -> _TXUNDERRUNINTCLRW {
        _TXUNDERRUNINTCLRW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txerrorintclr(&mut self) -> _TXERRORINTCLRW {
        _TXERRORINTCLRW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txfinishedintclr(&mut self) -> _TXFINISHEDINTCLRW {
        _TXFINISHEDINTCLRW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn txdoneintclr(&mut self) -> _TXDONEINTCLRW {
        _TXDONEINTCLRW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn softintclr(&mut self) -> _SOFTINTCLRW {
        _SOFTINTCLRW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    pub fn wakeupintclr(&mut self) -> _WAKEUPINTCLRW {
        _WAKEUPINTCLRW { w: self }
    }
}
