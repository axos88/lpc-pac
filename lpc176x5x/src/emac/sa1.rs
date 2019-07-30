#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SA1 {
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
pub type SADDR4_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADDR4W<'a> {
    w: &'a mut W,
}
impl<'a> _SADDR4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADDR3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADDR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SADDR3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&self) -> SADDR4_R {
        SADDR4_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&self) -> SADDR3_R {
        SADDR3_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&mut self) -> _SADDR4W {
        _SADDR4W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&mut self) -> _SADDR3W {
        _SADDR3W { w: self }
    }
}
