#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL7 {
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
#[doc = "Possible values of the field `P3_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25R {
    #[doc = "GPIO P3.25"]
    GPIO_P3,
    #[doc = "MAT0.0"]
    MAT0,
    #[doc = "PWM1.2"]
    PWM1,
}
impl crate::ToBits<u8> for P3_25R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P3_25R::GPIO_P3 => 0,
            P3_25R::MAT0 => 2,
            P3_25R::PWM1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P3_25_R = crate::FR<u8, P3_25R>;
impl P3_25_R {
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_25R::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_25R::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_25R::PWM1
    }
}
#[doc = "Values that can be written to the field `P3_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25W {
    #[doc = "GPIO P3.25"]
    GPIO_P3,
    #[doc = "MAT0.0"]
    MAT0,
    #[doc = "PWM1.2"]
    PWM1,
}
impl P3_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_25W::GPIO_P3 => 0,
            P3_25W::MAT0 => 2,
            P3_25W::PWM1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P3_25W<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_25W::GPIO_P3)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_25W::MAT0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_25W::PWM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P3_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26R {
    #[doc = "GPIO P3.26"]
    GPIO_P3,
    #[doc = "STCLK"]
    STCLK,
    #[doc = "MAT0.1"]
    MAT0,
    #[doc = "PWM1.3"]
    PWM1,
}
impl crate::ToBits<u8> for P3_26R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P3_26R::GPIO_P3 => 0,
            P3_26R::STCLK => 1,
            P3_26R::MAT0 => 2,
            P3_26R::PWM1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P3_26_R = crate::FR<u8, P3_26R>;
impl P3_26_R {
    #[doc = "Checks if the value of the field is `GPIO_P3`"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_26R::GPIO_P3
    }
    #[doc = "Checks if the value of the field is `STCLK`"]
    #[inline(always)]
    pub fn is_stclk(&self) -> bool {
        *self == P3_26R::STCLK
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_26R::MAT0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_26R::PWM1
    }
}
#[doc = "Values that can be written to the field `P3_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26W {
    #[doc = "GPIO P3.26"]
    GPIO_P3,
    #[doc = "STCLK"]
    STCLK,
    #[doc = "MAT0.1"]
    MAT0,
    #[doc = "PWM1.3"]
    PWM1,
}
impl P3_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_26W::GPIO_P3 => 0,
            P3_26W::STCLK => 1,
            P3_26W::MAT0 => 2,
            P3_26W::PWM1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P3_26W<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut W {
        self.variant(P3_26W::GPIO_P3)
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn stclk(self) -> &'a mut W {
        self.variant(P3_26W::STCLK)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P3_26W::MAT0)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P3_26W::PWM1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&self) -> P3_25_R {
        P3_25_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&self) -> P3_26_R {
        P3_26_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&mut self) -> _P3_25W {
        _P3_25W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&mut self) -> _P3_26W {
        _P3_26W { w: self }
    }
}
