#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL2 {
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
#[doc = "Possible values of the field `P1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_0R {
    #[doc = "GPIO P1.0"]
    GPIO_P1,
    #[doc = "ENET_TXD0"]
    ENET_TXD0,
}
impl crate::ToBits<u8> for P1_0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_0R::GPIO_P1 => 0,
            P1_0R::ENET_TXD0 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_0_R = crate::FR<u8, P1_0R>;
impl P1_0_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_0R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD0`"]
    #[inline(always)]
    pub fn is_enet_txd0(&self) -> bool {
        *self == P1_0R::ENET_TXD0
    }
}
#[doc = "Values that can be written to the field `P1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_0W {
    #[doc = "GPIO P1.0"]
    GPIO_P1,
    #[doc = "ENET_TXD0"]
    ENET_TXD0,
}
impl P1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_0W::GPIO_P1 => 0,
            P1_0W::ENET_TXD0 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_0W::GPIO_P1)
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn enet_txd0(self) -> &'a mut W {
        self.variant(P1_0W::ENET_TXD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_1R {
    #[doc = "GPIO P1.1"]
    GPIO_P1,
    #[doc = "ENET_TXD1"]
    ENET_TXD1,
}
impl crate::ToBits<u8> for P1_1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_1R::GPIO_P1 => 0,
            P1_1R::ENET_TXD1 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_1_R = crate::FR<u8, P1_1R>;
impl P1_1_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_1R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD1`"]
    #[inline(always)]
    pub fn is_enet_txd1(&self) -> bool {
        *self == P1_1R::ENET_TXD1
    }
}
#[doc = "Values that can be written to the field `P1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_1W {
    #[doc = "GPIO P1.1"]
    GPIO_P1,
    #[doc = "ENET_TXD1"]
    ENET_TXD1,
}
impl P1_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_1W::GPIO_P1 => 0,
            P1_1W::ENET_TXD1 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_1W::GPIO_P1)
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn enet_txd1(self) -> &'a mut W {
        self.variant(P1_1W::ENET_TXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_4R {
    #[doc = "GPIO P1.4."]
    GPIO_P1,
    #[doc = "ENET_TX_EN"]
    ENET_TX_EN,
}
impl crate::ToBits<u8> for P1_4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_4R::GPIO_P1 => 0,
            P1_4R::ENET_TX_EN => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_4_R = crate::FR<u8, P1_4R>;
impl P1_4_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_4R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TX_EN`"]
    #[inline(always)]
    pub fn is_enet_tx_en(&self) -> bool {
        *self == P1_4R::ENET_TX_EN
    }
}
#[doc = "Values that can be written to the field `P1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_4W {
    #[doc = "GPIO P1.4."]
    GPIO_P1,
    #[doc = "ENET_TX_EN"]
    ENET_TX_EN,
}
impl P1_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_4W::GPIO_P1 => 0,
            P1_4W::ENET_TX_EN => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_4W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_4W::GPIO_P1)
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn enet_tx_en(self) -> &'a mut W {
        self.variant(P1_4W::ENET_TX_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P1_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_8R {
    #[doc = "GPIO P1.8."]
    GPIO_P1,
    #[doc = "ENET_CRS"]
    ENET_CRS,
}
impl crate::ToBits<u8> for P1_8R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_8R::GPIO_P1 => 0,
            P1_8R::ENET_CRS => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_8_R = crate::FR<u8, P1_8R>;
impl P1_8_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_8R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_CRS`"]
    #[inline(always)]
    pub fn is_enet_crs(&self) -> bool {
        *self == P1_8R::ENET_CRS
    }
}
#[doc = "Values that can be written to the field `P1_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_8W {
    #[doc = "GPIO P1.8."]
    GPIO_P1,
    #[doc = "ENET_CRS"]
    ENET_CRS,
}
impl P1_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_8W::GPIO_P1 => 0,
            P1_8W::ENET_CRS => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_8W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_8W::GPIO_P1)
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn enet_crs(self) -> &'a mut W {
        self.variant(P1_8W::ENET_CRS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P1_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_9R {
    #[doc = "GPIO Port 1.9"]
    GPIO_PORT_1,
    #[doc = "ENET_RXD0"]
    ENET_RXD0,
}
impl crate::ToBits<u8> for P1_9R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_9R::GPIO_PORT_1 => 0,
            P1_9R::ENET_RXD0 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_9_R = crate::FR<u8, P1_9R>;
impl P1_9_R {
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_9R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD0`"]
    #[inline(always)]
    pub fn is_enet_rxd0(&self) -> bool {
        *self == P1_9R::ENET_RXD0
    }
}
#[doc = "Values that can be written to the field `P1_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_9W {
    #[doc = "GPIO Port 1.9"]
    GPIO_PORT_1,
    #[doc = "ENET_RXD0"]
    ENET_RXD0,
}
impl P1_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_9W::GPIO_PORT_1 => 0,
            P1_9W::ENET_RXD0 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_9W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_9W::GPIO_PORT_1)
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn enet_rxd0(self) -> &'a mut W {
        self.variant(P1_9W::ENET_RXD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P1_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10R {
    #[doc = "GPIO P1.10"]
    GPIO_P1,
    #[doc = "ENET_RXD1"]
    ENET_RXD1,
}
impl crate::ToBits<u8> for P1_10R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_10R::GPIO_P1 => 0,
            P1_10R::ENET_RXD1 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_10_R = crate::FR<u8, P1_10R>;
impl P1_10_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_10R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD1`"]
    #[inline(always)]
    pub fn is_enet_rxd1(&self) -> bool {
        *self == P1_10R::ENET_RXD1
    }
}
#[doc = "Values that can be written to the field `P1_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10W {
    #[doc = "GPIO P1.10"]
    GPIO_P1,
    #[doc = "ENET_RXD1"]
    ENET_RXD1,
}
impl P1_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_10W::GPIO_P1 => 0,
            P1_10W::ENET_RXD1 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_10W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_10W::GPIO_P1)
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn enet_rxd1(self) -> &'a mut W {
        self.variant(P1_10W::ENET_RXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P1_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14R {
    #[doc = "GPIO P1.14"]
    GPIO_P1,
    #[doc = "ENET_RX_ER"]
    ENET_RX_ER,
}
impl crate::ToBits<u8> for P1_14R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_14R::GPIO_P1 => 0,
            P1_14R::ENET_RX_ER => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_14_R = crate::FR<u8, P1_14R>;
impl P1_14_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_14R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RX_ER`"]
    #[inline(always)]
    pub fn is_enet_rx_er(&self) -> bool {
        *self == P1_14R::ENET_RX_ER
    }
}
#[doc = "Values that can be written to the field `P1_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14W {
    #[doc = "GPIO P1.14"]
    GPIO_P1,
    #[doc = "ENET_RX_ER"]
    ENET_RX_ER,
}
impl P1_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_14W::GPIO_P1 => 0,
            P1_14W::ENET_RX_ER => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_14W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_14W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_14W::GPIO_P1)
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn enet_rx_er(self) -> &'a mut W {
        self.variant(P1_14W::ENET_RX_ER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P1_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15R {
    #[doc = "GPIO P1.15"]
    GPIO_P1,
    #[doc = "ENET_REF_CLK"]
    ENET_REF_CLK,
}
impl crate::ToBits<u8> for P1_15R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_15R::GPIO_P1 => 0,
            P1_15R::ENET_REF_CLK => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_15_R = crate::FR<u8, P1_15R>;
impl P1_15_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_15R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_REF_CLK`"]
    #[inline(always)]
    pub fn is_enet_ref_clk(&self) -> bool {
        *self == P1_15R::ENET_REF_CLK
    }
}
#[doc = "Values that can be written to the field `P1_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15W {
    #[doc = "GPIO P1.15"]
    GPIO_P1,
    #[doc = "ENET_REF_CLK"]
    ENET_REF_CLK,
}
impl P1_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_15W::GPIO_P1 => 0,
            P1_15W::ENET_REF_CLK => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_15W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_15W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_15W::GPIO_P1)
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn enet_ref_clk(self) -> &'a mut W {
        self.variant(P1_15W::ENET_REF_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_0(&self) -> P1_0_R {
        P1_0_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_1(&self) -> P1_1_R {
        P1_1_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_4(&self) -> P1_4_R {
        P1_4_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_8(&self) -> P1_8_R {
        P1_8_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_9(&self) -> P1_9_R {
        P1_9_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&self) -> P1_10_R {
        P1_10_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&self) -> P1_14_R {
        P1_14_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&self) -> P1_15_R {
        P1_15_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_0(&mut self) -> _P1_0W {
        _P1_0W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_1(&mut self) -> _P1_1W {
        _P1_1W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_4(&mut self) -> _P1_4W {
        _P1_4W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_8(&mut self) -> _P1_8W {
        _P1_8W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_9(&mut self) -> _P1_9W {
        _P1_9W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&mut self) -> _P1_10W {
        _P1_10W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&mut self) -> _P1_14W {
        _P1_14W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&mut self) -> _P1_15W {
        _P1_15W { w: self }
    }
}
