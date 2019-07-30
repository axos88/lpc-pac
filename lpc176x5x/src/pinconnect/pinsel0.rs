#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL0 {
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
#[doc = "Possible values of the field `P0_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_0R {
    #[doc = "GPIO P0.0"]
    GPIO_P0,
    #[doc = "RD1"]
    RD1,
    #[doc = "TXD3"]
    TXD3,
    #[doc = "SDA1"]
    SDA1,
}
impl crate::ToBits<u8> for P0_0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_0R::GPIO_P0 => 0,
            P0_0R::RD1 => 1,
            P0_0R::TXD3 => 2,
            P0_0R::SDA1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_0_R = crate::FR<u8, P0_0R>;
impl P0_0_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_0R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RD1`"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_0R::RD1
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_0R::TXD3
    }
    #[doc = "Checks if the value of the field is `SDA1`"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_0R::SDA1
    }
}
#[doc = "Values that can be written to the field `P0_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_0W {
    #[doc = "GPIO P0.0"]
    GPIO_P0,
    #[doc = "RD1"]
    RD1,
    #[doc = "TXD3"]
    TXD3,
    #[doc = "SDA1"]
    SDA1,
}
impl P0_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_0W::GPIO_P0 => 0,
            P0_0W::RD1 => 1,
            P0_0W::TXD3 => 2,
            P0_0W::SDA1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_0W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_0W::GPIO_P0)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut W {
        self.variant(P0_0W::RD1)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P0_0W::TXD3)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut W {
        self.variant(P0_0W::SDA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_1R {
    #[doc = "GPIO P0.1"]
    GPIO_P0,
    #[doc = "TD1"]
    TD1,
    #[doc = "RXD3"]
    RXD3,
    #[doc = "SCL1"]
    SCL1,
}
impl crate::ToBits<u8> for P0_1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_1R::GPIO_P0 => 0,
            P0_1R::TD1 => 1,
            P0_1R::RXD3 => 2,
            P0_1R::SCL1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_1_R = crate::FR<u8, P0_1R>;
impl P0_1_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_1R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TD1`"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_1R::TD1
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_1R::RXD3
    }
    #[doc = "Checks if the value of the field is `SCL1`"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_1R::SCL1
    }
}
#[doc = "Values that can be written to the field `P0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_1W {
    #[doc = "GPIO P0.1"]
    GPIO_P0,
    #[doc = "TD1"]
    TD1,
    #[doc = "RXD3"]
    RXD3,
    #[doc = "SCL1"]
    SCL1,
}
impl P0_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_1W::GPIO_P0 => 0,
            P0_1W::TD1 => 1,
            P0_1W::RXD3 => 2,
            P0_1W::SCL1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_1W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_1W::GPIO_P0)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut W {
        self.variant(P0_1W::TD1)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P0_1W::RXD3)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut W {
        self.variant(P0_1W::SCL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P0_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_2R {
    #[doc = "GPIO P0.2"]
    GPIO_P0,
    #[doc = "TXD0"]
    TXD0,
    #[doc = "AD0.7"]
    AD0,
}
impl crate::ToBits<u8> for P0_2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_2R::GPIO_P0 => 0,
            P0_2R::TXD0 => 1,
            P0_2R::AD0 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_2_R = crate::FR<u8, P0_2R>;
impl P0_2_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_2R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD0`"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == P0_2R::TXD0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_2R::AD0
    }
}
#[doc = "Values that can be written to the field `P0_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_2W {
    #[doc = "GPIO P0.2"]
    GPIO_P0,
    #[doc = "TXD0"]
    TXD0,
    #[doc = "AD0.7"]
    AD0,
}
impl P0_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_2W::GPIO_P0 => 0,
            P0_2W::TXD0 => 1,
            P0_2W::AD0 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_2W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_2W::GPIO_P0)
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut W {
        self.variant(P0_2W::TXD0)
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_2W::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P0_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_3R {
    #[doc = "GPIO P0.3."]
    GPIO_P0,
    #[doc = "RXD0"]
    RXD0,
    #[doc = "AD0.6"]
    AD0,
}
impl crate::ToBits<u8> for P0_3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_3R::GPIO_P0 => 0,
            P0_3R::RXD0 => 1,
            P0_3R::AD0 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_3_R = crate::FR<u8, P0_3R>;
impl P0_3_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_3R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD0`"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == P0_3R::RXD0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_3R::AD0
    }
}
#[doc = "Values that can be written to the field `P0_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_3W {
    #[doc = "GPIO P0.3."]
    GPIO_P0,
    #[doc = "RXD0"]
    RXD0,
    #[doc = "AD0.6"]
    AD0,
}
impl P0_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_3W::GPIO_P0 => 0,
            P0_3W::RXD0 => 1,
            P0_3W::AD0 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_3W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_3W::GPIO_P0)
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut W {
        self.variant(P0_3W::RXD0)
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_3W::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P0_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_4R {
    #[doc = "GPIO P0.4."]
    GPIO_P0,
    #[doc = "I2SRX_CLK"]
    I2SRX_CLK,
    #[doc = "RD2"]
    RD2,
    #[doc = "CAP2.0"]
    CAP2,
}
impl crate::ToBits<u8> for P0_4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_4R::GPIO_P0 => 0,
            P0_4R::I2SRX_CLK => 1,
            P0_4R::RD2 => 2,
            P0_4R::CAP2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_4_R = crate::FR<u8, P0_4R>;
impl P0_4_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_4R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_CLK`"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_4R::I2SRX_CLK
    }
    #[doc = "Checks if the value of the field is `RD2`"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P0_4R::RD2
    }
    #[doc = "Checks if the value of the field is `CAP2`"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_4R::CAP2
    }
}
#[doc = "Values that can be written to the field `P0_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_4W {
    #[doc = "GPIO P0.4."]
    GPIO_P0,
    #[doc = "I2SRX_CLK"]
    I2SRX_CLK,
    #[doc = "RD2"]
    RD2,
    #[doc = "CAP2.0"]
    CAP2,
}
impl P0_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_4W::GPIO_P0 => 0,
            P0_4W::I2SRX_CLK => 1,
            P0_4W::RD2 => 2,
            P0_4W::CAP2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_4W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_4W::GPIO_P0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut W {
        self.variant(P0_4W::I2SRX_CLK)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut W {
        self.variant(P0_4W::RD2)
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut W {
        self.variant(P0_4W::CAP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P0_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_5R {
    #[doc = "GPIO P0.5."]
    GPIO_P0,
    #[doc = "I2SRX_WS"]
    I2SRX_WS,
    #[doc = "TD2"]
    TD2,
    #[doc = "CAP2.1"]
    CAP2,
}
impl crate::ToBits<u8> for P0_5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_5R::GPIO_P0 => 0,
            P0_5R::I2SRX_WS => 1,
            P0_5R::TD2 => 2,
            P0_5R::CAP2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_5_R = crate::FR<u8, P0_5R>;
impl P0_5_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_5R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_WS`"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_5R::I2SRX_WS
    }
    #[doc = "Checks if the value of the field is `TD2`"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P0_5R::TD2
    }
    #[doc = "Checks if the value of the field is `CAP2`"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_5R::CAP2
    }
}
#[doc = "Values that can be written to the field `P0_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_5W {
    #[doc = "GPIO P0.5."]
    GPIO_P0,
    #[doc = "I2SRX_WS"]
    I2SRX_WS,
    #[doc = "TD2"]
    TD2,
    #[doc = "CAP2.1"]
    CAP2,
}
impl P0_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_5W::GPIO_P0 => 0,
            P0_5W::I2SRX_WS => 1,
            P0_5W::TD2 => 2,
            P0_5W::CAP2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_5W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_5W::GPIO_P0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut W {
        self.variant(P0_5W::I2SRX_WS)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut W {
        self.variant(P0_5W::TD2)
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut W {
        self.variant(P0_5W::CAP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P0_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_6R {
    #[doc = "GPIO P0.6."]
    GPIO_P0,
    #[doc = "I2SRX_SDA"]
    I2SRX_SDA,
    #[doc = "SSEL1"]
    SSEL1,
    #[doc = "MAT2.0"]
    MAT2,
}
impl crate::ToBits<u8> for P0_6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_6R::GPIO_P0 => 0,
            P0_6R::I2SRX_SDA => 1,
            P0_6R::SSEL1 => 2,
            P0_6R::MAT2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_6_R = crate::FR<u8, P0_6R>;
impl P0_6_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_6R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_SDA`"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_6R::I2SRX_SDA
    }
    #[doc = "Checks if the value of the field is `SSEL1`"]
    #[inline(always)]
    pub fn is_ssel1(&self) -> bool {
        *self == P0_6R::SSEL1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_6R::MAT2
    }
}
#[doc = "Values that can be written to the field `P0_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_6W {
    #[doc = "GPIO P0.6."]
    GPIO_P0,
    #[doc = "I2SRX_SDA"]
    I2SRX_SDA,
    #[doc = "SSEL1"]
    SSEL1,
    #[doc = "MAT2.0"]
    MAT2,
}
impl P0_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_6W::GPIO_P0 => 0,
            P0_6W::I2SRX_SDA => 1,
            P0_6W::SSEL1 => 2,
            P0_6W::MAT2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_6W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_6W::GPIO_P0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut W {
        self.variant(P0_6W::I2SRX_SDA)
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn ssel1(self) -> &'a mut W {
        self.variant(P0_6W::SSEL1)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_6W::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P0_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_7R {
    #[doc = "GPIO P0.7."]
    GPIO_P0,
    #[doc = "I2STX_CLK"]
    I2STX_CLK,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "MAT2.1"]
    MAT2,
}
impl crate::ToBits<u8> for P0_7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_7R::GPIO_P0 => 0,
            P0_7R::I2STX_CLK => 1,
            P0_7R::SCK1 => 2,
            P0_7R::MAT2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_7_R = crate::FR<u8, P0_7R>;
impl P0_7_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_7R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_CLK`"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P0_7R::I2STX_CLK
    }
    #[doc = "Checks if the value of the field is `SCK1`"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P0_7R::SCK1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_7R::MAT2
    }
}
#[doc = "Values that can be written to the field `P0_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_7W {
    #[doc = "GPIO P0.7."]
    GPIO_P0,
    #[doc = "I2STX_CLK"]
    I2STX_CLK,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "MAT2.1"]
    MAT2,
}
impl P0_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_7W::GPIO_P0 => 0,
            P0_7W::I2STX_CLK => 1,
            P0_7W::SCK1 => 2,
            P0_7W::MAT2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_7W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_7W::GPIO_P0)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut W {
        self.variant(P0_7W::I2STX_CLK)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut W {
        self.variant(P0_7W::SCK1)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_7W::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P0_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_8R {
    #[doc = "GPIO P0.8."]
    GPIO_P0,
    #[doc = "I2STX_WS"]
    I2STX_WS,
    #[doc = "MISO1"]
    MISO1,
    #[doc = "MAT2.2"]
    MAT2,
}
impl crate::ToBits<u8> for P0_8R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_8R::GPIO_P0 => 0,
            P0_8R::I2STX_WS => 1,
            P0_8R::MISO1 => 2,
            P0_8R::MAT2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_8_R = crate::FR<u8, P0_8R>;
impl P0_8_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_8R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_WS`"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P0_8R::I2STX_WS
    }
    #[doc = "Checks if the value of the field is `MISO1`"]
    #[inline(always)]
    pub fn is_miso1(&self) -> bool {
        *self == P0_8R::MISO1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_8R::MAT2
    }
}
#[doc = "Values that can be written to the field `P0_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_8W {
    #[doc = "GPIO P0.8."]
    GPIO_P0,
    #[doc = "I2STX_WS"]
    I2STX_WS,
    #[doc = "MISO1"]
    MISO1,
    #[doc = "MAT2.2"]
    MAT2,
}
impl P0_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_8W::GPIO_P0 => 0,
            P0_8W::I2STX_WS => 1,
            P0_8W::MISO1 => 2,
            P0_8W::MAT2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_8W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_8W::GPIO_P0)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut W {
        self.variant(P0_8W::I2STX_WS)
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn miso1(self) -> &'a mut W {
        self.variant(P0_8W::MISO1)
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_8W::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P0_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_9R {
    #[doc = "GPIO P0.9"]
    GPIO_P0,
    #[doc = "I2STX_SDA"]
    I2STX_SDA,
    #[doc = "MOSI1"]
    MOSI1,
    #[doc = "MAT2.3"]
    MAT2,
}
impl crate::ToBits<u8> for P0_9R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_9R::GPIO_P0 => 0,
            P0_9R::I2STX_SDA => 1,
            P0_9R::MOSI1 => 2,
            P0_9R::MAT2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_9_R = crate::FR<u8, P0_9R>;
impl P0_9_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_9R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_SDA`"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P0_9R::I2STX_SDA
    }
    #[doc = "Checks if the value of the field is `MOSI1`"]
    #[inline(always)]
    pub fn is_mosi1(&self) -> bool {
        *self == P0_9R::MOSI1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_9R::MAT2
    }
}
#[doc = "Values that can be written to the field `P0_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_9W {
    #[doc = "GPIO P0.9"]
    GPIO_P0,
    #[doc = "I2STX_SDA"]
    I2STX_SDA,
    #[doc = "MOSI1"]
    MOSI1,
    #[doc = "MAT2.3"]
    MAT2,
}
impl P0_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_9W::GPIO_P0 => 0,
            P0_9W::I2STX_SDA => 1,
            P0_9W::MOSI1 => 2,
            P0_9W::MAT2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_9W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_9W::GPIO_P0)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut W {
        self.variant(P0_9W::I2STX_SDA)
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn mosi1(self) -> &'a mut W {
        self.variant(P0_9W::MOSI1)
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_9W::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P0_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10R {
    #[doc = "GPIO P0.10"]
    GPIO_P0,
    #[doc = "TXD2"]
    TXD2,
    #[doc = "SDA2"]
    SDA2,
    #[doc = "MAT3.0"]
    MAT3,
}
impl crate::ToBits<u8> for P0_10R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_10R::GPIO_P0 => 0,
            P0_10R::TXD2 => 1,
            P0_10R::SDA2 => 2,
            P0_10R::MAT3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_10_R = crate::FR<u8, P0_10R>;
impl P0_10_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_10R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD2`"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P0_10R::TXD2
    }
    #[doc = "Checks if the value of the field is `SDA2`"]
    #[inline(always)]
    pub fn is_sda2(&self) -> bool {
        *self == P0_10R::SDA2
    }
    #[doc = "Checks if the value of the field is `MAT3`"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_10R::MAT3
    }
}
#[doc = "Values that can be written to the field `P0_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10W {
    #[doc = "GPIO P0.10"]
    GPIO_P0,
    #[doc = "TXD2"]
    TXD2,
    #[doc = "SDA2"]
    SDA2,
    #[doc = "MAT3.0"]
    MAT3,
}
impl P0_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_10W::GPIO_P0 => 0,
            P0_10W::TXD2 => 1,
            P0_10W::SDA2 => 2,
            P0_10W::MAT3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_10W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_10W::GPIO_P0)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut W {
        self.variant(P0_10W::TXD2)
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn sda2(self) -> &'a mut W {
        self.variant(P0_10W::SDA2)
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut W {
        self.variant(P0_10W::MAT3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P0_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11R {
    #[doc = "GPIO P0.11"]
    GPIO_P0,
    #[doc = "RXD2"]
    RXD2,
    #[doc = "SCL2"]
    SCL2,
    #[doc = "MAT3.1"]
    MAT3,
}
impl crate::ToBits<u8> for P0_11R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_11R::GPIO_P0 => 0,
            P0_11R::RXD2 => 1,
            P0_11R::SCL2 => 2,
            P0_11R::MAT3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_11_R = crate::FR<u8, P0_11R>;
impl P0_11_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_11R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD2`"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P0_11R::RXD2
    }
    #[doc = "Checks if the value of the field is `SCL2`"]
    #[inline(always)]
    pub fn is_scl2(&self) -> bool {
        *self == P0_11R::SCL2
    }
    #[doc = "Checks if the value of the field is `MAT3`"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_11R::MAT3
    }
}
#[doc = "Values that can be written to the field `P0_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11W {
    #[doc = "GPIO P0.11"]
    GPIO_P0,
    #[doc = "RXD2"]
    RXD2,
    #[doc = "SCL2"]
    SCL2,
    #[doc = "MAT3.1"]
    MAT3,
}
impl P0_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_11W::GPIO_P0 => 0,
            P0_11W::RXD2 => 1,
            P0_11W::SCL2 => 2,
            P0_11W::MAT3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_11W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_11W::GPIO_P0)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut W {
        self.variant(P0_11W::RXD2)
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn scl2(self) -> &'a mut W {
        self.variant(P0_11W::SCL2)
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut W {
        self.variant(P0_11W::MAT3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P0_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15R {
    #[doc = "GPIO P0.15"]
    GPIO_P0,
    #[doc = "TXD1"]
    TXD1,
    #[doc = "SCK0"]
    SCK0,
    #[doc = "SCK"]
    SCK,
}
impl crate::ToBits<u8> for P0_15R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_15R::GPIO_P0 => 0,
            P0_15R::TXD1 => 1,
            P0_15R::SCK0 => 2,
            P0_15R::SCK => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_15_R = crate::FR<u8, P0_15R>;
impl P0_15_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_15R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD1`"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P0_15R::TXD1
    }
    #[doc = "Checks if the value of the field is `SCK0`"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P0_15R::SCK0
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == P0_15R::SCK
    }
}
#[doc = "Values that can be written to the field `P0_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15W {
    #[doc = "GPIO P0.15"]
    GPIO_P0,
    #[doc = "TXD1"]
    TXD1,
    #[doc = "SCK0"]
    SCK0,
    #[doc = "SCK"]
    SCK,
}
impl P0_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_15W::GPIO_P0 => 0,
            P0_15W::TXD1 => 1,
            P0_15W::SCK0 => 2,
            P0_15W::SCK => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_15W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_15W::GPIO_P0)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut W {
        self.variant(P0_15W::TXD1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut W {
        self.variant(P0_15W::SCK0)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(P0_15W::SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_0(&self) -> P0_0_R {
        P0_0_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_1(&self) -> P0_1_R {
        P0_1_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_2(&self) -> P0_2_R {
        P0_2_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_3(&self) -> P0_3_R {
        P0_3_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_4(&self) -> P0_4_R {
        P0_4_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_5(&self) -> P0_5_R {
        P0_5_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_6(&self) -> P0_6_R {
        P0_6_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_7(&self) -> P0_7_R {
        P0_7_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_8(&self) -> P0_8_R {
        P0_8_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_9(&self) -> P0_9_R {
        P0_9_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&self) -> P0_10_R {
        P0_10_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&self) -> P0_11_R {
        P0_11_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&self) -> P0_15_R {
        P0_15_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_0(&mut self) -> _P0_0W {
        _P0_0W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_1(&mut self) -> _P0_1W {
        _P0_1W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_2(&mut self) -> _P0_2W {
        _P0_2W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_3(&mut self) -> _P0_3W {
        _P0_3W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_4(&mut self) -> _P0_4W {
        _P0_4W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_5(&mut self) -> _P0_5W {
        _P0_5W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_6(&mut self) -> _P0_6W {
        _P0_6W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_7(&mut self) -> _P0_7W {
        _P0_7W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_8(&mut self) -> _P0_8W {
        _P0_8W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_9(&mut self) -> _P0_9W {
        _P0_9W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&mut self) -> _P0_10W {
        _P0_10W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&mut self) -> _P0_11W {
        _P0_11W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&mut self) -> _P0_15W {
        _P0_15W { w: self }
    }
}
