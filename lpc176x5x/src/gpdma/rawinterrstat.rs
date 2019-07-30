#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RAWINTERRSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTERRSTAT7_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat0(&self) -> RAWINTERRSTAT0_R {
        RAWINTERRSTAT0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat1(&self) -> RAWINTERRSTAT1_R {
        RAWINTERRSTAT1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat2(&self) -> RAWINTERRSTAT2_R {
        RAWINTERRSTAT2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat3(&self) -> RAWINTERRSTAT3_R {
        RAWINTERRSTAT3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat4(&self) -> RAWINTERRSTAT4_R {
        RAWINTERRSTAT4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat5(&self) -> RAWINTERRSTAT5_R {
        RAWINTERRSTAT5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat6(&self) -> RAWINTERRSTAT6_R {
        RAWINTERRSTAT6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat7(&self) -> RAWINTERRSTAT7_R {
        RAWINTERRSTAT7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
