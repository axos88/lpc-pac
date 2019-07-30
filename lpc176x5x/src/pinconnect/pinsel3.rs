#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL3 {
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
#[doc = "Possible values of the field `P1_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16R {
    #[doc = "GPIO P1.16"]
    GPIO_P1,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl crate::ToBits<u8> for P1_16R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_16R::GPIO_P1 => 0,
            P1_16R::ENET_MDC => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_16_R = crate::FR<u8, P1_16R>;
impl P1_16_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_16R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDC`"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P1_16R::ENET_MDC
    }
}
#[doc = "Values that can be written to the field `P1_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16W {
    #[doc = "GPIO P1.16"]
    GPIO_P1,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl P1_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_16W::GPIO_P1 => 0,
            P1_16W::ENET_MDC => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_16W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_16W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_16W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_16W::GPIO_P1)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut W {
        self.variant(P1_16W::ENET_MDC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P1_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17R {
    #[doc = "GPIO P1.17"]
    GPIO_P1,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl crate::ToBits<u8> for P1_17R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_17R::GPIO_P1 => 0,
            P1_17R::ENET_MDIO => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_17_R = crate::FR<u8, P1_17R>;
impl P1_17_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_17R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDIO`"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P1_17R::ENET_MDIO
    }
}
#[doc = "Values that can be written to the field `P1_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17W {
    #[doc = "GPIO P1.17"]
    GPIO_P1,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl P1_17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_17W::GPIO_P1 => 0,
            P1_17W::ENET_MDIO => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_17W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_17W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_17W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_17W::GPIO_P1)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut W {
        self.variant(P1_17W::ENET_MDIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P1_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18R {
    #[doc = "GPIO P1.18"]
    GPIO_P1,
    #[doc = "USB_UP_LED"]
    USB_UP_LED,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "CAP1.0"]
    CAP1,
}
impl crate::ToBits<u8> for P1_18R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_18R::GPIO_P1 => 0,
            P1_18R::USB_UP_LED => 1,
            P1_18R::PWM1 => 2,
            P1_18R::CAP1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_18_R = crate::FR<u8, P1_18R>;
impl P1_18_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_18R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `USB_UP_LED`"]
    #[inline(always)]
    pub fn is_usb_up_led(&self) -> bool {
        *self == P1_18R::USB_UP_LED
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_18R::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_18R::CAP1
    }
}
#[doc = "Values that can be written to the field `P1_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18W {
    #[doc = "GPIO P1.18"]
    GPIO_P1,
    #[doc = "USB_UP_LED"]
    USB_UP_LED,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "CAP1.0"]
    CAP1,
}
impl P1_18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_18W::GPIO_P1 => 0,
            P1_18W::USB_UP_LED => 1,
            P1_18W::PWM1 => 2,
            P1_18W::CAP1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_18W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_18W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_18W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_18W::GPIO_P1)
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn usb_up_led(self) -> &'a mut W {
        self.variant(P1_18W::USB_UP_LED)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_18W::PWM1)
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_18W::CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P1_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19R {
    #[doc = "GPIO P1.19."]
    GPIO_P1,
    #[doc = "MCOA0"]
    MCOA0,
    #[doc = "USB_PPWR"]
    USB_PPWR,
    #[doc = "CAP1.1"]
    CAP1,
}
impl crate::ToBits<u8> for P1_19R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_19R::GPIO_P1 => 0,
            P1_19R::MCOA0 => 1,
            P1_19R::USB_PPWR => 2,
            P1_19R::CAP1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_19_R = crate::FR<u8, P1_19R>;
impl P1_19_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_19R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA0`"]
    #[inline(always)]
    pub fn is_mcoa0(&self) -> bool {
        *self == P1_19R::MCOA0
    }
    #[doc = "Checks if the value of the field is `USB_PPWR`"]
    #[inline(always)]
    pub fn is_usb_ppwr(&self) -> bool {
        *self == P1_19R::USB_PPWR
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_19R::CAP1
    }
}
#[doc = "Values that can be written to the field `P1_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19W {
    #[doc = "GPIO P1.19."]
    GPIO_P1,
    #[doc = "MCOA0"]
    MCOA0,
    #[doc = "USB_PPWR"]
    USB_PPWR,
    #[doc = "CAP1.1"]
    CAP1,
}
impl P1_19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_19W::GPIO_P1 => 0,
            P1_19W::MCOA0 => 1,
            P1_19W::USB_PPWR => 2,
            P1_19W::CAP1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_19W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_19W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_19W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_19W::GPIO_P1)
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn mcoa0(self) -> &'a mut W {
        self.variant(P1_19W::MCOA0)
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn usb_ppwr(self) -> &'a mut W {
        self.variant(P1_19W::USB_PPWR)
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_19W::CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P1_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20R {
    #[doc = "GPIO P1.20."]
    GPIO_P1,
    #[doc = "MCI0"]
    MCI0,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "SCK0"]
    SCK0,
}
impl crate::ToBits<u8> for P1_20R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_20R::GPIO_P1 => 0,
            P1_20R::MCI0 => 1,
            P1_20R::PWM1 => 2,
            P1_20R::SCK0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_20_R = crate::FR<u8, P1_20R>;
impl P1_20_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_20R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI0`"]
    #[inline(always)]
    pub fn is_mci0(&self) -> bool {
        *self == P1_20R::MCI0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_20R::PWM1
    }
    #[doc = "Checks if the value of the field is `SCK0`"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P1_20R::SCK0
    }
}
#[doc = "Values that can be written to the field `P1_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20W {
    #[doc = "GPIO P1.20."]
    GPIO_P1,
    #[doc = "MCI0"]
    MCI0,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "SCK0"]
    SCK0,
}
impl P1_20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_20W::GPIO_P1 => 0,
            P1_20W::MCI0 => 1,
            P1_20W::PWM1 => 2,
            P1_20W::SCK0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_20W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_20W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_20W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_20W::GPIO_P1)
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn mci0(self) -> &'a mut W {
        self.variant(P1_20W::MCI0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_20W::PWM1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut W {
        self.variant(P1_20W::SCK0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P1_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21R {
    #[doc = "GPIO P1.21."]
    GPIO_P1,
    #[doc = "MCABORT"]
    MCABORT,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "SSEL0"]
    SSEL0,
}
impl crate::ToBits<u8> for P1_21R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_21R::GPIO_P1 => 0,
            P1_21R::MCABORT => 1,
            P1_21R::PWM1 => 2,
            P1_21R::SSEL0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_21_R = crate::FR<u8, P1_21R>;
impl P1_21_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_21R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCABORT`"]
    #[inline(always)]
    pub fn is_mcabort(&self) -> bool {
        *self == P1_21R::MCABORT
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_21R::PWM1
    }
    #[doc = "Checks if the value of the field is `SSEL0`"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P1_21R::SSEL0
    }
}
#[doc = "Values that can be written to the field `P1_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21W {
    #[doc = "GPIO P1.21."]
    GPIO_P1,
    #[doc = "MCABORT"]
    MCABORT,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "SSEL0"]
    SSEL0,
}
impl P1_21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_21W::GPIO_P1 => 0,
            P1_21W::MCABORT => 1,
            P1_21W::PWM1 => 2,
            P1_21W::SSEL0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_21W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_21W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_21W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_21W::GPIO_P1)
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn mcabort(self) -> &'a mut W {
        self.variant(P1_21W::MCABORT)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_21W::PWM1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut W {
        self.variant(P1_21W::SSEL0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P1_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22R {
    #[doc = "GPIO P1.22."]
    GPIO_P1,
    #[doc = "MCOB0"]
    MCOB0,
    #[doc = "USB_PWRD"]
    USB_PWRD,
    #[doc = "MAT1.0"]
    MAT1,
}
impl crate::ToBits<u8> for P1_22R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_22R::GPIO_P1 => 0,
            P1_22R::MCOB0 => 1,
            P1_22R::USB_PWRD => 2,
            P1_22R::MAT1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_22_R = crate::FR<u8, P1_22R>;
impl P1_22_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_22R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB0`"]
    #[inline(always)]
    pub fn is_mcob0(&self) -> bool {
        *self == P1_22R::MCOB0
    }
    #[doc = "Checks if the value of the field is `USB_PWRD`"]
    #[inline(always)]
    pub fn is_usb_pwrd(&self) -> bool {
        *self == P1_22R::USB_PWRD
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_22R::MAT1
    }
}
#[doc = "Values that can be written to the field `P1_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22W {
    #[doc = "GPIO P1.22."]
    GPIO_P1,
    #[doc = "MCOB0"]
    MCOB0,
    #[doc = "USB_PWRD"]
    USB_PWRD,
    #[doc = "MAT1.0"]
    MAT1,
}
impl P1_22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_22W::GPIO_P1 => 0,
            P1_22W::MCOB0 => 1,
            P1_22W::USB_PWRD => 2,
            P1_22W::MAT1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_22W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_22W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_22W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_22W::GPIO_P1)
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn mcob0(self) -> &'a mut W {
        self.variant(P1_22W::MCOB0)
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn usb_pwrd(self) -> &'a mut W {
        self.variant(P1_22W::USB_PWRD)
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_22W::MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P1_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23R {
    #[doc = "GPIO P1.23."]
    GPIO_P1,
    #[doc = "MCI1"]
    MCI1,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "MISO0"]
    MISO0,
}
impl crate::ToBits<u8> for P1_23R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_23R::GPIO_P1 => 0,
            P1_23R::MCI1 => 1,
            P1_23R::PWM1 => 2,
            P1_23R::MISO0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_23_R = crate::FR<u8, P1_23R>;
impl P1_23_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_23R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI1`"]
    #[inline(always)]
    pub fn is_mci1(&self) -> bool {
        *self == P1_23R::MCI1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_23R::PWM1
    }
    #[doc = "Checks if the value of the field is `MISO0`"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P1_23R::MISO0
    }
}
#[doc = "Values that can be written to the field `P1_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23W {
    #[doc = "GPIO P1.23."]
    GPIO_P1,
    #[doc = "MCI1"]
    MCI1,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "MISO0"]
    MISO0,
}
impl P1_23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_23W::GPIO_P1 => 0,
            P1_23W::MCI1 => 1,
            P1_23W::PWM1 => 2,
            P1_23W::MISO0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_23W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_23W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_23W::GPIO_P1)
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn mci1(self) -> &'a mut W {
        self.variant(P1_23W::MCI1)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_23W::PWM1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut W {
        self.variant(P1_23W::MISO0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P1_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24R {
    #[doc = "GPIO P1.24."]
    GPIO_P1,
    #[doc = "MCI2"]
    MCI2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "MOSI0"]
    MOSI0,
}
impl crate::ToBits<u8> for P1_24R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_24R::GPIO_P1 => 0,
            P1_24R::MCI2 => 1,
            P1_24R::PWM1 => 2,
            P1_24R::MOSI0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_24_R = crate::FR<u8, P1_24R>;
impl P1_24_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_24R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI2`"]
    #[inline(always)]
    pub fn is_mci2(&self) -> bool {
        *self == P1_24R::MCI2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_24R::PWM1
    }
    #[doc = "Checks if the value of the field is `MOSI0`"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P1_24R::MOSI0
    }
}
#[doc = "Values that can be written to the field `P1_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24W {
    #[doc = "GPIO P1.24."]
    GPIO_P1,
    #[doc = "MCI2"]
    MCI2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "MOSI0"]
    MOSI0,
}
impl P1_24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_24W::GPIO_P1 => 0,
            P1_24W::MCI2 => 1,
            P1_24W::PWM1 => 2,
            P1_24W::MOSI0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_24W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_24W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_24W::GPIO_P1)
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn mci2(self) -> &'a mut W {
        self.variant(P1_24W::MCI2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_24W::PWM1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut W {
        self.variant(P1_24W::MOSI0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P1_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25R {
    #[doc = "GPIO P1.25"]
    GPIO_P1,
    #[doc = "MCOA1"]
    MCOA1,
    #[doc = "MAT1.1"]
    MAT1,
}
impl crate::ToBits<u8> for P1_25R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_25R::GPIO_P1 => 0,
            P1_25R::MCOA1 => 1,
            P1_25R::MAT1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_25_R = crate::FR<u8, P1_25R>;
impl P1_25_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_25R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA1`"]
    #[inline(always)]
    pub fn is_mcoa1(&self) -> bool {
        *self == P1_25R::MCOA1
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_25R::MAT1
    }
}
#[doc = "Values that can be written to the field `P1_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25W {
    #[doc = "GPIO P1.25"]
    GPIO_P1,
    #[doc = "MCOA1"]
    MCOA1,
    #[doc = "MAT1.1"]
    MAT1,
}
impl P1_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_25W::GPIO_P1 => 0,
            P1_25W::MCOA1 => 1,
            P1_25W::MAT1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_25W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_25W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_25W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_25W::GPIO_P1)
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn mcoa1(self) -> &'a mut W {
        self.variant(P1_25W::MCOA1)
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_25W::MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P1_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26R {
    #[doc = "GPIO P1.26"]
    GPIO_P1,
    #[doc = "MCOB1"]
    MCOB1,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "CAP0.0"]
    CAP0,
}
impl crate::ToBits<u8> for P1_26R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_26R::GPIO_P1 => 0,
            P1_26R::MCOB1 => 1,
            P1_26R::PWM1 => 2,
            P1_26R::CAP0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_26_R = crate::FR<u8, P1_26R>;
impl P1_26_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_26R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB1`"]
    #[inline(always)]
    pub fn is_mcob1(&self) -> bool {
        *self == P1_26R::MCOB1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_26R::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_26R::CAP0
    }
}
#[doc = "Values that can be written to the field `P1_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26W {
    #[doc = "GPIO P1.26"]
    GPIO_P1,
    #[doc = "MCOB1"]
    MCOB1,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "CAP0.0"]
    CAP0,
}
impl P1_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_26W::GPIO_P1 => 0,
            P1_26W::MCOB1 => 1,
            P1_26W::PWM1 => 2,
            P1_26W::CAP0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_26W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_26W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_26W::GPIO_P1)
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn mcob1(self) -> &'a mut W {
        self.variant(P1_26W::MCOB1)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_26W::PWM1)
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_26W::CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P1_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27R {
    #[doc = "GPIO P1.27"]
    GPIO_P1,
    #[doc = "CLKOUT"]
    CLKOUT,
    #[doc = "USB_OVRCR"]
    USB_OVRCR,
    #[doc = "CAP0.1"]
    CAP0,
}
impl crate::ToBits<u8> for P1_27R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_27R::GPIO_P1 => 0,
            P1_27R::CLKOUT => 1,
            P1_27R::USB_OVRCR => 2,
            P1_27R::CAP0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_27_R = crate::FR<u8, P1_27R>;
impl P1_27_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_27R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == P1_27R::CLKOUT
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR`"]
    #[inline(always)]
    pub fn is_usb_ovrcr(&self) -> bool {
        *self == P1_27R::USB_OVRCR
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_27R::CAP0
    }
}
#[doc = "Values that can be written to the field `P1_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27W {
    #[doc = "GPIO P1.27"]
    GPIO_P1,
    #[doc = "CLKOUT"]
    CLKOUT,
    #[doc = "USB_OVRCR"]
    USB_OVRCR,
    #[doc = "CAP0.1"]
    CAP0,
}
impl P1_27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_27W::GPIO_P1 => 0,
            P1_27W::CLKOUT => 1,
            P1_27W::USB_OVRCR => 2,
            P1_27W::CAP0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_27W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_27W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_27W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_27W::GPIO_P1)
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(P1_27W::CLKOUT)
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn usb_ovrcr(self) -> &'a mut W {
        self.variant(P1_27W::USB_OVRCR)
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_27W::CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P1_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28R {
    #[doc = "GPIO P1.28"]
    GPIO_P1,
    #[doc = "MCOA2"]
    MCOA2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "MAT0.0"]
    MAT0,
}
impl crate::ToBits<u8> for P1_28R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_28R::GPIO_P1 => 0,
            P1_28R::MCOA2 => 1,
            P1_28R::PCAP1 => 2,
            P1_28R::MAT0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_28_R = crate::FR<u8, P1_28R>;
impl P1_28_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_28R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA2`"]
    #[inline(always)]
    pub fn is_mcoa2(&self) -> bool {
        *self == P1_28R::MCOA2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_28R::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_28R::MAT0
    }
}
#[doc = "Values that can be written to the field `P1_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28W {
    #[doc = "GPIO P1.28"]
    GPIO_P1,
    #[doc = "MCOA2"]
    MCOA2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "MAT0.0"]
    MAT0,
}
impl P1_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_28W::GPIO_P1 => 0,
            P1_28W::MCOA2 => 1,
            P1_28W::PCAP1 => 2,
            P1_28W::MAT0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_28W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_28W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_28W::GPIO_P1)
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn mcoa2(self) -> &'a mut W {
        self.variant(P1_28W::MCOA2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_28W::PCAP1)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_28W::MAT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P1_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29R {
    #[doc = "GPIO P1.29"]
    GPIO_P1,
    #[doc = "MCOB2"]
    MCOB2,
    #[doc = "PCAP1.1"]
    PCAP1,
    #[doc = "MAT0.1"]
    MAT0,
}
impl crate::ToBits<u8> for P1_29R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_29R::GPIO_P1 => 0,
            P1_29R::MCOB2 => 1,
            P1_29R::PCAP1 => 2,
            P1_29R::MAT0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_29_R = crate::FR<u8, P1_29R>;
impl P1_29_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_29R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB2`"]
    #[inline(always)]
    pub fn is_mcob2(&self) -> bool {
        *self == P1_29R::MCOB2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_29R::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_29R::MAT0
    }
}
#[doc = "Values that can be written to the field `P1_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29W {
    #[doc = "GPIO P1.29"]
    GPIO_P1,
    #[doc = "MCOB2"]
    MCOB2,
    #[doc = "PCAP1.1"]
    PCAP1,
    #[doc = "MAT0.1"]
    MAT0,
}
impl P1_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_29W::GPIO_P1 => 0,
            P1_29W::MCOB2 => 1,
            P1_29W::PCAP1 => 2,
            P1_29W::MAT0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_29W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_29W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_29W::GPIO_P1)
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn mcob2(self) -> &'a mut W {
        self.variant(P1_29W::MCOB2)
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_29W::PCAP1)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_29W::MAT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `P1_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30R {
    #[doc = "GPIO P1.30"]
    GPIO_P1,
    #[doc = "VBUS"]
    VBUS,
    #[doc = "AD0.4"]
    AD0,
}
impl crate::ToBits<u8> for P1_30R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_30R::GPIO_P1 => 0,
            P1_30R::VBUS => 2,
            P1_30R::AD0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_30_R = crate::FR<u8, P1_30R>;
impl P1_30_R {
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_30R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `VBUS`"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == P1_30R::VBUS
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_30R::AD0
    }
}
#[doc = "Values that can be written to the field `P1_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30W {
    #[doc = "GPIO P1.30"]
    GPIO_P1,
    #[doc = "VBUS"]
    VBUS,
    #[doc = "AD0.4"]
    AD0,
}
impl P1_30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_30W::GPIO_P1 => 0,
            P1_30W::VBUS => 2,
            P1_30W::AD0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_30W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_30W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_30W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_30W::GPIO_P1)
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut W {
        self.variant(P1_30W::VBUS)
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_30W::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `P1_31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31R {
    #[doc = "GPIO Port 1.31"]
    GPIO_PORT_1,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "AD0.5"]
    AD0,
}
impl crate::ToBits<u8> for P1_31R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_31R::GPIO_PORT_1 => 0,
            P1_31R::SCK1 => 2,
            P1_31R::AD0 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_31_R = crate::FR<u8, P1_31R>;
impl P1_31_R {
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_31R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `SCK1`"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P1_31R::SCK1
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_31R::AD0
    }
}
#[doc = "Values that can be written to the field `P1_31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31W {
    #[doc = "GPIO Port 1.31"]
    GPIO_PORT_1,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "AD0.5"]
    AD0,
}
impl P1_31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_31W::GPIO_PORT_1 => 0,
            P1_31W::SCK1 => 2,
            P1_31W::AD0 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_31W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_31W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_31W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_31W::GPIO_PORT_1)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut W {
        self.variant(P1_31W::SCK1)
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_31W::AD0)
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
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&self) -> P1_16_R {
        P1_16_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&self) -> P1_17_R {
        P1_17_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&self) -> P1_18_R {
        P1_18_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&self) -> P1_19_R {
        P1_19_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&self) -> P1_20_R {
        P1_20_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&self) -> P1_21_R {
        P1_21_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&self) -> P1_22_R {
        P1_22_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&self) -> P1_23_R {
        P1_23_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&self) -> P1_24_R {
        P1_24_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&self) -> P1_25_R {
        P1_25_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&self) -> P1_26_R {
        P1_26_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&self) -> P1_27_R {
        P1_27_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&self) -> P1_28_R {
        P1_28_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&self) -> P1_29_R {
        P1_29_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&self) -> P1_30_R {
        P1_30_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&self) -> P1_31_R {
        P1_31_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&mut self) -> _P1_16W {
        _P1_16W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&mut self) -> _P1_17W {
        _P1_17W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&mut self) -> _P1_18W {
        _P1_18W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&mut self) -> _P1_19W {
        _P1_19W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&mut self) -> _P1_20W {
        _P1_20W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&mut self) -> _P1_21W {
        _P1_21W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&mut self) -> _P1_22W {
        _P1_22W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&mut self) -> _P1_23W {
        _P1_23W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&mut self) -> _P1_24W {
        _P1_24W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&mut self) -> _P1_25W {
        _P1_25W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&mut self) -> _P1_26W {
        _P1_26W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&mut self) -> _P1_27W {
        _P1_27W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&mut self) -> _P1_28W {
        _P1_28W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&mut self) -> _P1_29W {
        _P1_29W { w: self }
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&mut self) -> _P1_30W {
        _P1_30W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&mut self) -> _P1_31W {
        _P1_31W { w: self }
    }
}
