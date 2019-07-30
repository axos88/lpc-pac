#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DMAINTST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTR {
    #[doc = "All bits in the USBEoTIntSt register are 0."]
    ALL_BITS_IN_THE_USBE,
    #[doc = "At least one bit in the USBEoTIntSt is set."]
    AT_LEAST_ONE_BIT_IN_,
}
impl crate::ToBits<bool> for EOTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EOTR::ALL_BITS_IN_THE_USBE => false,
            EOTR::AT_LEAST_ONE_BIT_IN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EOT_R = crate::FR<bool, EOTR>;
impl EOT_R {
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBE`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbe(&self) -> bool {
        *self == EOTR::ALL_BITS_IN_THE_USBE
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == EOTR::AT_LEAST_ONE_BIT_IN_
    }
}
#[doc = "Possible values of the field `NDDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDRR {
    #[doc = "All bits in the USBNDDRIntSt register are 0."]
    ALL_BITS_IN_THE_USBN,
    #[doc = "At least one bit in the USBNDDRIntSt is set."]
    AT_LEAST_ONE_BIT_IN_,
}
impl crate::ToBits<bool> for NDDRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NDDRR::ALL_BITS_IN_THE_USBN => false,
            NDDRR::AT_LEAST_ONE_BIT_IN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NDDR_R = crate::FR<bool, NDDRR>;
impl NDDR_R {
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBN`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbn(&self) -> bool {
        *self == NDDRR::ALL_BITS_IN_THE_USBN
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == NDDRR::AT_LEAST_ONE_BIT_IN_
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "All bits in the USBSysErrIntSt register are 0."]
    ALL_BITS_IN_THE_USBS,
    #[doc = "At least one bit in the USBSysErrIntSt is set."]
    AT_LEAST_ONE_BIT_IN_,
}
impl crate::ToBits<bool> for ERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ERRR::ALL_BITS_IN_THE_USBS => false,
            ERRR::AT_LEAST_ONE_BIT_IN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ERR_R = crate::FR<bool, ERRR>;
impl ERR_R {
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBS`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbs(&self) -> bool {
        *self == ERRR::ALL_BITS_IN_THE_USBS
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == ERRR::AT_LEAST_ONE_BIT_IN_
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
