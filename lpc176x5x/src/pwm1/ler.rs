#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LER {
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
pub type MAT0LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT0LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT0LATCHENW<'a> {
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
pub type MAT1LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT1LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT1LATCHENW<'a> {
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
pub type MAT2LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT2LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT2LATCHENW<'a> {
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
pub type MAT3LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT3LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT3LATCHENW<'a> {
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
pub type MAT4LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT4LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT4LATCHENW<'a> {
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
pub type MAT5LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT5LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT5LATCHENW<'a> {
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
pub type MAT6LATCHEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MAT6LATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAT6LATCHENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&self) -> MAT0LATCHEN_R {
        MAT0LATCHEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&self) -> MAT1LATCHEN_R {
        MAT1LATCHEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&self) -> MAT2LATCHEN_R {
        MAT2LATCHEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&self) -> MAT3LATCHEN_R {
        MAT3LATCHEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&self) -> MAT4LATCHEN_R {
        MAT4LATCHEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&self) -> MAT5LATCHEN_R {
        MAT5LATCHEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&self) -> MAT6LATCHEN_R {
        MAT6LATCHEN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&mut self) -> _MAT0LATCHENW {
        _MAT0LATCHENW { w: self }
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&mut self) -> _MAT1LATCHENW {
        _MAT1LATCHENW { w: self }
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&mut self) -> _MAT2LATCHENW {
        _MAT2LATCHENW { w: self }
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&mut self) -> _MAT3LATCHENW {
        _MAT3LATCHENW { w: self }
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&mut self) -> _MAT4LATCHENW {
        _MAT4LATCHENW { w: self }
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&mut self) -> _MAT5LATCHENW {
        _MAT5LATCHENW { w: self }
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&mut self) -> _MAT6LATCHENW {
        _MAT6LATCHENW { w: self }
    }
}
