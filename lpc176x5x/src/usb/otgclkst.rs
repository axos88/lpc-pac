#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OTGCLKST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HOST_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ONR {
    #[doc = "Host clock is not available."]
    HOST_CLOCK_IS_NOT_AV,
    #[doc = "Host clock is available."]
    HOST_CLOCK_IS_AVAILA,
}
impl crate::ToBits<bool> for HOST_CLK_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HOST_CLK_ONR::HOST_CLOCK_IS_NOT_AV => false,
            HOST_CLK_ONR::HOST_CLOCK_IS_AVAILA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HOST_CLK_ON_R = crate::FR<bool, HOST_CLK_ONR>;
impl HOST_CLK_ON_R {
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_NOT_AV`"]
    #[inline(always)]
    pub fn is_host_clock_is_not_av(&self) -> bool {
        *self == HOST_CLK_ONR::HOST_CLOCK_IS_NOT_AV
    }
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_AVAILA`"]
    #[inline(always)]
    pub fn is_host_clock_is_availa(&self) -> bool {
        *self == HOST_CLK_ONR::HOST_CLOCK_IS_AVAILA
    }
}
#[doc = "Possible values of the field `DEV_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ONR {
    #[doc = "Device clock is not available."]
    DEVICE_CLOCK_IS_NOT_,
    #[doc = "Device clock is available."]
    DEVICE_CLOCK_IS_AVAI,
}
impl crate::ToBits<bool> for DEV_CLK_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DEV_CLK_ONR::DEVICE_CLOCK_IS_NOT_ => false,
            DEV_CLK_ONR::DEVICE_CLOCK_IS_AVAI => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DEV_CLK_ON_R = crate::FR<bool, DEV_CLK_ONR>;
impl DEV_CLK_ON_R {
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_NOT_`"]
    #[inline(always)]
    pub fn is_device_clock_is_not_(&self) -> bool {
        *self == DEV_CLK_ONR::DEVICE_CLOCK_IS_NOT_
    }
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_AVAI`"]
    #[inline(always)]
    pub fn is_device_clock_is_avai(&self) -> bool {
        *self == DEV_CLK_ONR::DEVICE_CLOCK_IS_AVAI
    }
}
#[doc = "Possible values of the field `I2C_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ONR {
    #[doc = "I2C clock is not available."]
    I2C_CLOCK_IS_NOT_AVA,
    #[doc = "I2C clock is available."]
    I2C_CLOCK_IS_AVAILAB,
}
impl crate::ToBits<bool> for I2C_CLK_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C_CLK_ONR::I2C_CLOCK_IS_NOT_AVA => false,
            I2C_CLK_ONR::I2C_CLOCK_IS_AVAILAB => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C_CLK_ON_R = crate::FR<bool, I2C_CLK_ONR>;
impl I2C_CLK_ON_R {
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_i2c_clock_is_not_ava(&self) -> bool {
        *self == I2C_CLK_ONR::I2C_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_i2c_clock_is_availab(&self) -> bool {
        *self == I2C_CLK_ONR::I2C_CLOCK_IS_AVAILAB
    }
}
#[doc = "Possible values of the field `OTG_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ONR {
    #[doc = "OTG clock is not available."]
    OTG_CLOCK_IS_NOT_AVA,
    #[doc = "OTG clock is available."]
    OTG_CLOCK_IS_AVAILAB,
}
impl crate::ToBits<bool> for OTG_CLK_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OTG_CLK_ONR::OTG_CLOCK_IS_NOT_AVA => false,
            OTG_CLK_ONR::OTG_CLOCK_IS_AVAILAB => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OTG_CLK_ON_R = crate::FR<bool, OTG_CLK_ONR>;
impl OTG_CLK_ON_R {
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_otg_clock_is_not_ava(&self) -> bool {
        *self == OTG_CLK_ONR::OTG_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_otg_clock_is_availab(&self) -> bool {
        *self == OTG_CLK_ONR::OTG_CLOCK_IS_AVAILAB
    }
}
#[doc = "Possible values of the field `AHB_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ONR {
    #[doc = "AHB clock is not available."]
    AHB_CLOCK_IS_NOT_AVA,
    #[doc = "AHB clock is available."]
    AHB_CLOCK_IS_AVAILAB,
}
impl crate::ToBits<bool> for AHB_CLK_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AHB_CLK_ONR::AHB_CLOCK_IS_NOT_AVA => false,
            AHB_CLK_ONR::AHB_CLOCK_IS_AVAILAB => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AHB_CLK_ON_R = crate::FR<bool, AHB_CLK_ONR>;
impl AHB_CLK_ON_R {
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_ahb_clock_is_not_ava(&self) -> bool {
        *self == AHB_CLK_ONR::AHB_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_ahb_clock_is_availab(&self) -> bool {
        *self == AHB_CLK_ONR::AHB_CLOCK_IS_AVAILAB
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host clock status."]
    #[inline(always)]
    pub fn host_clk_on(&self) -> HOST_CLK_ON_R {
        HOST_CLK_ON_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline(always)]
    pub fn i2c_clk_on(&self) -> I2C_CLK_ON_R {
        I2C_CLK_ON_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline(always)]
    pub fn otg_clk_on(&self) -> OTG_CLK_ON_R {
        OTG_CLK_ON_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
