#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IR {
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
#[doc = r"Reader of the field"]
pub type PWMMR0INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0INTW<'a> {
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
#[doc = r"Reader of the field"]
pub type PWMMR1INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1INTW<'a> {
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
#[doc = r"Reader of the field"]
pub type PWMMR2INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR2INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2INTW<'a> {
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
pub type PWMMR3INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR3INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3INTW<'a> {
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
pub type PWMCAP0INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMCAP0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMCAP0INTW<'a> {
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
pub type PWMCAP1INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMCAP1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMCAP1INTW<'a> {
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
pub type PWMMR4INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR4INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR5INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR5INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PWMMR6INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWMMR6INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&self) -> PWMMR0INT_R {
        PWMMR0INT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&self) -> PWMMR1INT_R {
        PWMMR1INT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&self) -> PWMMR2INT_R {
        PWMMR2INT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&self) -> PWMMR3INT_R {
        PWMMR3INT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&self) -> PWMCAP0INT_R {
        PWMCAP0INT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&self) -> PWMCAP1INT_R {
        PWMCAP1INT_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&self) -> PWMMR4INT_R {
        PWMMR4INT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&self) -> PWMMR5INT_R {
        PWMMR5INT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&self) -> PWMMR6INT_R {
        PWMMR6INT_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&mut self) -> _PWMMR0INTW {
        _PWMMR0INTW { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&mut self) -> _PWMMR1INTW {
        _PWMMR1INTW { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&mut self) -> _PWMMR2INTW {
        _PWMMR2INTW { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&mut self) -> _PWMMR3INTW {
        _PWMMR3INTW { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&mut self) -> _PWMCAP0INTW {
        _PWMCAP0INTW { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&mut self) -> _PWMCAP1INTW {
        _PWMCAP1INTW { w: self }
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&mut self) -> _PWMMR4INTW {
        _PWMMR4INTW { w: self }
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&mut self) -> _PWMMR5INTW {
        _PWMMR5INTW { w: self }
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&mut self) -> _PWMMR6INTW {
        _PWMMR6INTW { w: self }
    }
}
