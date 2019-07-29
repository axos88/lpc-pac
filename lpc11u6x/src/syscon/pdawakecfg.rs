#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDAWAKECFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `IRCOUT_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCOUT_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRCOUT_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRCOUT_PDR::POWERED_DOWN => true,
            IRCOUT_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCOUT_PDR {
        match value {
            true => IRCOUT_PDR::POWERED_DOWN,
            false => IRCOUT_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == IRCOUT_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == IRCOUT_PDR::POWERED
    }
}
#[doc = "Possible values of the field `IRC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRC_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRC_PDR::POWERED_DOWN => true,
            IRC_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRC_PDR {
        match value {
            true => IRC_PDR::POWERED_DOWN,
            false => IRC_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == IRC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == IRC_PDR::POWERED
    }
}
#[doc = "Possible values of the field `FLASH_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl FLASH_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLASH_PDR::POWERED_DOWN => true,
            FLASH_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_PDR {
        match value {
            true => FLASH_PDR::POWERED_DOWN,
            false => FLASH_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PDR::POWERED
    }
}
#[doc = "Possible values of the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl BOD_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BOD_PDR::POWERED_DOWN => true,
            BOD_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOD_PDR {
        match value {
            true => BOD_PDR::POWERED_DOWN,
            false => BOD_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PDR::POWERED
    }
}
#[doc = "Possible values of the field `ADC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl ADC_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADC_PDR::POWERED_DOWN => true,
            ADC_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_PDR {
        match value {
            true => ADC_PDR::POWERED_DOWN,
            false => ADC_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == ADC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == ADC_PDR::POWERED
    }
}
#[doc = "Possible values of the field `SYSOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSOSC_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYSOSC_PDR::POWERED_DOWN => true,
            SYSOSC_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSOSC_PDR {
        match value {
            true => SYSOSC_PDR::POWERED_DOWN,
            false => SYSOSC_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PDR::POWERED
    }
}
#[doc = "Possible values of the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl WDTOSC_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WDTOSC_PDR::POWERED_DOWN => true,
            WDTOSC_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTOSC_PDR {
        match value {
            true => WDTOSC_PDR::POWERED_DOWN,
            false => WDTOSC_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PDR::POWERED
    }
}
#[doc = "Possible values of the field `SYSPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSPLL_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYSPLL_PDR::POWERED_DOWN => true,
            SYSPLL_PDR::POWERED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSPLL_PDR {
        match value {
            true => SYSPLL_PDR::POWERED_DOWN,
            false => SYSPLL_PDR::POWERED,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSPLL_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == SYSPLL_PDR::POWERED
    }
}
#[doc = "Possible values of the field `USBPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPLL_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPLL_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USBPLL_PDR::POWERED => false,
            USBPLL_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPLL_PDR {
        match value {
            false => USBPLL_PDR::POWERED,
            true => USBPLL_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == USBPLL_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPLL_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `USBPAD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPAD_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPAD_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USBPAD_PDR::POWERED => false,
            USBPAD_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBPAD_PDR {
        match value {
            false => USBPAD_PDR::POWERED,
            true => USBPAD_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == USBPAD_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPAD_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `TEMPSENSE_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPSENSE_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl TEMPSENSE_PDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TEMPSENSE_PDR::POWERED => false,
            TEMPSENSE_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEMPSENSE_PDR {
        match value {
            false => TEMPSENSE_PDR::POWERED,
            true => TEMPSENSE_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == TEMPSENSE_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == TEMPSENSE_PDR::POWERED_DOWN
    }
}
#[doc = "Values that can be written to the field `IRCOUT_PD`"]
pub enum IRCOUT_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRCOUT_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRCOUT_PDW::POWERED_DOWN => true,
            IRCOUT_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRCOUT_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _IRCOUT_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRCOUT_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRCOUT_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRCOUT_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRC_PD`"]
pub enum IRC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRC_PDW::POWERED_DOWN => true,
            IRC_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _IRC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRC_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLASH_PD`"]
pub enum FLASH_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl FLASH_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_PDW::POWERED_DOWN => true,
            FLASH_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOD_PD`"]
pub enum BOD_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl BOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_PDW::POWERED_DOWN => true,
            BOD_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_PD`"]
pub enum ADC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl ADC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_PDW::POWERED_DOWN => true,
            ADC_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSOSC_PD`"]
pub enum SYSOSC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSOSC_PDW::POWERED_DOWN => true,
            SYSOSC_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSOSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTOSC_PD`"]
pub enum WDTOSC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl WDTOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDW::POWERED_DOWN => true,
            WDTOSC_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSPLL_PD`"]
pub enum SYSPLL_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSPLL_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSPLL_PDW::POWERED_DOWN => true,
            SYSPLL_PDW::POWERED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSPLL_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSPLL_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSPLL_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::POWERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBPLL_PD`"]
pub enum USBPLL_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPLL_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPLL_PDW::POWERED => false,
            USBPLL_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBPLL_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPLL_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBPLL_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPLL_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPLL_PDW::POWERED_DOWN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBPAD_PD`"]
pub enum USBPAD_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPAD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPAD_PDW::POWERED => false,
            USBPAD_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBPAD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPAD_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBPAD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPAD_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPAD_PDW::POWERED_DOWN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEMPSENSE_PD`"]
pub enum TEMPSENSE_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl TEMPSENSE_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEMPSENSE_PDW::POWERED => false,
            TEMPSENSE_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEMPSENSE_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPSENSE_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEMPSENSE_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(TEMPSENSE_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(TEMPSENSE_PDW::POWERED_DOWN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline]
    pub fn ircout_pd(&self) -> IRCOUT_PDR {
        IRCOUT_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline]
    pub fn irc_pd(&self) -> IRC_PDR {
        IRC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline]
    pub fn flash_pd(&self) -> FLASH_PDR {
        FLASH_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline]
    pub fn bod_pd(&self) -> BOD_PDR {
        BOD_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline]
    pub fn adc_pd(&self) -> ADC_PDR {
        ADC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline]
    pub fn sysosc_pd(&self) -> SYSOSC_PDR {
        SYSOSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline]
    pub fn wdtosc_pd(&self) -> WDTOSC_PDR {
        WDTOSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline]
    pub fn syspll_pd(&self) -> SYSPLL_PDR {
        SYSPLL_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline]
    pub fn usbpll_pd(&self) -> USBPLL_PDR {
        USBPLL_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline]
    pub fn usbpad_pd(&self) -> USBPAD_PDR {
        USBPAD_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Temperature sensor wake-up configuration"]
    #[inline]
    pub fn tempsense_pd(&self) -> TEMPSENSE_PDR {
        TEMPSENSE_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline]
    pub fn ircout_pd(&mut self) -> _IRCOUT_PDW {
        _IRCOUT_PDW { w: self }
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline]
    pub fn irc_pd(&mut self) -> _IRC_PDW {
        _IRC_PDW { w: self }
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline]
    pub fn flash_pd(&mut self) -> _FLASH_PDW {
        _FLASH_PDW { w: self }
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline]
    pub fn bod_pd(&mut self) -> _BOD_PDW {
        _BOD_PDW { w: self }
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline]
    pub fn adc_pd(&mut self) -> _ADC_PDW {
        _ADC_PDW { w: self }
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline]
    pub fn sysosc_pd(&mut self) -> _SYSOSC_PDW {
        _SYSOSC_PDW { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline]
    pub fn wdtosc_pd(&mut self) -> _WDTOSC_PDW {
        _WDTOSC_PDW { w: self }
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline]
    pub fn syspll_pd(&mut self) -> _SYSPLL_PDW {
        _SYSPLL_PDW { w: self }
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline]
    pub fn usbpll_pd(&mut self) -> _USBPLL_PDW {
        _USBPLL_PDW { w: self }
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline]
    pub fn usbpad_pd(&mut self) -> _USBPAD_PDW {
        _USBPAD_PDW { w: self }
    }
    #[doc = "Bit 13 - Temperature sensor wake-up configuration"]
    #[inline]
    pub fn tempsense_pd(&mut self) -> _TEMPSENSE_PDW {
        _TEMPSENSE_PDW { w: self }
    }
}
