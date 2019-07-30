#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXMODE {
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
#[doc = "Possible values of the field `RXCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCLKSELR {
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI,
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S,
}
impl crate::ToBits<u8> for RXCLKSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RXCLKSELR::SELECT_THE_RX_FRACTI => 0,
            RXCLKSELR::SELECT_THE_TX_MCLK_S => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXCLKSEL_R = crate::FR<u8, RXCLKSELR>;
impl RXCLKSEL_R {
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_FRACTI`"]
    #[inline(always)]
    pub fn is_select_the_rx_fracti(&self) -> bool {
        *self == RXCLKSELR::SELECT_THE_RX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_MCLK_S`"]
    #[inline(always)]
    pub fn is_select_the_tx_mclk_s(&self) -> bool {
        *self == RXCLKSELR::SELECT_THE_TX_MCLK_S
    }
}
#[doc = "Values that can be written to the field `RXCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCLKSELW {
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI,
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S,
}
impl RXCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXCLKSELW::SELECT_THE_RX_FRACTI => 0,
            RXCLKSELW::SELECT_THE_TX_MCLK_S => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCLKSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_rx_fracti(self) -> &'a mut W {
        self.variant(RXCLKSELW::SELECT_THE_RX_FRACTI)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_tx_mclk_s(self) -> &'a mut W {
        self.variant(RXCLKSELW::SELECT_THE_TX_MCLK_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RX4PIN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RX4PINW<'a> {
    w: &'a mut W,
}
impl<'a> _RX4PINW<'a> {
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
pub type RXMCENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXMCENAW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMCENAW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&self) -> RXCLKSEL_R {
        RXCLKSEL_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&self) -> RX4PIN_R {
        RX4PIN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&self) -> RXMCENA_R {
        RXMCENA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&mut self) -> _RXCLKSELW {
        _RXCLKSELW { w: self }
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&mut self) -> _RX4PINW {
        _RX4PINW { w: self }
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&mut self) -> _RXMCENAW {
        _RXMCENAW { w: self }
    }
}
