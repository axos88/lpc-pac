#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAO {
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
        0x87e1
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `WORDWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDWIDTHR {
    #[doc = "8-bit data"]
    _8_BIT_DATA,
    #[doc = "16-bit data"]
    _16_BIT_DATA,
    #[doc = "32-bit data"]
    _32_BIT_DATA,
}
impl crate::ToBits<u8> for WORDWIDTHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WORDWIDTHR::_8_BIT_DATA => 0,
            WORDWIDTHR::_16_BIT_DATA => 1,
            WORDWIDTHR::_32_BIT_DATA => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WORDWIDTH_R = crate::FR<u8, WORDWIDTHR>;
impl WORDWIDTH_R {
    #[doc = "Checks if the value of the field is `_8_BIT_DATA`"]
    #[inline(always)]
    pub fn is_8_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_8_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_16_BIT_DATA`"]
    #[inline(always)]
    pub fn is_16_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_16_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_32_BIT_DATA`"]
    #[inline(always)]
    pub fn is_32_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_32_BIT_DATA
    }
}
#[doc = "Values that can be written to the field `WORDWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDWIDTHW {
    #[doc = "8-bit data"]
    _8_BIT_DATA,
    #[doc = "16-bit data"]
    _16_BIT_DATA,
    #[doc = "32-bit data"]
    _32_BIT_DATA,
}
impl WORDWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WORDWIDTHW::_8_BIT_DATA => 0,
            WORDWIDTHW::_16_BIT_DATA => 1,
            WORDWIDTHW::_32_BIT_DATA => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WORDWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDWIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_8_BIT_DATA)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_16_BIT_DATA)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_32_BIT_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MONO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MONOW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOW<'a> {
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
pub type STOP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
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
pub type RESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
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
pub type WS_SEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _WS_SELW<'a> {
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
pub type WS_HALFPERIOD_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _WS_HALFPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _WS_HALFPERIODW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MUTE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MUTEW<'a> {
    w: &'a mut W,
}
impl<'a> _MUTEW<'a> {
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
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WORDWIDTH_R {
        WORDWIDTH_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WS_SEL_R {
        WS_SEL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WS_HALFPERIOD_R {
        WS_HALFPERIOD_R::new(((self.bits() >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> _WORDWIDTHW {
        _WORDWIDTHW { w: self }
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&mut self) -> _MONOW {
        _MONOW { w: self }
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&mut self) -> _WS_SELW {
        _WS_SELW { w: self }
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&mut self) -> _WS_HALFPERIODW {
        _WS_HALFPERIODW { w: self }
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&mut self) -> _MUTEW {
        _MUTEW { w: self }
    }
}
