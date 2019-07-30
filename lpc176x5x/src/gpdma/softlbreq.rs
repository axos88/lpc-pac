#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOFTLBREQ {
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
pub type SOFTLBREQ0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ0W<'a> {
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
pub type SOFTLBREQ1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ1W<'a> {
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
pub type SOFTLBREQ2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ2W<'a> {
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
pub type SOFTLBREQ3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ3W<'a> {
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
pub type SOFTLBREQ4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ4W<'a> {
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
pub type SOFTLBREQ5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ5W<'a> {
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
pub type SOFTLBREQ6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ6W<'a> {
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
pub type SOFTLBREQ7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ7W<'a> {
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
pub type SOFTLBREQ8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ8W<'a> {
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
pub type SOFTLBREQ9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ9W<'a> {
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
pub type SOFTLBREQ10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ10W<'a> {
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
pub type SOFTLBREQ11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ11W<'a> {
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
pub type SOFTLBREQ12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ12W<'a> {
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
pub type SOFTLBREQ13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ13W<'a> {
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
pub type SOFTLBREQ14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ14W<'a> {
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
pub type SOFTLBREQ15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLBREQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLBREQ15W<'a> {
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
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&self) -> SOFTLBREQ0_R {
        SOFTLBREQ0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&self) -> SOFTLBREQ1_R {
        SOFTLBREQ1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&self) -> SOFTLBREQ2_R {
        SOFTLBREQ2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&self) -> SOFTLBREQ3_R {
        SOFTLBREQ3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&self) -> SOFTLBREQ4_R {
        SOFTLBREQ4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&self) -> SOFTLBREQ5_R {
        SOFTLBREQ5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&self) -> SOFTLBREQ6_R {
        SOFTLBREQ6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&self) -> SOFTLBREQ7_R {
        SOFTLBREQ7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&self) -> SOFTLBREQ8_R {
        SOFTLBREQ8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&self) -> SOFTLBREQ9_R {
        SOFTLBREQ9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&self) -> SOFTLBREQ10_R {
        SOFTLBREQ10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&self) -> SOFTLBREQ11_R {
        SOFTLBREQ11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&self) -> SOFTLBREQ12_R {
        SOFTLBREQ12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&self) -> SOFTLBREQ13_R {
        SOFTLBREQ13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&self) -> SOFTLBREQ14_R {
        SOFTLBREQ14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&self) -> SOFTLBREQ15_R {
        SOFTLBREQ15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&mut self) -> _SOFTLBREQ0W {
        _SOFTLBREQ0W { w: self }
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&mut self) -> _SOFTLBREQ1W {
        _SOFTLBREQ1W { w: self }
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&mut self) -> _SOFTLBREQ2W {
        _SOFTLBREQ2W { w: self }
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&mut self) -> _SOFTLBREQ3W {
        _SOFTLBREQ3W { w: self }
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&mut self) -> _SOFTLBREQ4W {
        _SOFTLBREQ4W { w: self }
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&mut self) -> _SOFTLBREQ5W {
        _SOFTLBREQ5W { w: self }
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&mut self) -> _SOFTLBREQ6W {
        _SOFTLBREQ6W { w: self }
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&mut self) -> _SOFTLBREQ7W {
        _SOFTLBREQ7W { w: self }
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&mut self) -> _SOFTLBREQ8W {
        _SOFTLBREQ8W { w: self }
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&mut self) -> _SOFTLBREQ9W {
        _SOFTLBREQ9W { w: self }
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&mut self) -> _SOFTLBREQ10W {
        _SOFTLBREQ10W { w: self }
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&mut self) -> _SOFTLBREQ11W {
        _SOFTLBREQ11W { w: self }
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&mut self) -> _SOFTLBREQ12W {
        _SOFTLBREQ12W { w: self }
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&mut self) -> _SOFTLBREQ13W {
        _SOFTLBREQ13W { w: self }
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&mut self) -> _SOFTLBREQ14W {
        _SOFTLBREQ14W { w: self }
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&mut self) -> _SOFTLBREQ15W {
        _SOFTLBREQ15W { w: self }
    }
}
