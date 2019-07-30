#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPDMADIS {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS0W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS1W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS2W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS3W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS4W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS5W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS6W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS7W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS8W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS8W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS9W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS9W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS10W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS10W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS11W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS11W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS12W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS12W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS13W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS13W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS14W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS14W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS15W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS15W<'a> {
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
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS16W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS16W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS17W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS17W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS18W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS18W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS19W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS19W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS20W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS20W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS21W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS21W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS22W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS22W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS23W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS23W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS24W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS24W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS25W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS25W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS26W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS26W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS27W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS27W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS28W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS28W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS29W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS29W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS30W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS30W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _EP_DMA_DIS31W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DMA_DIS31W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_DISABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_dis0(&mut self) -> _EP_DMA_DIS0W {
        _EP_DMA_DIS0W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_DISABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_dis1(&mut self) -> _EP_DMA_DIS1W {
        _EP_DMA_DIS1W { w: self }
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis2(&mut self) -> _EP_DMA_DIS2W {
        _EP_DMA_DIS2W { w: self }
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis3(&mut self) -> _EP_DMA_DIS3W {
        _EP_DMA_DIS3W { w: self }
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis4(&mut self) -> _EP_DMA_DIS4W {
        _EP_DMA_DIS4W { w: self }
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis5(&mut self) -> _EP_DMA_DIS5W {
        _EP_DMA_DIS5W { w: self }
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis6(&mut self) -> _EP_DMA_DIS6W {
        _EP_DMA_DIS6W { w: self }
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis7(&mut self) -> _EP_DMA_DIS7W {
        _EP_DMA_DIS7W { w: self }
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis8(&mut self) -> _EP_DMA_DIS8W {
        _EP_DMA_DIS8W { w: self }
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis9(&mut self) -> _EP_DMA_DIS9W {
        _EP_DMA_DIS9W { w: self }
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis10(&mut self) -> _EP_DMA_DIS10W {
        _EP_DMA_DIS10W { w: self }
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis11(&mut self) -> _EP_DMA_DIS11W {
        _EP_DMA_DIS11W { w: self }
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis12(&mut self) -> _EP_DMA_DIS12W {
        _EP_DMA_DIS12W { w: self }
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis13(&mut self) -> _EP_DMA_DIS13W {
        _EP_DMA_DIS13W { w: self }
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis14(&mut self) -> _EP_DMA_DIS14W {
        _EP_DMA_DIS14W { w: self }
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis15(&mut self) -> _EP_DMA_DIS15W {
        _EP_DMA_DIS15W { w: self }
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis16(&mut self) -> _EP_DMA_DIS16W {
        _EP_DMA_DIS16W { w: self }
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis17(&mut self) -> _EP_DMA_DIS17W {
        _EP_DMA_DIS17W { w: self }
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis18(&mut self) -> _EP_DMA_DIS18W {
        _EP_DMA_DIS18W { w: self }
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis19(&mut self) -> _EP_DMA_DIS19W {
        _EP_DMA_DIS19W { w: self }
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis20(&mut self) -> _EP_DMA_DIS20W {
        _EP_DMA_DIS20W { w: self }
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis21(&mut self) -> _EP_DMA_DIS21W {
        _EP_DMA_DIS21W { w: self }
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis22(&mut self) -> _EP_DMA_DIS22W {
        _EP_DMA_DIS22W { w: self }
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis23(&mut self) -> _EP_DMA_DIS23W {
        _EP_DMA_DIS23W { w: self }
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis24(&mut self) -> _EP_DMA_DIS24W {
        _EP_DMA_DIS24W { w: self }
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis25(&mut self) -> _EP_DMA_DIS25W {
        _EP_DMA_DIS25W { w: self }
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis26(&mut self) -> _EP_DMA_DIS26W {
        _EP_DMA_DIS26W { w: self }
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis27(&mut self) -> _EP_DMA_DIS27W {
        _EP_DMA_DIS27W { w: self }
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis28(&mut self) -> _EP_DMA_DIS28W {
        _EP_DMA_DIS28W { w: self }
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis29(&mut self) -> _EP_DMA_DIS29W {
        _EP_DMA_DIS29W { w: self }
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis30(&mut self) -> _EP_DMA_DIS30W {
        _EP_DMA_DIS30W { w: self }
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis31(&mut self) -> _EP_DMA_DIS31W {
        _EP_DMA_DIS31W { w: self }
    }
}
