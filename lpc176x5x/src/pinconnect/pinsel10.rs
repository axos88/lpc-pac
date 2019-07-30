#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL10 {
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
#[doc = "Possible values of the field `TPIUCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIUCTRLR {
    #[doc = "Disabled. TPIU interface is disabled."]
    DISABLED,
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    ENABLED,
}
impl crate::ToBits<bool> for TPIUCTRLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TPIUCTRLR::DISABLED => false,
            TPIUCTRLR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TPIUCTRL_R = crate::FR<bool, TPIUCTRLR>;
impl TPIUCTRL_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIUCTRLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIUCTRLR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TPIUCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIUCTRLW {
    #[doc = "Disabled. TPIU interface is disabled."]
    DISABLED,
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    ENABLED,
}
impl TPIUCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TPIUCTRLW::DISABLED => false,
            TPIUCTRLW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TPIUCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TPIUCTRLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPIUCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TPIUCTRLW::DISABLED)
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TPIUCTRLW::ENABLED)
    }
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
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&self) -> TPIUCTRL_R {
        TPIUCTRL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&mut self) -> _TPIUCTRLW {
        _TPIUCTRLW { w: self }
    }
}
