#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BTR {
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
        0x001c_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type BRP_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _BRPW<'a> {
    w: &'a mut W,
}
impl<'a> _BRPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SJW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SJWW<'a> {
    w: &'a mut W,
}
impl<'a> _SJWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TESG1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TESG1W<'a> {
    w: &'a mut W,
}
impl<'a> _TESG1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TESG2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TESG2W<'a> {
    w: &'a mut W,
}
impl<'a> _TESG2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMR {
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O,
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3,
}
impl crate::ToBits<bool> for SAMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SAMR::THE_BUS_IS_SAMPLED_O => false,
            SAMR::THE_BUS_IS_SAMPLED_3 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SAM_R = crate::FR<bool, SAMR>;
impl SAM_R {
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_O`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_o(&self) -> bool {
        *self == SAMR::THE_BUS_IS_SAMPLED_O
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_3`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_3(&self) -> bool {
        *self == SAMR::THE_BUS_IS_SAMPLED_3
    }
}
#[doc = "Values that can be written to the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMW {
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O,
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3,
}
impl SAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMW::THE_BUS_IS_SAMPLED_O => false,
            SAMW::THE_BUS_IS_SAMPLED_3 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_o(self) -> &'a mut W {
        self.variant(SAMW::THE_BUS_IS_SAMPLED_O)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_3(self) -> &'a mut W {
        self.variant(SAMW::THE_BUS_IS_SAMPLED_3)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&self) -> TESG1_R {
        TESG1_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&self) -> TESG2_R {
        TESG2_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&mut self) -> _BRPW {
        _BRPW { w: self }
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&mut self) -> _SJWW {
        _SJWW { w: self }
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&mut self) -> _TESG1W {
        _TESG1W { w: self }
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&mut self) -> _TESG2W {
        _TESG2W { w: self }
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
}
