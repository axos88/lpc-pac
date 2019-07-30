#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCLKSEL0 {
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
#[doc = "Possible values of the field `PCLK_WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_WDTR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_WDTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_WDTR::CCLK_DIV_4 => 0,
            PCLK_WDTR::CCLK => 1,
            PCLK_WDTR::CCLK_DIV_2 => 2,
            PCLK_WDTR::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_WDT_R = crate::FR<u8, PCLK_WDTR>;
impl PCLK_WDT_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_WDTR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_WDTW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_WDTW::CCLK_DIV_4 => 0,
            PCLK_WDTW::CCLK => 1,
            PCLK_WDTW::CCLK_DIV_2 => 2,
            PCLK_WDTW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_WDTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_WDTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_TIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_TIMER0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER0R::CCLK_DIV_4 => 0,
            PCLK_TIMER0R::CCLK => 1,
            PCLK_TIMER0R::CCLK_DIV_2 => 2,
            PCLK_TIMER0R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_TIMER0_R = crate::FR<u8, PCLK_TIMER0R>;
impl PCLK_TIMER0_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_TIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER0W::CCLK_DIV_4 => 0,
            PCLK_TIMER0W::CCLK => 1,
            PCLK_TIMER0W::CCLK_DIV_2 => 2,
            PCLK_TIMER0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_TIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_TIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_TIMER1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER1R::CCLK_DIV_4 => 0,
            PCLK_TIMER1R::CCLK => 1,
            PCLK_TIMER1R::CCLK_DIV_2 => 2,
            PCLK_TIMER1R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_TIMER1_R = crate::FR<u8, PCLK_TIMER1R>;
impl PCLK_TIMER1_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_TIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER1W::CCLK_DIV_4 => 0,
            PCLK_TIMER1W::CCLK => 1,
            PCLK_TIMER1W::CCLK_DIV_2 => 2,
            PCLK_TIMER1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_TIMER1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_TIMER1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_UART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_UART0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART0R::CCLK_DIV_4 => 0,
            PCLK_UART0R::CCLK => 1,
            PCLK_UART0R::CCLK_DIV_2 => 2,
            PCLK_UART0R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_UART0_R = crate::FR<u8, PCLK_UART0R>;
impl PCLK_UART0_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_UART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART0W::CCLK_DIV_4 => 0,
            PCLK_UART0W::CCLK => 1,
            PCLK_UART0W::CCLK_DIV_2 => 2,
            PCLK_UART0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_UART0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_UART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_UART1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART1R::CCLK_DIV_4 => 0,
            PCLK_UART1R::CCLK => 1,
            PCLK_UART1R::CCLK_DIV_2 => 2,
            PCLK_UART1R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_UART1_R = crate::FR<u8, PCLK_UART1R>;
impl PCLK_UART1_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_UART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART1W::CCLK_DIV_4 => 0,
            PCLK_UART1W::CCLK => 1,
            PCLK_UART1W::CCLK_DIV_2 => 2,
            PCLK_UART1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_UART1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_UART1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_PWM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_PWM1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_PWM1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_PWM1R::CCLK_DIV_4 => 0,
            PCLK_PWM1R::CCLK => 1,
            PCLK_PWM1R::CCLK_DIV_2 => 2,
            PCLK_PWM1R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_PWM1_R = crate::FR<u8, PCLK_PWM1R>;
impl PCLK_PWM1_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PWM1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_PWM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_PWM1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_PWM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_PWM1W::CCLK_DIV_4 => 0,
            PCLK_PWM1W::CCLK => 1,
            PCLK_PWM1W::CCLK_DIV_2 => 2,
            PCLK_PWM1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_PWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_PWM1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_PWM1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2C0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_I2C0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2C0R::CCLK_DIV_4 => 0,
            PCLK_I2C0R::CCLK => 1,
            PCLK_I2C0R::CCLK_DIV_2 => 2,
            PCLK_I2C0R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_I2C0_R = crate::FR<u8, PCLK_I2C0R>;
impl PCLK_I2C0_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2C0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2C0W::CCLK_DIV_4 => 0,
            PCLK_I2C0W::CCLK => 1,
            PCLK_I2C0W::CCLK_DIV_2 => 2,
            PCLK_I2C0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_I2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_I2C0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_I2C0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_SPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SPIR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_SPIR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_SPIR::CCLK_DIV_4 => 0,
            PCLK_SPIR::CCLK => 1,
            PCLK_SPIR::CCLK_DIV_2 => 2,
            PCLK_SPIR::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_SPI_R = crate::FR<u8, PCLK_SPIR>;
impl PCLK_SPI_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SPIR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_SPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SPIW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SPIW::CCLK_DIV_4 => 0,
            PCLK_SPIW::CCLK => 1,
            PCLK_SPIW::CCLK_DIV_2 => 2,
            PCLK_SPIW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_SPIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SPIW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SPIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SSP1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_SSP1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_SSP1R::CCLK_DIV_4 => 0,
            PCLK_SSP1R::CCLK => 1,
            PCLK_SSP1R::CCLK_DIV_2 => 2,
            PCLK_SSP1R::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_SSP1_R = crate::FR<u8, PCLK_SSP1R>;
impl PCLK_SSP1_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SSP1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SSP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SSP1W::CCLK_DIV_4 => 0,
            PCLK_SSP1W::CCLK => 1,
            PCLK_SSP1W::CCLK_DIV_2 => 2,
            PCLK_SSP1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_SSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SSP1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_SSP1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_DAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_DACR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_DACR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_DACR::CCLK_DIV_4 => 0,
            PCLK_DACR::CCLK => 1,
            PCLK_DACR::CCLK_DIV_2 => 2,
            PCLK_DACR::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_DAC_R = crate::FR<u8, PCLK_DACR>;
impl PCLK_DAC_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_DACR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_DAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_DACW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_DACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_DACW::CCLK_DIV_4 => 0,
            PCLK_DACW::CCLK => 1,
            PCLK_DACW::CCLK_DIV_2 => 2,
            PCLK_DACW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_DACW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_DACW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_DACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ADCR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl crate::ToBits<u8> for PCLK_ADCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_ADCR::CCLK_DIV_4 => 0,
            PCLK_ADCR::CCLK => 1,
            PCLK_ADCR::CCLK_DIV_2 => 2,
            PCLK_ADCR::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_ADC_R = crate::FR<u8, PCLK_ADCR>;
impl PCLK_ADC_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ADCR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ADCW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_ADCW::CCLK_DIV_4 => 0,
            PCLK_ADCW::CCLK => 1,
            PCLK_ADCW::CCLK_DIV_2 => 2,
            PCLK_ADCW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_ADCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_ADCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_CAN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6,
}
impl crate::ToBits<u8> for PCLK_CAN1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN1R::CCLK_DIV_4 => 0,
            PCLK_CAN1R::CCLK => 1,
            PCLK_CAN1R::CCLK_DIV_2 => 2,
            PCLK_CAN1R::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_CAN1_R = crate::FR<u8, PCLK_CAN1R>;
impl PCLK_CAN1_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `PCLK_CAN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6,
}
impl PCLK_CAN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN1W::CCLK_DIV_4 => 0,
            PCLK_CAN1W::CCLK => 1,
            PCLK_CAN1W::CCLK_DIV_2 => 2,
            PCLK_CAN1W::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_CAN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_CAN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_CAN1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_CAN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN2R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6,
}
impl crate::ToBits<u8> for PCLK_CAN2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN2R::CCLK_DIV_4 => 0,
            PCLK_CAN2R::CCLK => 1,
            PCLK_CAN2R::CCLK_DIV_2 => 2,
            PCLK_CAN2R::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_CAN2_R = crate::FR<u8, PCLK_CAN2R>;
impl PCLK_CAN2_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN2R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `PCLK_CAN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN2W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6,
}
impl PCLK_CAN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN2W::CCLK_DIV_4 => 0,
            PCLK_CAN2W::CCLK => 1,
            PCLK_CAN2W::CCLK_DIV_2 => 2,
            PCLK_CAN2W::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_CAN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_CAN2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_CAN2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `PCLK_ACF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ACFR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6,
}
impl crate::ToBits<u8> for PCLK_ACFR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PCLK_ACFR::CCLK_DIV_4 => 0,
            PCLK_ACFR::CCLK => 1,
            PCLK_ACFR::CCLK_DIV_2 => 2,
            PCLK_ACFR::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PCLK_ACF_R = crate::FR<u8, PCLK_ACFR>;
impl PCLK_ACF_R {
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ACFR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `PCLK_ACF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ACFW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6,
}
impl PCLK_ACFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_ACFW::CCLK_DIV_4 => 0,
            PCLK_ACFW::CCLK => 1,
            PCLK_ACFW::CCLK_DIV_2 => 2,
            PCLK_ACFW::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PCLK_ACFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_ACFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK_ACFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_6)
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
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&self) -> PCLK_WDT_R {
        PCLK_WDT_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&self) -> PCLK_TIMER0_R {
        PCLK_TIMER0_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1_R {
        PCLK_TIMER1_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&self) -> PCLK_UART0_R {
        PCLK_UART0_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PCLK_UART1_R {
        PCLK_UART1_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&self) -> PCLK_PWM1_R {
        PCLK_PWM1_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&self) -> PCLK_I2C0_R {
        PCLK_I2C0_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&self) -> PCLK_SPI_R {
        PCLK_SPI_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&self) -> PCLK_SSP1_R {
        PCLK_SSP1_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PCLK_DAC_R {
        PCLK_DAC_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PCLK_ADC_R {
        PCLK_ADC_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PCLK_CAN1_R {
        PCLK_CAN1_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PCLK_CAN2_R {
        PCLK_CAN2_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&self) -> PCLK_ACF_R {
        PCLK_ACF_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&mut self) -> _PCLK_WDTW {
        _PCLK_WDTW { w: self }
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&mut self) -> _PCLK_TIMER0W {
        _PCLK_TIMER0W { w: self }
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&mut self) -> _PCLK_TIMER1W {
        _PCLK_TIMER1W { w: self }
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&mut self) -> _PCLK_UART0W {
        _PCLK_UART0W { w: self }
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&mut self) -> _PCLK_UART1W {
        _PCLK_UART1W { w: self }
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&mut self) -> _PCLK_PWM1W {
        _PCLK_PWM1W { w: self }
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&mut self) -> _PCLK_I2C0W {
        _PCLK_I2C0W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&mut self) -> _PCLK_SPIW {
        _PCLK_SPIW { w: self }
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&mut self) -> _PCLK_SSP1W {
        _PCLK_SSP1W { w: self }
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&mut self) -> _PCLK_DACW {
        _PCLK_DACW { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&mut self) -> _PCLK_ADCW {
        _PCLK_ADCW { w: self }
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&mut self) -> _PCLK_CAN1W {
        _PCLK_CAN1W { w: self }
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&mut self) -> _PCLK_CAN2W {
        _PCLK_CAN2W { w: self }
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&mut self) -> _PCLK_ACFW {
        _PCLK_ACFW { w: self }
    }
}
