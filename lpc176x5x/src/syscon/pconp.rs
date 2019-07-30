#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCONP {
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
        0x03be
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type PCTIM0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCTIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCTIM0W<'a> {
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
pub type PCTIM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCTIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCTIM1W<'a> {
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
pub type PCUART0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCUART0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCUART0W<'a> {
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
pub type PCUART1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCUART1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCUART1W<'a> {
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
pub type PCPWM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCPWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCPWM1W<'a> {
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
pub type PCI2C0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCI2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCI2C0W<'a> {
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
pub type PCSPI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCSPIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSPIW<'a> {
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
pub type PCRTC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCRTCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCRTCW<'a> {
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
pub type PCSSP1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCSSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSSP1W<'a> {
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
pub type PCADC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCADCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCADCW<'a> {
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
pub type PCCAN1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCCAN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCCAN1W<'a> {
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
pub type PCCAN2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCCAN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCCAN2W<'a> {
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
pub type PCGPIO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCGPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _PCGPIOW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCRIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCRITW<'a> {
    w: &'a mut W,
}
impl<'a> _PCRITW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCMCPWM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCMCPWMW<'a> {
    w: &'a mut W,
}
impl<'a> _PCMCPWMW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCQEI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCQEIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCQEIW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCI2C1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCI2C1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCI2C1W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCSSP0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCSSP0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCSSP0W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCTIM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCTIM2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCTIM2W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCTIM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCTIM3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCTIM3W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCUART2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCUART2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCUART2W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCUART3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCUART3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCUART3W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCI2C2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCI2C2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCI2C2W<'a> {
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
#[doc = r"Reader of the field"]
pub type PCI2S_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCI2SW<'a> {
    w: &'a mut W,
}
impl<'a> _PCI2SW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCGPDMA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCGPDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _PCGPDMAW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCENET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCENETW<'a> {
    w: &'a mut W,
}
impl<'a> _PCENETW<'a> {
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
#[doc = r"Reader of the field"]
pub type PCUSB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCUSBW<'a> {
    w: &'a mut W,
}
impl<'a> _PCUSBW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&self) -> PCTIM0_R {
        PCTIM0_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&self) -> PCTIM1_R {
        PCTIM1_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&self) -> PCUART0_R {
        PCUART0_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&self) -> PCUART1_R {
        PCUART1_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&self) -> PCPWM1_R {
        PCPWM1_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&self) -> PCI2C0_R {
        PCI2C0_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&self) -> PCSPI_R {
        PCSPI_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&self) -> PCRTC_R {
        PCRTC_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&self) -> PCSSP1_R {
        PCSSP1_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&self) -> PCADC_R {
        PCADC_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&self) -> PCCAN1_R {
        PCCAN1_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&self) -> PCCAN2_R {
        PCCAN2_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&self) -> PCGPIO_R {
        PCGPIO_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&self) -> PCRIT_R {
        PCRIT_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&self) -> PCMCPWM_R {
        PCMCPWM_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&self) -> PCQEI_R {
        PCQEI_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&self) -> PCI2C1_R {
        PCI2C1_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&self) -> PCSSP0_R {
        PCSSP0_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&self) -> PCTIM2_R {
        PCTIM2_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&self) -> PCTIM3_R {
        PCTIM3_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&self) -> PCUART2_R {
        PCUART2_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&self) -> PCUART3_R {
        PCUART3_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&self) -> PCI2C2_R {
        PCI2C2_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&self) -> PCI2S_R {
        PCI2S_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&self) -> PCGPDMA_R {
        PCGPDMA_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&self) -> PCENET_R {
        PCENET_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&self) -> PCUSB_R {
        PCUSB_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&mut self) -> _PCTIM0W {
        _PCTIM0W { w: self }
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&mut self) -> _PCTIM1W {
        _PCTIM1W { w: self }
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&mut self) -> _PCUART0W {
        _PCUART0W { w: self }
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&mut self) -> _PCUART1W {
        _PCUART1W { w: self }
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&mut self) -> _PCPWM1W {
        _PCPWM1W { w: self }
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&mut self) -> _PCI2C0W {
        _PCI2C0W { w: self }
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&mut self) -> _PCSPIW {
        _PCSPIW { w: self }
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&mut self) -> _PCRTCW {
        _PCRTCW { w: self }
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&mut self) -> _PCSSP1W {
        _PCSSP1W { w: self }
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&mut self) -> _PCADCW {
        _PCADCW { w: self }
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&mut self) -> _PCCAN1W {
        _PCCAN1W { w: self }
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&mut self) -> _PCCAN2W {
        _PCCAN2W { w: self }
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&mut self) -> _PCGPIOW {
        _PCGPIOW { w: self }
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&mut self) -> _PCRITW {
        _PCRITW { w: self }
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&mut self) -> _PCMCPWMW {
        _PCMCPWMW { w: self }
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&mut self) -> _PCQEIW {
        _PCQEIW { w: self }
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&mut self) -> _PCI2C1W {
        _PCI2C1W { w: self }
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&mut self) -> _PCSSP0W {
        _PCSSP0W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&mut self) -> _PCTIM2W {
        _PCTIM2W { w: self }
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&mut self) -> _PCTIM3W {
        _PCTIM3W { w: self }
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&mut self) -> _PCUART2W {
        _PCUART2W { w: self }
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&mut self) -> _PCUART3W {
        _PCUART3W { w: self }
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&mut self) -> _PCI2C2W {
        _PCI2C2W { w: self }
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&mut self) -> _PCI2SW {
        _PCI2SW { w: self }
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&mut self) -> _PCGPDMAW {
        _PCGPDMAW { w: self }
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&mut self) -> _PCENETW {
        _PCENETW { w: self }
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&mut self) -> _PCUSBW {
        _PCUSBW { w: self }
    }
}
