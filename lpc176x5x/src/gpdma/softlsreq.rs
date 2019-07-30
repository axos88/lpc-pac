#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOFTLSREQ {
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
pub type SOFTLSREQ0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ0W<'a> {
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
pub type SOFTLSREQ1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ1W<'a> {
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
pub type SOFTLSREQ2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ2W<'a> {
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
pub type SOFTLSREQ3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ3W<'a> {
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
pub type SOFTLSREQ4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ4W<'a> {
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
pub type SOFTLSREQ5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ5W<'a> {
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
pub type SOFTLSREQ6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ6W<'a> {
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
pub type SOFTLSREQ7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ7W<'a> {
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
pub type SOFTLSREQ8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ8W<'a> {
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
pub type SOFTLSREQ9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ9W<'a> {
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
pub type SOFTLSREQ10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ10W<'a> {
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
pub type SOFTLSREQ11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ11W<'a> {
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
pub type SOFTLSREQ12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ12W<'a> {
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
pub type SOFTLSREQ13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ13W<'a> {
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
pub type SOFTLSREQ14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ14W<'a> {
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
pub type SOFTLSREQ15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOFTLSREQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTLSREQ15W<'a> {
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
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&self) -> SOFTLSREQ0_R {
        SOFTLSREQ0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&self) -> SOFTLSREQ1_R {
        SOFTLSREQ1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&self) -> SOFTLSREQ2_R {
        SOFTLSREQ2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&self) -> SOFTLSREQ3_R {
        SOFTLSREQ3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&self) -> SOFTLSREQ4_R {
        SOFTLSREQ4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&self) -> SOFTLSREQ5_R {
        SOFTLSREQ5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&self) -> SOFTLSREQ6_R {
        SOFTLSREQ6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&self) -> SOFTLSREQ7_R {
        SOFTLSREQ7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&self) -> SOFTLSREQ8_R {
        SOFTLSREQ8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&self) -> SOFTLSREQ9_R {
        SOFTLSREQ9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&self) -> SOFTLSREQ10_R {
        SOFTLSREQ10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&self) -> SOFTLSREQ11_R {
        SOFTLSREQ11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&self) -> SOFTLSREQ12_R {
        SOFTLSREQ12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&self) -> SOFTLSREQ13_R {
        SOFTLSREQ13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&self) -> SOFTLSREQ14_R {
        SOFTLSREQ14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&self) -> SOFTLSREQ15_R {
        SOFTLSREQ15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&mut self) -> _SOFTLSREQ0W {
        _SOFTLSREQ0W { w: self }
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&mut self) -> _SOFTLSREQ1W {
        _SOFTLSREQ1W { w: self }
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&mut self) -> _SOFTLSREQ2W {
        _SOFTLSREQ2W { w: self }
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&mut self) -> _SOFTLSREQ3W {
        _SOFTLSREQ3W { w: self }
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&mut self) -> _SOFTLSREQ4W {
        _SOFTLSREQ4W { w: self }
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&mut self) -> _SOFTLSREQ5W {
        _SOFTLSREQ5W { w: self }
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&mut self) -> _SOFTLSREQ6W {
        _SOFTLSREQ6W { w: self }
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&mut self) -> _SOFTLSREQ7W {
        _SOFTLSREQ7W { w: self }
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&mut self) -> _SOFTLSREQ8W {
        _SOFTLSREQ8W { w: self }
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&mut self) -> _SOFTLSREQ9W {
        _SOFTLSREQ9W { w: self }
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&mut self) -> _SOFTLSREQ10W {
        _SOFTLSREQ10W { w: self }
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&mut self) -> _SOFTLSREQ11W {
        _SOFTLSREQ11W { w: self }
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&mut self) -> _SOFTLSREQ12W {
        _SOFTLSREQ12W { w: self }
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&mut self) -> _SOFTLSREQ13W {
        _SOFTLSREQ13W { w: self }
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&mut self) -> _SOFTLSREQ14W {
        _SOFTLSREQ14W { w: self }
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&mut self) -> _SOFTLSREQ15W {
        _SOFTLSREQ15W { w: self }
    }
}
