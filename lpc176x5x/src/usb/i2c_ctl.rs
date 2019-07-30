#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2C_CTL {
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
#[doc = "Possible values of the field `TDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIER {
    #[doc = "Disable the TDI interrupt."]
    DISABLE_THE_TDI_INTE,
    #[doc = "Enable the TDI interrupt."]
    ENABLE_THE_TDI_INTER,
}
impl crate::ToBits<bool> for TDIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TDIER::DISABLE_THE_TDI_INTE => false,
            TDIER::ENABLE_THE_TDI_INTER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TDIE_R = crate::FR<bool, TDIER>;
impl TDIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_TDI_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_tdi_inte(&self) -> bool {
        *self == TDIER::DISABLE_THE_TDI_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_TDI_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_tdi_inter(&self) -> bool {
        *self == TDIER::ENABLE_THE_TDI_INTER
    }
}
#[doc = "Values that can be written to the field `TDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIEW {
    #[doc = "Disable the TDI interrupt."]
    DISABLE_THE_TDI_INTE,
    #[doc = "Enable the TDI interrupt."]
    ENABLE_THE_TDI_INTER,
}
impl TDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TDIEW::DISABLE_THE_TDI_INTE => false,
            TDIEW::ENABLE_THE_TDI_INTER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn disable_the_tdi_inte(self) -> &'a mut W {
        self.variant(TDIEW::DISABLE_THE_TDI_INTE)
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn enable_the_tdi_inter(self) -> &'a mut W {
        self.variant(TDIEW::ENABLE_THE_TDI_INTER)
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
#[doc = "Possible values of the field `AFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIER {
    #[doc = "Disable the AFI."]
    DISABLE_THE_AFI_,
    #[doc = "Enable the AFI."]
    ENABLE_THE_AFI_,
}
impl crate::ToBits<bool> for AFIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AFIER::DISABLE_THE_AFI_ => false,
            AFIER::ENABLE_THE_AFI_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AFIE_R = crate::FR<bool, AFIER>;
impl AFIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_AFI_`"]
    #[inline(always)]
    pub fn is_disable_the_afi_(&self) -> bool {
        *self == AFIER::DISABLE_THE_AFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_AFI_`"]
    #[inline(always)]
    pub fn is_enable_the_afi_(&self) -> bool {
        *self == AFIER::ENABLE_THE_AFI_
    }
}
#[doc = "Values that can be written to the field `AFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIEW {
    #[doc = "Disable the AFI."]
    DISABLE_THE_AFI_,
    #[doc = "Enable the AFI."]
    ENABLE_THE_AFI_,
}
impl AFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AFIEW::DISABLE_THE_AFI_ => false,
            AFIEW::ENABLE_THE_AFI_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AFIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn disable_the_afi_(self) -> &'a mut W {
        self.variant(AFIEW::DISABLE_THE_AFI_)
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn enable_the_afi_(self) -> &'a mut W {
        self.variant(AFIEW::ENABLE_THE_AFI_)
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
#[doc = "Possible values of the field `NAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIER {
    #[doc = "Disable the NAI."]
    DISABLE_THE_NAI_,
    #[doc = "Enable the NAI."]
    ENABLE_THE_NAI_,
}
impl crate::ToBits<bool> for NAIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NAIER::DISABLE_THE_NAI_ => false,
            NAIER::ENABLE_THE_NAI_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NAIE_R = crate::FR<bool, NAIER>;
impl NAIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_NAI_`"]
    #[inline(always)]
    pub fn is_disable_the_nai_(&self) -> bool {
        *self == NAIER::DISABLE_THE_NAI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_NAI_`"]
    #[inline(always)]
    pub fn is_enable_the_nai_(&self) -> bool {
        *self == NAIER::ENABLE_THE_NAI_
    }
}
#[doc = "Values that can be written to the field `NAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIEW {
    #[doc = "Disable the NAI."]
    DISABLE_THE_NAI_,
    #[doc = "Enable the NAI."]
    ENABLE_THE_NAI_,
}
impl NAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NAIEW::DISABLE_THE_NAI_ => false,
            NAIEW::ENABLE_THE_NAI_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NAIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn disable_the_nai_(self) -> &'a mut W {
        self.variant(NAIEW::DISABLE_THE_NAI_)
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn enable_the_nai_(self) -> &'a mut W {
        self.variant(NAIEW::ENABLE_THE_NAI_)
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
#[doc = "Possible values of the field `DRMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIER {
    #[doc = "Disable the DRMI interrupt."]
    DISABLE_THE_DRMI_INT,
    #[doc = "Enable the DRMI interrupt."]
    ENABLE_THE_DRMI_INTE,
}
impl crate::ToBits<bool> for DRMIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DRMIER::DISABLE_THE_DRMI_INT => false,
            DRMIER::ENABLE_THE_DRMI_INTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DRMIE_R = crate::FR<bool, DRMIER>;
impl DRMIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_DRMI_INT`"]
    #[inline(always)]
    pub fn is_disable_the_drmi_int(&self) -> bool {
        *self == DRMIER::DISABLE_THE_DRMI_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DRMI_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_drmi_inte(&self) -> bool {
        *self == DRMIER::ENABLE_THE_DRMI_INTE
    }
}
#[doc = "Values that can be written to the field `DRMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIEW {
    #[doc = "Disable the DRMI interrupt."]
    DISABLE_THE_DRMI_INT,
    #[doc = "Enable the DRMI interrupt."]
    ENABLE_THE_DRMI_INTE,
}
impl DRMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DRMIEW::DISABLE_THE_DRMI_INT => false,
            DRMIEW::ENABLE_THE_DRMI_INTE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DRMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRMIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn disable_the_drmi_int(self) -> &'a mut W {
        self.variant(DRMIEW::DISABLE_THE_DRMI_INT)
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn enable_the_drmi_inte(self) -> &'a mut W {
        self.variant(DRMIEW::ENABLE_THE_DRMI_INTE)
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
#[doc = "Possible values of the field `DRSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIER {
    #[doc = "Disable the DRSI interrupt."]
    DISABLE_THE_DRSI_INT,
    #[doc = "Enable the DRSI interrupt."]
    ENABLE_THE_DRSI_INTE,
}
impl crate::ToBits<bool> for DRSIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DRSIER::DISABLE_THE_DRSI_INT => false,
            DRSIER::ENABLE_THE_DRSI_INTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DRSIE_R = crate::FR<bool, DRSIER>;
impl DRSIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_DRSI_INT`"]
    #[inline(always)]
    pub fn is_disable_the_drsi_int(&self) -> bool {
        *self == DRSIER::DISABLE_THE_DRSI_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DRSI_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_drsi_inte(&self) -> bool {
        *self == DRSIER::ENABLE_THE_DRSI_INTE
    }
}
#[doc = "Values that can be written to the field `DRSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIEW {
    #[doc = "Disable the DRSI interrupt."]
    DISABLE_THE_DRSI_INT,
    #[doc = "Enable the DRSI interrupt."]
    ENABLE_THE_DRSI_INTE,
}
impl DRSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DRSIEW::DISABLE_THE_DRSI_INT => false,
            DRSIEW::ENABLE_THE_DRSI_INTE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DRSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRSIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn disable_the_drsi_int(self) -> &'a mut W {
        self.variant(DRSIEW::DISABLE_THE_DRSI_INT)
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn enable_the_drsi_inte(self) -> &'a mut W {
        self.variant(DRSIEW::ENABLE_THE_DRSI_INTE)
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
#[doc = "Possible values of the field `REFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFIER {
    #[doc = "Disable the RFFI."]
    DISABLE_THE_RFFI_,
    #[doc = "Enable the RFFI."]
    ENABLE_THE_RFFI_,
}
impl crate::ToBits<bool> for REFIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            REFIER::DISABLE_THE_RFFI_ => false,
            REFIER::ENABLE_THE_RFFI_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type REFIE_R = crate::FR<bool, REFIER>;
impl REFIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_RFFI_`"]
    #[inline(always)]
    pub fn is_disable_the_rffi_(&self) -> bool {
        *self == REFIER::DISABLE_THE_RFFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RFFI_`"]
    #[inline(always)]
    pub fn is_enable_the_rffi_(&self) -> bool {
        *self == REFIER::ENABLE_THE_RFFI_
    }
}
#[doc = "Values that can be written to the field `REFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFIEW {
    #[doc = "Disable the RFFI."]
    DISABLE_THE_RFFI_,
    #[doc = "Enable the RFFI."]
    ENABLE_THE_RFFI_,
}
impl REFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            REFIEW::DISABLE_THE_RFFI_ => false,
            REFIEW::ENABLE_THE_RFFI_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _REFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REFIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn disable_the_rffi_(self) -> &'a mut W {
        self.variant(REFIEW::DISABLE_THE_RFFI_)
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn enable_the_rffi_(self) -> &'a mut W {
        self.variant(REFIEW::ENABLE_THE_RFFI_)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `RFDAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDAIER {
    #[doc = "Disable the DAI."]
    DISABLE_THE_DAI_,
    #[doc = "Enable the DAI."]
    ENABLE_THE_DAI_,
}
impl crate::ToBits<bool> for RFDAIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RFDAIER::DISABLE_THE_DAI_ => false,
            RFDAIER::ENABLE_THE_DAI_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RFDAIE_R = crate::FR<bool, RFDAIER>;
impl RFDAIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_DAI_`"]
    #[inline(always)]
    pub fn is_disable_the_dai_(&self) -> bool {
        *self == RFDAIER::DISABLE_THE_DAI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DAI_`"]
    #[inline(always)]
    pub fn is_enable_the_dai_(&self) -> bool {
        *self == RFDAIER::ENABLE_THE_DAI_
    }
}
#[doc = "Values that can be written to the field `RFDAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDAIEW {
    #[doc = "Disable the DAI."]
    DISABLE_THE_DAI_,
    #[doc = "Enable the DAI."]
    ENABLE_THE_DAI_,
}
impl RFDAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RFDAIEW::DISABLE_THE_DAI_ => false,
            RFDAIEW::ENABLE_THE_DAI_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RFDAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDAIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn disable_the_dai_(self) -> &'a mut W {
        self.variant(RFDAIEW::DISABLE_THE_DAI_)
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn enable_the_dai_(self) -> &'a mut W {
        self.variant(RFDAIEW::ENABLE_THE_DAI_)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `TFFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFIER {
    #[doc = "Disable the TFFI."]
    DISABLE_THE_TFFI_,
    #[doc = "Enable the TFFI."]
    ENABLE_THE_TFFI_,
}
impl crate::ToBits<bool> for TFFIER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TFFIER::DISABLE_THE_TFFI_ => false,
            TFFIER::ENABLE_THE_TFFI_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TFFIE_R = crate::FR<bool, TFFIER>;
impl TFFIE_R {
    #[doc = "Checks if the value of the field is `DISABLE_THE_TFFI_`"]
    #[inline(always)]
    pub fn is_disable_the_tffi_(&self) -> bool {
        *self == TFFIER::DISABLE_THE_TFFI_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_TFFI_`"]
    #[inline(always)]
    pub fn is_enable_the_tffi_(&self) -> bool {
        *self == TFFIER::ENABLE_THE_TFFI_
    }
}
#[doc = "Values that can be written to the field `TFFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFIEW {
    #[doc = "Disable the TFFI."]
    DISABLE_THE_TFFI_,
    #[doc = "Enable the TFFI."]
    ENABLE_THE_TFFI_,
}
impl TFFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFIEW::DISABLE_THE_TFFI_ => false,
            TFFIEW::ENABLE_THE_TFFI_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TFFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFIEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn disable_the_tffi_(self) -> &'a mut W {
        self.variant(TFFIEW::DISABLE_THE_TFFI_)
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn enable_the_tffi_(self) -> &'a mut W {
        self.variant(TFFIEW::ENABLE_THE_TFFI_)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `SRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTR {
    #[doc = "No reset."]
    NO_RESET,
    #[doc = "Reset the I2C to idle state. Self clearing."]
    RESET,
}
impl crate::ToBits<bool> for SRSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SRSTR::NO_RESET => false,
            SRSTR::RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRST_R = crate::FR<bool, SRSTR>;
impl SRST_R {
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SRSTR::NO_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `SRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTW {
    #[doc = "No reset."]
    NO_RESET,
    #[doc = "Reset the I2C to idle state. Self clearing."]
    RESET,
}
impl SRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SRSTW::NO_RESET => false,
            SRSTW::RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(SRSTW::NO_RESET)
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRSTW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&self) -> AFIE_R {
        AFIE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&self) -> NAIE_R {
        NAIE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&self) -> DRMIE_R {
        DRMIE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&self) -> DRSIE_R {
        DRSIE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&self) -> REFIE_R {
        REFIE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&self) -> RFDAIE_R {
        RFDAIE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&mut self) -> _TDIEW {
        _TDIEW { w: self }
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&mut self) -> _AFIEW {
        _AFIEW { w: self }
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&mut self) -> _NAIEW {
        _NAIEW { w: self }
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&mut self) -> _DRMIEW {
        _DRMIEW { w: self }
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&mut self) -> _DRSIEW {
        _DRSIEW { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&mut self) -> _REFIEW {
        _REFIEW { w: self }
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&mut self) -> _RFDAIEW {
        _RFDAIEW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&mut self) -> _TFFIEW {
        _TFFIEW { w: self }
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&mut self) -> _SRSTW {
        _SRSTW { w: self }
    }
}
