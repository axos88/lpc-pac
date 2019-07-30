#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOFTSREQ {
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
pub type SOFTSREQ0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ0W<'a> {
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
pub type SOFTSREQ1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ1W<'a> {
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
pub type SOFTSREQ2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ2W<'a> {
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
pub type SOFTSREQ3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ3W<'a> {
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
pub type SOFTSREQ4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ4W<'a> {
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
pub type SOFTSREQ5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ5W<'a> {
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
pub type SOFTSREQ6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ6W<'a> {
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
#[doc = r"Reader of the field"]
pub type SOFTSREQ7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ7W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTSREQ8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ8W<'a> {
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
pub type SOFTSREQ9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ9W<'a> {
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
pub type SOFTSREQ10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ10W<'a> {
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
#[doc = r"Reader of the field"]
pub type SOFTSREQ11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ11W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTSREQ12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ12W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTSREQ13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ13W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTSREQ14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ14W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOFTSREQ15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTSREQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTSREQ15W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&self) -> SOFTSREQ0_R {
        SOFTSREQ0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&self) -> SOFTSREQ1_R {
        SOFTSREQ1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&self) -> SOFTSREQ2_R {
        SOFTSREQ2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&self) -> SOFTSREQ3_R {
        SOFTSREQ3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&self) -> SOFTSREQ4_R {
        SOFTSREQ4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&self) -> SOFTSREQ5_R {
        SOFTSREQ5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&self) -> SOFTSREQ6_R {
        SOFTSREQ6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&self) -> SOFTSREQ7_R {
        SOFTSREQ7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&self) -> SOFTSREQ8_R {
        SOFTSREQ8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&self) -> SOFTSREQ9_R {
        SOFTSREQ9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&self) -> SOFTSREQ10_R {
        SOFTSREQ10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&self) -> SOFTSREQ11_R {
        SOFTSREQ11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&self) -> SOFTSREQ12_R {
        SOFTSREQ12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&self) -> SOFTSREQ13_R {
        SOFTSREQ13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&self) -> SOFTSREQ14_R {
        SOFTSREQ14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&self) -> SOFTSREQ15_R {
        SOFTSREQ15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&mut self) -> _SOFTSREQ0W {
        _SOFTSREQ0W { w: self }
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&mut self) -> _SOFTSREQ1W {
        _SOFTSREQ1W { w: self }
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&mut self) -> _SOFTSREQ2W {
        _SOFTSREQ2W { w: self }
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&mut self) -> _SOFTSREQ3W {
        _SOFTSREQ3W { w: self }
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&mut self) -> _SOFTSREQ4W {
        _SOFTSREQ4W { w: self }
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&mut self) -> _SOFTSREQ5W {
        _SOFTSREQ5W { w: self }
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&mut self) -> _SOFTSREQ6W {
        _SOFTSREQ6W { w: self }
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&mut self) -> _SOFTSREQ7W {
        _SOFTSREQ7W { w: self }
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&mut self) -> _SOFTSREQ8W {
        _SOFTSREQ8W { w: self }
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&mut self) -> _SOFTSREQ9W {
        _SOFTSREQ9W { w: self }
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&mut self) -> _SOFTSREQ10W {
        _SOFTSREQ10W { w: self }
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&mut self) -> _SOFTSREQ11W {
        _SOFTSREQ11W { w: self }
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&mut self) -> _SOFTSREQ12W {
        _SOFTSREQ12W { w: self }
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&mut self) -> _SOFTSREQ13W {
        _SOFTSREQ13W { w: self }
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&mut self) -> _SOFTSREQ14W {
        _SOFTSREQ14W { w: self }
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&mut self) -> _SOFTSREQ15W {
        _SOFTSREQ15W { w: self }
    }
}
