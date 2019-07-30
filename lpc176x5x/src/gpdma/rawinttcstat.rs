#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RAWINTTCSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWINTTCSTAT7_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat0(&self) -> RAWINTTCSTAT0_R {
        RAWINTTCSTAT0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat1(&self) -> RAWINTTCSTAT1_R {
        RAWINTTCSTAT1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat2(&self) -> RAWINTTCSTAT2_R {
        RAWINTTCSTAT2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat3(&self) -> RAWINTTCSTAT3_R {
        RAWINTTCSTAT3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat4(&self) -> RAWINTTCSTAT4_R {
        RAWINTTCSTAT4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat5(&self) -> RAWINTTCSTAT5_R {
        RAWINTTCSTAT5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat6(&self) -> RAWINTTCSTAT6_R {
        RAWINTTCSTAT6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat7(&self) -> RAWINTTCSTAT7_R {
        RAWINTTCSTAT7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
