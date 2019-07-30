#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL1 {
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
#[doc = "Possible values of the field `P0_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16R {
    #[doc = "GPIO P0.16"]
    GPIO_P0,
    #[doc = "RXD1"]
    RXD1,
    #[doc = "SSEL0"]
    SSEL0,
    #[doc = "SSEL"]
    SSEL,
}
impl crate::ToBits<u8> for P0_16R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_16R::GPIO_P0 => 0,
            P0_16R::RXD1 => 1,
            P0_16R::SSEL0 => 2,
            P0_16R::SSEL => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_16_R = crate::FR<u8, P0_16R>;
impl P0_16_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_16R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD1`"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == P0_16R::RXD1
    }
    #[doc = "Checks if the value of the field is `SSEL0`"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P0_16R::SSEL0
    }
    #[doc = "Checks if the value of the field is `SSEL`"]
    #[inline(always)]
    pub fn is_ssel(&self) -> bool {
        *self == P0_16R::SSEL
    }
}
#[doc = "Values that can be written to the field `P0_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16W {
    #[doc = "GPIO P0.16"]
    GPIO_P0,
    #[doc = "RXD1"]
    RXD1,
    #[doc = "SSEL0"]
    SSEL0,
    #[doc = "SSEL"]
    SSEL,
}
impl P0_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_16W::GPIO_P0 => 0,
            P0_16W::RXD1 => 1,
            P0_16W::SSEL0 => 2,
            P0_16W::SSEL => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_16W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_16W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.16"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_16W::GPIO_P0)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut W {
        self.variant(P0_16W::RXD1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut W {
        self.variant(P0_16W::SSEL0)
    }
    #[doc = "SSEL"]
    #[inline(always)]
    pub fn ssel(self) -> &'a mut W {
        self.variant(P0_16W::SSEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P0_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17R {
    #[doc = "GPIO P0.17"]
    GPIO_P0,
    #[doc = "CTS1"]
    CTS1,
    #[doc = "MISO0"]
    MISO0,
    #[doc = "MISO"]
    MISO,
}
impl crate::ToBits<u8> for P0_17R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_17R::GPIO_P0 => 0,
            P0_17R::CTS1 => 1,
            P0_17R::MISO0 => 2,
            P0_17R::MISO => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_17_R = crate::FR<u8, P0_17R>;
impl P0_17_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_17R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `CTS1`"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == P0_17R::CTS1
    }
    #[doc = "Checks if the value of the field is `MISO0`"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P0_17R::MISO0
    }
    #[doc = "Checks if the value of the field is `MISO`"]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        *self == P0_17R::MISO
    }
}
#[doc = "Values that can be written to the field `P0_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17W {
    #[doc = "GPIO P0.17"]
    GPIO_P0,
    #[doc = "CTS1"]
    CTS1,
    #[doc = "MISO0"]
    MISO0,
    #[doc = "MISO"]
    MISO,
}
impl P0_17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_17W::GPIO_P0 => 0,
            P0_17W::CTS1 => 1,
            P0_17W::MISO0 => 2,
            P0_17W::MISO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_17W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_17W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.17"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_17W::GPIO_P0)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut W {
        self.variant(P0_17W::CTS1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut W {
        self.variant(P0_17W::MISO0)
    }
    #[doc = "MISO"]
    #[inline(always)]
    pub fn miso(self) -> &'a mut W {
        self.variant(P0_17W::MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P0_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18R {
    #[doc = "GPIO P0.18"]
    GPIO_P0,
    #[doc = "DCD1"]
    DCD1,
    #[doc = "MOSI0"]
    MOSI0,
    #[doc = "MOSI"]
    MOSI,
}
impl crate::ToBits<u8> for P0_18R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_18R::GPIO_P0 => 0,
            P0_18R::DCD1 => 1,
            P0_18R::MOSI0 => 2,
            P0_18R::MOSI => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_18_R = crate::FR<u8, P0_18R>;
impl P0_18_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_18R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DCD1`"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == P0_18R::DCD1
    }
    #[doc = "Checks if the value of the field is `MOSI0`"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P0_18R::MOSI0
    }
    #[doc = "Checks if the value of the field is `MOSI`"]
    #[inline(always)]
    pub fn is_mosi(&self) -> bool {
        *self == P0_18R::MOSI
    }
}
#[doc = "Values that can be written to the field `P0_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18W {
    #[doc = "GPIO P0.18"]
    GPIO_P0,
    #[doc = "DCD1"]
    DCD1,
    #[doc = "MOSI0"]
    MOSI0,
    #[doc = "MOSI"]
    MOSI,
}
impl P0_18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_18W::GPIO_P0 => 0,
            P0_18W::DCD1 => 1,
            P0_18W::MOSI0 => 2,
            P0_18W::MOSI => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_18W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_18W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.18"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_18W::GPIO_P0)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut W {
        self.variant(P0_18W::DCD1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut W {
        self.variant(P0_18W::MOSI0)
    }
    #[doc = "MOSI"]
    #[inline(always)]
    pub fn mosi(self) -> &'a mut W {
        self.variant(P0_18W::MOSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P0_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19R {
    #[doc = "GPIO P0.19."]
    GPIO_P0,
    #[doc = "DSR1"]
    DSR1,
    #[doc = "SDA1"]
    SDA1,
}
impl crate::ToBits<u8> for P0_19R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_19R::GPIO_P0 => 0,
            P0_19R::DSR1 => 1,
            P0_19R::SDA1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_19_R = crate::FR<u8, P0_19R>;
impl P0_19_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_19R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DSR1`"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == P0_19R::DSR1
    }
    #[doc = "Checks if the value of the field is `SDA1`"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_19R::SDA1
    }
}
#[doc = "Values that can be written to the field `P0_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19W {
    #[doc = "GPIO P0.19."]
    GPIO_P0,
    #[doc = "DSR1"]
    DSR1,
    #[doc = "SDA1"]
    SDA1,
}
impl P0_19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_19W::GPIO_P0 => 0,
            P0_19W::DSR1 => 1,
            P0_19W::SDA1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_19W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_19W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.19."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_19W::GPIO_P0)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut W {
        self.variant(P0_19W::DSR1)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut W {
        self.variant(P0_19W::SDA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P0_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20R {
    #[doc = "GPIO P0.20."]
    GPIO_P0,
    #[doc = "DTR1"]
    DTR1,
    #[doc = "SCL1"]
    SCL1,
}
impl crate::ToBits<u8> for P0_20R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_20R::GPIO_P0 => 0,
            P0_20R::DTR1 => 1,
            P0_20R::SCL1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_20_R = crate::FR<u8, P0_20R>;
impl P0_20_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_20R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DTR1`"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == P0_20R::DTR1
    }
    #[doc = "Checks if the value of the field is `SCL1`"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_20R::SCL1
    }
}
#[doc = "Values that can be written to the field `P0_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20W {
    #[doc = "GPIO P0.20."]
    GPIO_P0,
    #[doc = "DTR1"]
    DTR1,
    #[doc = "SCL1"]
    SCL1,
}
impl P0_20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_20W::GPIO_P0 => 0,
            P0_20W::DTR1 => 1,
            P0_20W::SCL1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_20W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_20W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.20."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_20W::GPIO_P0)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut W {
        self.variant(P0_20W::DTR1)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut W {
        self.variant(P0_20W::SCL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P0_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21R {
    #[doc = "GPIO Port 0.21."]
    GPIO_PORT_0,
    #[doc = "RI1"]
    RI1,
    #[doc = "RD1"]
    RD1,
}
impl crate::ToBits<u8> for P0_21R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_21R::GPIO_PORT_0 => 0,
            P0_21R::RI1 => 1,
            P0_21R::RD1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_21_R = crate::FR<u8, P0_21R>;
impl P0_21_R {
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline(always)]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == P0_21R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `RI1`"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == P0_21R::RI1
    }
    #[doc = "Checks if the value of the field is `RD1`"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_21R::RD1
    }
}
#[doc = "Values that can be written to the field `P0_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21W {
    #[doc = "GPIO Port 0.21."]
    GPIO_PORT_0,
    #[doc = "RI1"]
    RI1,
    #[doc = "RD1"]
    RD1,
}
impl P0_21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_21W::GPIO_PORT_0 => 0,
            P0_21W::RI1 => 1,
            P0_21W::RD1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_21W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_21W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO Port 0.21."]
    #[inline(always)]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(P0_21W::GPIO_PORT_0)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut W {
        self.variant(P0_21W::RI1)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut W {
        self.variant(P0_21W::RD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P0_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22R {
    #[doc = "GPIO P0.22."]
    GPIO_P0,
    #[doc = "RTS1"]
    RTS1,
    #[doc = "TD1"]
    TD1,
}
impl crate::ToBits<u8> for P0_22R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_22R::GPIO_P0 => 0,
            P0_22R::RTS1 => 1,
            P0_22R::TD1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_22_R = crate::FR<u8, P0_22R>;
impl P0_22_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_22R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RTS1`"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == P0_22R::RTS1
    }
    #[doc = "Checks if the value of the field is `TD1`"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_22R::TD1
    }
}
#[doc = "Values that can be written to the field `P0_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22W {
    #[doc = "GPIO P0.22."]
    GPIO_P0,
    #[doc = "RTS1"]
    RTS1,
    #[doc = "TD1"]
    TD1,
}
impl P0_22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_22W::GPIO_P0 => 0,
            P0_22W::RTS1 => 1,
            P0_22W::TD1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_22W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_22W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.22."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_22W::GPIO_P0)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut W {
        self.variant(P0_22W::RTS1)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut W {
        self.variant(P0_22W::TD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P0_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23R {
    #[doc = "GPIO P0.23."]
    GPIO_P0,
    #[doc = "AD0.0"]
    AD0,
    #[doc = "I2SRX_CLK"]
    I2SRX_CLK,
    #[doc = "CAP3.0"]
    CAP3,
}
impl crate::ToBits<u8> for P0_23R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_23R::GPIO_P0 => 0,
            P0_23R::AD0 => 1,
            P0_23R::I2SRX_CLK => 2,
            P0_23R::CAP3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_23_R = crate::FR<u8, P0_23R>;
impl P0_23_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_23R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_23R::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_CLK`"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_23R::I2SRX_CLK
    }
    #[doc = "Checks if the value of the field is `CAP3`"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == P0_23R::CAP3
    }
}
#[doc = "Values that can be written to the field `P0_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23W {
    #[doc = "GPIO P0.23."]
    GPIO_P0,
    #[doc = "AD0.0"]
    AD0,
    #[doc = "I2SRX_CLK"]
    I2SRX_CLK,
    #[doc = "CAP3.0"]
    CAP3,
}
impl P0_23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_23W::GPIO_P0 => 0,
            P0_23W::AD0 => 1,
            P0_23W::I2SRX_CLK => 2,
            P0_23W::CAP3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_23W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.23."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_23W::GPIO_P0)
    }
    #[doc = "AD0.0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_23W::AD0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut W {
        self.variant(P0_23W::I2SRX_CLK)
    }
    #[doc = "CAP3.0"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut W {
        self.variant(P0_23W::CAP3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P0_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24R {
    #[doc = "GPIO P0.24."]
    GPIO_P0,
    #[doc = "AD0.1"]
    AD0,
    #[doc = "I2SRX_WS"]
    I2SRX_WS,
    #[doc = "CAP3.1"]
    CAP3,
}
impl crate::ToBits<u8> for P0_24R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_24R::GPIO_P0 => 0,
            P0_24R::AD0 => 1,
            P0_24R::I2SRX_WS => 2,
            P0_24R::CAP3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_24_R = crate::FR<u8, P0_24R>;
impl P0_24_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_24R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_24R::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_WS`"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_24R::I2SRX_WS
    }
    #[doc = "Checks if the value of the field is `CAP3`"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == P0_24R::CAP3
    }
}
#[doc = "Values that can be written to the field `P0_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24W {
    #[doc = "GPIO P0.24."]
    GPIO_P0,
    #[doc = "AD0.1"]
    AD0,
    #[doc = "I2SRX_WS"]
    I2SRX_WS,
    #[doc = "CAP3.1"]
    CAP3,
}
impl P0_24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_24W::GPIO_P0 => 0,
            P0_24W::AD0 => 1,
            P0_24W::I2SRX_WS => 2,
            P0_24W::CAP3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_24W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.24."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_24W::GPIO_P0)
    }
    #[doc = "AD0.1"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_24W::AD0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut W {
        self.variant(P0_24W::I2SRX_WS)
    }
    #[doc = "CAP3.1"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut W {
        self.variant(P0_24W::CAP3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P0_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25R {
    #[doc = "GPIO P0.25"]
    GPIO_P0,
    #[doc = "AD0.2"]
    AD0,
    #[doc = "I2SRX_SDA"]
    I2SRX_SDA,
    #[doc = "TXD3"]
    TXD3,
}
impl crate::ToBits<u8> for P0_25R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_25R::GPIO_P0 => 0,
            P0_25R::AD0 => 1,
            P0_25R::I2SRX_SDA => 2,
            P0_25R::TXD3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_25_R = crate::FR<u8, P0_25R>;
impl P0_25_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_25R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_25R::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_SDA`"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_25R::I2SRX_SDA
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_25R::TXD3
    }
}
#[doc = "Values that can be written to the field `P0_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25W {
    #[doc = "GPIO P0.25"]
    GPIO_P0,
    #[doc = "AD0.2"]
    AD0,
    #[doc = "I2SRX_SDA"]
    I2SRX_SDA,
    #[doc = "TXD3"]
    TXD3,
}
impl P0_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_25W::GPIO_P0 => 0,
            P0_25W::AD0 => 1,
            P0_25W::I2SRX_SDA => 2,
            P0_25W::TXD3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_25W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_25W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.25"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_25W::GPIO_P0)
    }
    #[doc = "AD0.2"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_25W::AD0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut W {
        self.variant(P0_25W::I2SRX_SDA)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P0_25W::TXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P0_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26R {
    #[doc = "GPIO P0.26"]
    GPIO_P0,
    #[doc = "AD0.3"]
    AD0,
    #[doc = "AOUT"]
    AOUT,
    #[doc = "RXD3"]
    RXD3,
}
impl crate::ToBits<u8> for P0_26R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_26R::GPIO_P0 => 0,
            P0_26R::AD0 => 1,
            P0_26R::AOUT => 2,
            P0_26R::RXD3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_26_R = crate::FR<u8, P0_26R>;
impl P0_26_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_26R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_26R::AD0
    }
    #[doc = "Checks if the value of the field is `AOUT`"]
    #[inline(always)]
    pub fn is_aout(&self) -> bool {
        *self == P0_26R::AOUT
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_26R::RXD3
    }
}
#[doc = "Values that can be written to the field `P0_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26W {
    #[doc = "GPIO P0.26"]
    GPIO_P0,
    #[doc = "AD0.3"]
    AD0,
    #[doc = "AOUT"]
    AOUT,
    #[doc = "RXD3"]
    RXD3,
}
impl P0_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_26W::GPIO_P0 => 0,
            P0_26W::AD0 => 1,
            P0_26W::AOUT => 2,
            P0_26W::RXD3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_26W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P0.26"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_26W::GPIO_P0)
    }
    #[doc = "AD0.3"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_26W::AD0)
    }
    #[doc = "AOUT"]
    #[inline(always)]
    pub fn aout(self) -> &'a mut W {
        self.variant(P0_26W::AOUT)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P0_26W::RXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P0_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_27R {
    #[doc = "GPIO P0.27"]
    GPIO_P0,
    #[doc = "SDA0"]
    SDA0,
    #[doc = "USB_SDA"]
    USB_SDA,
}
impl crate::ToBits<u8> for P0_27R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_27R::GPIO_P0 => 0,
            P0_27R::SDA0 => 1,
            P0_27R::USB_SDA => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_27_R = crate::FR<u8, P0_27R>;
impl P0_27_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_27R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `SDA0`"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == P0_27R::SDA0
    }
    #[doc = "Checks if the value of the field is `USB_SDA`"]
    #[inline(always)]
    pub fn is_usb_sda(&self) -> bool {
        *self == P0_27R::USB_SDA
    }
}
#[doc = "Values that can be written to the field `P0_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_27W {
    #[doc = "GPIO P0.27"]
    GPIO_P0,
    #[doc = "SDA0"]
    SDA0,
    #[doc = "USB_SDA"]
    USB_SDA,
}
impl P0_27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_27W::GPIO_P0 => 0,
            P0_27W::SDA0 => 1,
            P0_27W::USB_SDA => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_27W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_27W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_27W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.27"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_27W::GPIO_P0)
    }
    #[doc = "SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut W {
        self.variant(P0_27W::SDA0)
    }
    #[doc = "USB_SDA"]
    #[inline(always)]
    pub fn usb_sda(self) -> &'a mut W {
        self.variant(P0_27W::USB_SDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P0_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_28R {
    #[doc = "GPIO P0.28"]
    GPIO_P0,
    #[doc = "SCL0"]
    SCL0,
    #[doc = "USB_SCL"]
    USB_SCL,
}
impl crate::ToBits<u8> for P0_28R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_28R::GPIO_P0 => 0,
            P0_28R::SCL0 => 1,
            P0_28R::USB_SCL => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_28_R = crate::FR<u8, P0_28R>;
impl P0_28_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_28R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `SCL0`"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == P0_28R::SCL0
    }
    #[doc = "Checks if the value of the field is `USB_SCL`"]
    #[inline(always)]
    pub fn is_usb_scl(&self) -> bool {
        *self == P0_28R::USB_SCL
    }
}
#[doc = "Values that can be written to the field `P0_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_28W {
    #[doc = "GPIO P0.28"]
    GPIO_P0,
    #[doc = "SCL0"]
    SCL0,
    #[doc = "USB_SCL"]
    USB_SCL,
}
impl P0_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_28W::GPIO_P0 => 0,
            P0_28W::SCL0 => 1,
            P0_28W::USB_SCL => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_28W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_28W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_28W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.28"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_28W::GPIO_P0)
    }
    #[doc = "SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut W {
        self.variant(P0_28W::SCL0)
    }
    #[doc = "USB_SCL"]
    #[inline(always)]
    pub fn usb_scl(self) -> &'a mut W {
        self.variant(P0_28W::USB_SCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P0_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_29R {
    #[doc = "GPIO P0.29"]
    GPIO_P0,
    #[doc = "USB_D+"]
    USB_DP,
}
impl crate::ToBits<u8> for P0_29R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_29R::GPIO_P0 => 0,
            P0_29R::USB_DP => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_29_R = crate::FR<u8, P0_29R>;
impl P0_29_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_29R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `USB_DP`"]
    #[inline(always)]
    pub fn is_usb_dp(&self) -> bool {
        *self == P0_29R::USB_DP
    }
}
#[doc = "Values that can be written to the field `P0_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_29W {
    #[doc = "GPIO P0.29"]
    GPIO_P0,
    #[doc = "USB_D+"]
    USB_DP,
}
impl P0_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_29W::GPIO_P0 => 0,
            P0_29W::USB_DP => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_29W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_29W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_29W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.29"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_29W::GPIO_P0)
    }
    #[doc = "USB_D+"]
    #[inline(always)]
    pub fn usb_dp(self) -> &'a mut W {
        self.variant(P0_29W::USB_DP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `P0_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_30R {
    #[doc = "GPIO P0.30"]
    GPIO_P0,
    #[doc = "USB_D-"]
    USB_DM,
}
impl crate::ToBits<u8> for P0_30R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_30R::GPIO_P0 => 0,
            P0_30R::USB_DM => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_30_R = crate::FR<u8, P0_30R>;
impl P0_30_R {
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_30R::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `USB_DM`"]
    #[inline(always)]
    pub fn is_usb_dm(&self) -> bool {
        *self == P0_30R::USB_DM
    }
}
#[doc = "Values that can be written to the field `P0_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_30W {
    #[doc = "GPIO P0.30"]
    GPIO_P0,
    #[doc = "USB_D-"]
    USB_DM,
}
impl P0_30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_30W::GPIO_P0 => 0,
            P0_30W::USB_DM => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_30W<'a> {
    w: &'a mut W,
}
impl<'a> _P0_30W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_30W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P0.30"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_30W::GPIO_P0)
    }
    #[doc = "USB_D-"]
    #[inline(always)]
    pub fn usb_dm(self) -> &'a mut W {
        self.variant(P0_30W::USB_DM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&self) -> P0_16_R {
        P0_16_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&self) -> P0_17_R {
        P0_17_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&self) -> P0_18_R {
        P0_18_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&self) -> P0_19_R {
        P0_19_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&self) -> P0_20_R {
        P0_20_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&self) -> P0_21_R {
        P0_21_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&self) -> P0_22_R {
        P0_22_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&self) -> P0_23_R {
        P0_23_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&self) -> P0_24_R {
        P0_24_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&self) -> P0_25_R {
        P0_25_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&self) -> P0_26_R {
        P0_26_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&self) -> P0_27_R {
        P0_27_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&self) -> P0_28_R {
        P0_28_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&self) -> P0_29_R {
        P0_29_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&self) -> P0_30_R {
        P0_30_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&mut self) -> _P0_16W {
        _P0_16W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&mut self) -> _P0_17W {
        _P0_17W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&mut self) -> _P0_18W {
        _P0_18W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&mut self) -> _P0_19W {
        _P0_19W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&mut self) -> _P0_20W {
        _P0_20W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&mut self) -> _P0_21W {
        _P0_21W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&mut self) -> _P0_22W {
        _P0_22W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&mut self) -> _P0_23W {
        _P0_23W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&mut self) -> _P0_24W {
        _P0_24W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&mut self) -> _P0_25W {
        _P0_25W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&mut self) -> _P0_26W {
        _P0_26W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&mut self) -> _P0_27W {
        _P0_27W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&mut self) -> _P0_28W {
        _P0_28W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&mut self) -> _P0_29W {
        _P0_29W { w: self }
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&mut self) -> _P0_30W {
        _P0_30W { w: self }
    }
}
