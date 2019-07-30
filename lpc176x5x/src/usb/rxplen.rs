#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXPLEN {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PKT_LNGTH_R = crate::FR<u16, u16>;
#[doc = "Possible values of the field `DV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVR {
    #[doc = "Data is invalid."]
    DATA_IS_INVALID_,
    #[doc = "Data is valid."]
    DATA_IS_VALID_,
}
impl crate::ToBits<bool> for DVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DVR::DATA_IS_INVALID_ => false,
            DVR::DATA_IS_VALID_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DV_R = crate::FR<bool, DVR>;
impl DV_R {
    #[doc = "Checks if the value of the field is `DATA_IS_INVALID_`"]
    #[inline(always)]
    pub fn is_data_is_invalid_(&self) -> bool {
        *self == DVR::DATA_IS_INVALID_
    }
    #[doc = "Checks if the value of the field is `DATA_IS_VALID_`"]
    #[inline(always)]
    pub fn is_data_is_valid_(&self) -> bool {
        *self == DVR::DATA_IS_VALID_
    }
}
#[doc = r"Reader of the field"]
pub type PKT_RDY_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&self) -> PKT_LNGTH_R {
        PKT_LNGTH_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The PKT_LNGTH field is valid and the packet is ready for reading."]
    #[inline(always)]
    pub fn pkt_rdy(&self) -> PKT_RDY_R {
        PKT_RDY_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
