#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL9 {
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
#[doc = "Possible values of the field `P4_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28R {
    #[doc = "GPIO P4.28"]
    GPIO_P4,
    #[doc = "RX_MCLK"]
    RX_MCLK,
    #[doc = "MAT2.0"]
    MAT2,
    #[doc = "TXD3"]
    TXD3,
}
impl crate::ToBits<u8> for P4_28R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P4_28R::GPIO_P4 => 0,
            P4_28R::RX_MCLK => 1,
            P4_28R::MAT2 => 2,
            P4_28R::TXD3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P4_28_R = crate::FR<u8, P4_28R>;
impl P4_28_R {
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_28R::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `RX_MCLK`"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == P4_28R::RX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_28R::MAT2
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P4_28R::TXD3
    }
}
#[doc = "Values that can be written to the field `P4_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28W {
    #[doc = "GPIO P4.28"]
    GPIO_P4,
    #[doc = "RX_MCLK"]
    RX_MCLK,
    #[doc = "MAT2.0"]
    MAT2,
    #[doc = "TXD3"]
    TXD3,
}
impl P4_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_28W::GPIO_P4 => 0,
            P4_28W::RX_MCLK => 1,
            P4_28W::MAT2 => 2,
            P4_28W::TXD3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P4_28W<'a> {
    w: &'a mut W,
}
impl<'a> _P4_28W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_28W::GPIO_P4)
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut W {
        self.variant(P4_28W::RX_MCLK)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_28W::MAT2)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P4_28W::TXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P4_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29R {
    #[doc = "GPIO P4.29"]
    GPIO_P4,
    #[doc = "TX_MCLK"]
    TX_MCLK,
    #[doc = "MAT2.1"]
    MAT2,
    #[doc = "RXD3"]
    RXD3,
}
impl crate::ToBits<u8> for P4_29R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P4_29R::GPIO_P4 => 0,
            P4_29R::TX_MCLK => 1,
            P4_29R::MAT2 => 2,
            P4_29R::RXD3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P4_29_R = crate::FR<u8, P4_29R>;
impl P4_29_R {
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_29R::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `TX_MCLK`"]
    #[inline(always)]
    pub fn is_tx_mclk(&self) -> bool {
        *self == P4_29R::TX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_29R::MAT2
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P4_29R::RXD3
    }
}
#[doc = "Values that can be written to the field `P4_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29W {
    #[doc = "GPIO P4.29"]
    GPIO_P4,
    #[doc = "TX_MCLK"]
    TX_MCLK,
    #[doc = "MAT2.1"]
    MAT2,
    #[doc = "RXD3"]
    RXD3,
}
impl P4_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_29W::GPIO_P4 => 0,
            P4_29W::TX_MCLK => 1,
            P4_29W::MAT2 => 2,
            P4_29W::RXD3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P4_29W<'a> {
    w: &'a mut W,
}
impl<'a> _P4_29W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_29W::GPIO_P4)
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn tx_mclk(self) -> &'a mut W {
        self.variant(P4_29W::TX_MCLK)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_29W::MAT2)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P4_29W::RXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&self) -> P4_28_R {
        P4_28_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&self) -> P4_29_R {
        P4_29_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&mut self) -> _P4_28W {
        _P4_28W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&mut self) -> _P4_29W {
        _P4_29W { w: self }
    }
}
