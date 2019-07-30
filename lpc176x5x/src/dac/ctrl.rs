#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `INT_DMA_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_DMA_REQR {
    #[doc = "Clear on any write to the DACR register."]
    CLEAR_ON_ANY_WRITE_T,
    #[doc = "Set by hardware when the timer times out."]
    SET_BY_HARDWARE_WHEN,
}
impl crate::ToBits<bool> for INT_DMA_REQR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INT_DMA_REQR::CLEAR_ON_ANY_WRITE_T => false,
            INT_DMA_REQR::SET_BY_HARDWARE_WHEN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INT_DMA_REQ_R = crate::FR<bool, INT_DMA_REQR>;
impl INT_DMA_REQ_R {
    #[doc = "Checks if the value of the field is `CLEAR_ON_ANY_WRITE_T`"]
    #[inline(always)]
    pub fn is_clear_on_any_write_t(&self) -> bool {
        *self == INT_DMA_REQR::CLEAR_ON_ANY_WRITE_T
    }
    #[doc = "Checks if the value of the field is `SET_BY_HARDWARE_WHEN`"]
    #[inline(always)]
    pub fn is_set_by_hardware_when(&self) -> bool {
        *self == INT_DMA_REQR::SET_BY_HARDWARE_WHEN
    }
}
#[doc = "Values that can be written to the field `INT_DMA_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_DMA_REQW {
    #[doc = "Clear on any write to the DACR register."]
    CLEAR_ON_ANY_WRITE_T,
    #[doc = "Set by hardware when the timer times out."]
    SET_BY_HARDWARE_WHEN,
}
impl INT_DMA_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INT_DMA_REQW::CLEAR_ON_ANY_WRITE_T => false,
            INT_DMA_REQW::SET_BY_HARDWARE_WHEN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INT_DMA_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_DMA_REQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_DMA_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn clear_on_any_write_t(self) -> &'a mut W {
        self.variant(INT_DMA_REQW::CLEAR_ON_ANY_WRITE_T)
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn set_by_hardware_when(self) -> &'a mut W {
        self.variant(INT_DMA_REQW::SET_BY_HARDWARE_WHEN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `DBLBUF_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLBUF_ENAR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLE_WHEN_THIS_BI,
}
impl crate::ToBits<bool> for DBLBUF_ENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DBLBUF_ENAR::DISABLE => false,
            DBLBUF_ENAR::ENABLE_WHEN_THIS_BI => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DBLBUF_ENA_R = crate::FR<bool, DBLBUF_ENAR>;
impl DBLBUF_ENA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBLBUF_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_WHEN_THIS_BI`"]
    #[inline(always)]
    pub fn is_enable_when_this_bi(&self) -> bool {
        *self == DBLBUF_ENAR::ENABLE_WHEN_THIS_BI
    }
}
#[doc = "Values that can be written to the field `DBLBUF_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLBUF_ENAW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLE_WHEN_THIS_BI,
}
impl DBLBUF_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DBLBUF_ENAW::DISABLE => false,
            DBLBUF_ENAW::ENABLE_WHEN_THIS_BI => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DBLBUF_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLBUF_ENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLBUF_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBLBUF_ENAW::DISABLE)
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn enable_when_this_bi(self) -> &'a mut W {
        self.variant(DBLBUF_ENAW::ENABLE_WHEN_THIS_BI)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `CNT_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_ENAR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CNT_ENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CNT_ENAR::DISABLE => false,
            CNT_ENAR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CNT_ENA_R = crate::FR<bool, CNT_ENAR>;
impl CNT_ENA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CNT_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CNT_ENAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `CNT_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_ENAW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CNT_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CNT_ENAW::DISABLE => false,
            CNT_ENAW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CNT_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CNT_ENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CNT_ENAW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CNT_ENAW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `DMA_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENAR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    ENABLE_DMA_BURST_RE,
}
impl crate::ToBits<bool> for DMA_ENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DMA_ENAR::DISABLE => false,
            DMA_ENAR::ENABLE_DMA_BURST_RE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DMA_ENA_R = crate::FR<bool, DMA_ENAR>;
impl DMA_ENA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_DMA_BURST_RE`"]
    #[inline(always)]
    pub fn is_enable_dma_burst_re(&self) -> bool {
        *self == DMA_ENAR::ENABLE_DMA_BURST_RE
    }
}
#[doc = "Values that can be written to the field `DMA_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENAW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    ENABLE_DMA_BURST_RE,
}
impl DMA_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_ENAW::DISABLE => false,
            DMA_ENAW::ENABLE_DMA_BURST_RE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DMA_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENAW::DISABLE)
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn enable_dma_burst_re(self) -> &'a mut W {
        self.variant(DMA_ENAW::ENABLE_DMA_BURST_RE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&self) -> INT_DMA_REQ_R {
        INT_DMA_REQ_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&self) -> DBLBUF_ENA_R {
        DBLBUF_ENA_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&self) -> CNT_ENA_R {
        CNT_ENA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&self) -> DMA_ENA_R {
        DMA_ENA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&mut self) -> _INT_DMA_REQW {
        _INT_DMA_REQW { w: self }
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&mut self) -> _DBLBUF_ENAW {
        _DBLBUF_ENAW { w: self }
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&mut self) -> _CNT_ENAW {
        _CNT_ENAW { w: self }
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&mut self) -> _DMA_ENAW {
        _DMA_ENAW { w: self }
    }
}
