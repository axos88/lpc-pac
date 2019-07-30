#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXFILTERCTRL {
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
pub type AUE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUEW<'a> {
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
pub type ABE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEW<'a> {
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
pub type AME_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMEW<'a> {
    w: &'a mut W,
}
impl<'a> _AMEW<'a> {
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
pub type AUHE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUHEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUHEW<'a> {
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
pub type AMHE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMHEW<'a> {
    w: &'a mut W,
}
impl<'a> _AMHEW<'a> {
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
pub type APE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _APEW<'a> {
    w: &'a mut W,
}
impl<'a> _APEW<'a> {
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
pub type MPEW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MPEWW<'a> {
    w: &'a mut W,
}
impl<'a> _MPEWW<'a> {
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
pub type RFEW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RFEWW<'a> {
    w: &'a mut W,
}
impl<'a> _RFEWW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&self) -> AUE_R {
        AUE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&self) -> ABE_R {
        ABE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&self) -> AME_R {
        AME_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&self) -> AUHE_R {
        AUHE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&self) -> AMHE_R {
        AMHE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&self) -> APE_R {
        APE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&self) -> MPEW_R {
        MPEW_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&self) -> RFEW_R {
        RFEW_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&mut self) -> _AUEW {
        _AUEW { w: self }
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&mut self) -> _ABEW {
        _ABEW { w: self }
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&mut self) -> _AMEW {
        _AMEW { w: self }
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&mut self) -> _AUHEW {
        _AUHEW { w: self }
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&mut self) -> _AMHEW {
        _AMHEW { w: self }
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&mut self) -> _APEW {
        _APEW { w: self }
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&mut self) -> _MPEWW {
        _MPEWW { w: self }
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&mut self) -> _RFEWW {
        _RFEWW { w: self }
    }
}
