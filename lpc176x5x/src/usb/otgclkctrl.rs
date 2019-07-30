#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OTGCLKCTRL {
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
#[doc = "Possible values of the field `HOST_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ENR {
    #[doc = "Disable the Host clock."]
    DISABLE_THE_HOST_CLO,
    #[doc = "Enable the Host clock."]
    ENABLE_THE_HOST_CLOC,
}
impl crate::ToBits<bool> for HOST_CLK_ENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HOST_CLK_ENR::DISABLE_THE_HOST_CLO => false,
            HOST_CLK_ENR::ENABLE_THE_HOST_CLOC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HOST_CLK_EN_R = crate::FR<bool, HOST_CLK_ENR>;
impl HOST_CLK_EN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_HOST_CLO`"]
    #[inline(always)]
    pub fn is_disable_the_host_clo(&self) -> bool {
        *self == HOST_CLK_ENR::DISABLE_THE_HOST_CLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_HOST_CLOC`"]
    #[inline(always)]
    pub fn is_enable_the_host_cloc(&self) -> bool {
        *self == HOST_CLK_ENR::ENABLE_THE_HOST_CLOC
    }
}
#[doc = "Values that can be written to the field `HOST_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ENW {
    #[doc = "Disable the Host clock."]
    DISABLE_THE_HOST_CLO,
    #[doc = "Enable the Host clock."]
    ENABLE_THE_HOST_CLOC,
}
impl HOST_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HOST_CLK_ENW::DISABLE_THE_HOST_CLO => false,
            HOST_CLK_ENW::ENABLE_THE_HOST_CLOC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HOST_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_CLK_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOST_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn disable_the_host_clo(self) -> &'a mut W {
        self.variant(HOST_CLK_ENW::DISABLE_THE_HOST_CLO)
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn enable_the_host_cloc(self) -> &'a mut W {
        self.variant(HOST_CLK_ENW::ENABLE_THE_HOST_CLOC)
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
#[doc = "Possible values of the field `DEV_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ENR {
    #[doc = "Disable the Device clock."]
    DISABLE_THE_DEVICE_C,
    #[doc = "Enable the Device clock."]
    ENABLE_THE_DEVICE_CL,
}
impl crate::ToBits<bool> for DEV_CLK_ENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DEV_CLK_ENR::DISABLE_THE_DEVICE_C => false,
            DEV_CLK_ENR::ENABLE_THE_DEVICE_CL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DEV_CLK_EN_R = crate::FR<bool, DEV_CLK_ENR>;
impl DEV_CLK_EN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_DEVICE_C`"]
    #[inline(always)]
    pub fn is_disable_the_device_c(&self) -> bool {
        *self == DEV_CLK_ENR::DISABLE_THE_DEVICE_C
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DEVICE_CL`"]
    #[inline(always)]
    pub fn is_enable_the_device_cl(&self) -> bool {
        *self == DEV_CLK_ENR::ENABLE_THE_DEVICE_CL
    }
}
#[doc = "Values that can be written to the field `DEV_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ENW {
    #[doc = "Disable the Device clock."]
    DISABLE_THE_DEVICE_C,
    #[doc = "Enable the Device clock."]
    ENABLE_THE_DEVICE_CL,
}
impl DEV_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DEV_CLK_ENW::DISABLE_THE_DEVICE_C => false,
            DEV_CLK_ENW::ENABLE_THE_DEVICE_CL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DEV_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_CLK_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn disable_the_device_c(self) -> &'a mut W {
        self.variant(DEV_CLK_ENW::DISABLE_THE_DEVICE_C)
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn enable_the_device_cl(self) -> &'a mut W {
        self.variant(DEV_CLK_ENW::ENABLE_THE_DEVICE_CL)
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
#[doc = "Possible values of the field `I2C_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ENR {
    #[doc = "Disable the I2C clock."]
    DISABLE_THE_I2C_CLOC,
    #[doc = "Enable the I2C clock."]
    ENABLE_THE_I2C_CLOCK,
}
impl crate::ToBits<bool> for I2C_CLK_ENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C_CLK_ENR::DISABLE_THE_I2C_CLOC => false,
            I2C_CLK_ENR::ENABLE_THE_I2C_CLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C_CLK_EN_R = crate::FR<bool, I2C_CLK_ENR>;
impl I2C_CLK_EN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_I2C_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_i2c_cloc(&self) -> bool {
        *self == I2C_CLK_ENR::DISABLE_THE_I2C_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_I2C_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_i2c_clock(&self) -> bool {
        *self == I2C_CLK_ENR::ENABLE_THE_I2C_CLOCK
    }
}
#[doc = "Values that can be written to the field `I2C_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ENW {
    #[doc = "Disable the I2C clock."]
    DISABLE_THE_I2C_CLOC,
    #[doc = "Enable the I2C clock."]
    ENABLE_THE_I2C_CLOCK,
}
impl I2C_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C_CLK_ENW::DISABLE_THE_I2C_CLOC => false,
            I2C_CLK_ENW::ENABLE_THE_I2C_CLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_CLK_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn disable_the_i2c_cloc(self) -> &'a mut W {
        self.variant(I2C_CLK_ENW::DISABLE_THE_I2C_CLOC)
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn enable_the_i2c_clock(self) -> &'a mut W {
        self.variant(I2C_CLK_ENW::ENABLE_THE_I2C_CLOCK)
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
#[doc = "Possible values of the field `OTG_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ENR {
    #[doc = "Disable the OTG clock."]
    DISABLE_THE_OTG_CLOC,
    #[doc = "Enable the OTG clock."]
    ENABLE_THE_OTG_CLOCK,
}
impl crate::ToBits<bool> for OTG_CLK_ENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OTG_CLK_ENR::DISABLE_THE_OTG_CLOC => false,
            OTG_CLK_ENR::ENABLE_THE_OTG_CLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OTG_CLK_EN_R = crate::FR<bool, OTG_CLK_ENR>;
impl OTG_CLK_EN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_OTG_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_otg_cloc(&self) -> bool {
        *self == OTG_CLK_ENR::DISABLE_THE_OTG_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_OTG_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_otg_clock(&self) -> bool {
        *self == OTG_CLK_ENR::ENABLE_THE_OTG_CLOCK
    }
}
#[doc = "Values that can be written to the field `OTG_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ENW {
    #[doc = "Disable the OTG clock."]
    DISABLE_THE_OTG_CLOC,
    #[doc = "Enable the OTG clock."]
    ENABLE_THE_OTG_CLOCK,
}
impl OTG_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OTG_CLK_ENW::DISABLE_THE_OTG_CLOC => false,
            OTG_CLK_ENW::ENABLE_THE_OTG_CLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OTG_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG_CLK_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTG_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn disable_the_otg_cloc(self) -> &'a mut W {
        self.variant(OTG_CLK_ENW::DISABLE_THE_OTG_CLOC)
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn enable_the_otg_clock(self) -> &'a mut W {
        self.variant(OTG_CLK_ENW::ENABLE_THE_OTG_CLOCK)
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
#[doc = "Possible values of the field `AHB_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ENR {
    #[doc = "Disable the AHB clock."]
    DISABLE_THE_AHB_CLOC,
    #[doc = "Enable the AHB clock."]
    ENABLE_THE_AHB_CLOCK,
}
impl crate::ToBits<bool> for AHB_CLK_ENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AHB_CLK_ENR::DISABLE_THE_AHB_CLOC => false,
            AHB_CLK_ENR::ENABLE_THE_AHB_CLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AHB_CLK_EN_R = crate::FR<bool, AHB_CLK_ENR>;
impl AHB_CLK_EN_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_AHB_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_ahb_cloc(&self) -> bool {
        *self == AHB_CLK_ENR::DISABLE_THE_AHB_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_AHB_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_ahb_clock(&self) -> bool {
        *self == AHB_CLK_ENR::ENABLE_THE_AHB_CLOCK
    }
}
#[doc = "Values that can be written to the field `AHB_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ENW {
    #[doc = "Disable the AHB clock."]
    DISABLE_THE_AHB_CLOC,
    #[doc = "Enable the AHB clock."]
    ENABLE_THE_AHB_CLOCK,
}
impl AHB_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_CLK_ENW::DISABLE_THE_AHB_CLOC => false,
            AHB_CLK_ENW::ENABLE_THE_AHB_CLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AHB_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_CLK_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn disable_the_ahb_cloc(self) -> &'a mut W {
        self.variant(AHB_CLK_ENW::DISABLE_THE_AHB_CLOC)
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn enable_the_ahb_clock(self) -> &'a mut W {
        self.variant(AHB_CLK_ENW::ENABLE_THE_AHB_CLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&self) -> HOST_CLK_EN_R {
        HOST_CLK_EN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DEV_CLK_EN_R {
        DEV_CLK_EN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&self) -> OTG_CLK_EN_R {
        OTG_CLK_EN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AHB_CLK_EN_R {
        AHB_CLK_EN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&mut self) -> _HOST_CLK_ENW {
        _HOST_CLK_ENW { w: self }
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&mut self) -> _DEV_CLK_ENW {
        _DEV_CLK_ENW { w: self }
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> _I2C_CLK_ENW {
        _I2C_CLK_ENW { w: self }
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&mut self) -> _OTG_CLK_ENW {
        _OTG_CLK_ENW { w: self }
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&mut self) -> _AHB_CLK_ENW {
        _AHB_CLK_ENW { w: self }
    }
}
