#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
pub type TMR_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TMR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_ENW<'a> {
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
pub type REMOVE_PU_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _REMOVE_PU_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _REMOVE_PU_ENW<'a> {
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
pub type HNP_FAILURE_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HNP_FAILURE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_FAILURE_ENW<'a> {
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
pub type HNP_SUCCES_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HNP_SUCCES_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_SUCCES_ENW<'a> {
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
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&self) -> REMOVE_PU_EN_R {
        REMOVE_PU_EN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&self) -> HNP_FAILURE_EN_R {
        HNP_FAILURE_EN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&self) -> HNP_SUCCES_EN_R {
        HNP_SUCCES_EN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> _TMR_ENW {
        _TMR_ENW { w: self }
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&mut self) -> _REMOVE_PU_ENW {
        _REMOVE_PU_ENW { w: self }
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&mut self) -> _HNP_FAILURE_ENW {
        _HNP_FAILURE_ENW { w: self }
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&mut self) -> _HNP_SUCCES_ENW {
        _HNP_SUCCES_ENW { w: self }
    }
}
