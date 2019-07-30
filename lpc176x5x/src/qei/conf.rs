#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
pub type DIRINV_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DIRINVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRINVW<'a> {
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
pub type SIGMODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SIGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGMODEW<'a> {
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
pub type CAPMODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPMODEW<'a> {
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
pub type INVINX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INVINXW<'a> {
    w: &'a mut W,
}
impl<'a> _INVINXW<'a> {
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
pub type CRESPI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CRESPIW<'a> {
    w: &'a mut W,
}
impl<'a> _CRESPIW<'a> {
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
pub type INXGATE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INXGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INXGATEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&self) -> DIRINV_R {
        DIRINV_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&self) -> SIGMODE_R {
        SIGMODE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&self) -> INVINX_R {
        INVINX_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&self) -> CRESPI_R {
        CRESPI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\] = 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\] = 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\] = 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\] = 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&self) -> INXGATE_R {
        INXGATE_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&mut self) -> _DIRINVW {
        _DIRINVW { w: self }
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&mut self) -> _SIGMODEW {
        _SIGMODEW { w: self }
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&mut self) -> _CAPMODEW {
        _CAPMODEW { w: self }
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&mut self) -> _INVINXW {
        _INVINXW { w: self }
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&mut self) -> _CRESPIW {
        _CRESPIW { w: self }
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\] = 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\] = 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\] = 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\] = 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&mut self) -> _INXGATEW {
        _INXGATEW { w: self }
    }
}
