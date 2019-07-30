#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATE {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type IRQ_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMAREQ1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMAREQ2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RX_LEVEL_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TX_LEVEL_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
    #[inline(always)]
    pub fn dmareq1(&self) -> DMAREQ1_R {
        DMAREQ1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
    #[inline(always)]
    pub fn dmareq2(&self) -> DMAREQ2_R {
        DMAREQ2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Reflects the current level of the Receive FIFO."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
