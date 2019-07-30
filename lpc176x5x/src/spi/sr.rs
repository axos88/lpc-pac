#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABRT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MODF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ROVR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WCOL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SPIF_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn abrt(&self) -> ABRT_R {
        ABRT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register."]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register."]
    #[inline(always)]
    pub fn wcol(&self) -> WCOL_R {
        WCOL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register."]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
