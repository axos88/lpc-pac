#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DT {
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
        0x3fff_ffff
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DT0_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _DT0W<'a> {
    w: &'a mut W,
}
impl<'a> _DT0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DT1_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _DT1W<'a> {
    w: &'a mut W,
}
impl<'a> _DT1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DT2_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _DT2W<'a> {
    w: &'a mut W,
}
impl<'a> _DT2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits() >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits() >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&mut self) -> _DT0W {
        _DT0W { w: self }
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&mut self) -> _DT1W {
        _DT1W { w: self }
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&mut self) -> _DT2W {
        _DT2W { w: self }
    }
}
