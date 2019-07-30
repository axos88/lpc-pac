#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    STOP,
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    START,
}
impl crate::ToBits<bool> for STARTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STARTR::STOP => false,
            STARTR::START => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type START_R = crate::FR<bool, STARTR>;
impl START_R {
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STARTR::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTR::START
    }
}
#[doc = "Values that can be written to the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTW {
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    STOP,
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    START,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::STOP => false,
            STARTW::START => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STARTW::STOP)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW::START)
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Mode 0."]
    MODE_0_,
    #[doc = "Mode 1."]
    MODE_1_,
}
impl crate::ToBits<bool> for MODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MODER::MODE_0_ => false,
            MODER::MODE_1_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE_R = crate::FR<bool, MODER>;
impl MODE_R {
    #[doc = "Checks if the value of the field is `MODE_0_`"]
    #[inline(always)]
    pub fn is_mode_0_(&self) -> bool {
        *self == MODER::MODE_0_
    }
    #[doc = "Checks if the value of the field is `MODE_1_`"]
    #[inline(always)]
    pub fn is_mode_1_(&self) -> bool {
        *self == MODER::MODE_1_
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Mode 0."]
    MODE_0_,
    #[doc = "Mode 1."]
    MODE_1_,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::MODE_0_ => false,
            MODEW::MODE_1_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn mode_0_(self) -> &'a mut W {
        self.variant(MODEW::MODE_0_)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode_1_(self) -> &'a mut W {
        self.variant(MODEW::MODE_1_)
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
#[doc = "Possible values of the field `AUTORESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTORESTARTR {
    #[doc = "No restart"]
    NO_RESTART,
    #[doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    RESTART_IN_CASE_OF_T,
}
impl crate::ToBits<bool> for AUTORESTARTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AUTORESTARTR::NO_RESTART => false,
            AUTORESTARTR::RESTART_IN_CASE_OF_T => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUTORESTART_R = crate::FR<bool, AUTORESTARTR>;
impl AUTORESTART_R {
    #[doc = "Checks if the value of the field is `NO_RESTART`"]
    #[inline(always)]
    pub fn is_no_restart(&self) -> bool {
        *self == AUTORESTARTR::NO_RESTART
    }
    #[doc = "Checks if the value of the field is `RESTART_IN_CASE_OF_T`"]
    #[inline(always)]
    pub fn is_restart_in_case_of_t(&self) -> bool {
        *self == AUTORESTARTR::RESTART_IN_CASE_OF_T
    }
}
#[doc = "Values that can be written to the field `AUTORESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTORESTARTW {
    #[doc = "No restart"]
    NO_RESTART,
    #[doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    RESTART_IN_CASE_OF_T,
}
impl AUTORESTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTORESTARTW::NO_RESTART => false,
            AUTORESTARTW::RESTART_IN_CASE_OF_T => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AUTORESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTORESTARTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTORESTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No restart"]
    #[inline(always)]
    pub fn no_restart(self) -> &'a mut W {
        self.variant(AUTORESTARTW::NO_RESTART)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    #[inline(always)]
    pub fn restart_in_case_of_t(self) -> &'a mut W {
        self.variant(AUTORESTARTW::RESTART_IN_CASE_OF_T)
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
#[doc = "Possible values of the field `ABEOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTCLRR {
    #[doc = "Writing a 0 has no impact."]
    WRITING_A_0_HAS_NO_I,
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    WRITING_A_1_WILL_CLE,
}
impl crate::ToBits<bool> for ABEOINTCLRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABEOINTCLRR::WRITING_A_0_HAS_NO_I => false,
            ABEOINTCLRR::WRITING_A_1_WILL_CLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABEOINTCLR_R = crate::FR<bool, ABEOINTCLRR>;
impl ABEOINTCLR_R {
    #[doc = "Checks if the value of the field is `WRITING_A_0_HAS_NO_I`"]
    #[inline(always)]
    pub fn is_writing_a_0_has_no_i(&self) -> bool {
        *self == ABEOINTCLRR::WRITING_A_0_HAS_NO_I
    }
    #[doc = "Checks if the value of the field is `WRITING_A_1_WILL_CLE`"]
    #[inline(always)]
    pub fn is_writing_a_1_will_cle(&self) -> bool {
        *self == ABEOINTCLRR::WRITING_A_1_WILL_CLE
    }
}
#[doc = "Values that can be written to the field `ABEOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTCLRW {
    #[doc = "Writing a 0 has no impact."]
    WRITING_A_0_HAS_NO_I,
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    WRITING_A_1_WILL_CLE,
}
impl ABEOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTCLRW::WRITING_A_0_HAS_NO_I => false,
            ABEOINTCLRW::WRITING_A_1_WILL_CLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ABEOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn writing_a_0_has_no_i(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::WRITING_A_0_HAS_NO_I)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn writing_a_1_will_cle(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::WRITING_A_1_WILL_CLE)
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
#[doc = "Possible values of the field `ABTOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTCLRR {
    #[doc = "Writing a 0 has no impact."]
    WRITING_A_0_HAS_NO_I,
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    WRITING_A_1_WILL_CLE,
}
impl crate::ToBits<bool> for ABTOINTCLRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABTOINTCLRR::WRITING_A_0_HAS_NO_I => false,
            ABTOINTCLRR::WRITING_A_1_WILL_CLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABTOINTCLR_R = crate::FR<bool, ABTOINTCLRR>;
impl ABTOINTCLR_R {
    #[doc = "Checks if the value of the field is `WRITING_A_0_HAS_NO_I`"]
    #[inline(always)]
    pub fn is_writing_a_0_has_no_i(&self) -> bool {
        *self == ABTOINTCLRR::WRITING_A_0_HAS_NO_I
    }
    #[doc = "Checks if the value of the field is `WRITING_A_1_WILL_CLE`"]
    #[inline(always)]
    pub fn is_writing_a_1_will_cle(&self) -> bool {
        *self == ABTOINTCLRR::WRITING_A_1_WILL_CLE
    }
}
#[doc = "Values that can be written to the field `ABTOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTCLRW {
    #[doc = "Writing a 0 has no impact."]
    WRITING_A_0_HAS_NO_I,
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    WRITING_A_1_WILL_CLE,
}
impl ABTOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTCLRW::WRITING_A_0_HAS_NO_I => false,
            ABTOINTCLRW::WRITING_A_1_WILL_CLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ABTOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn writing_a_0_has_no_i(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::WRITING_A_0_HAS_NO_I)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn writing_a_1_will_cle(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::WRITING_A_1_WILL_CLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto-baud restart bit."]
    #[inline(always)]
    pub fn autorestart(&self) -> AUTORESTART_R {
        AUTORESTART_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abeointclr(&self) -> ABEOINTCLR_R {
        ABEOINTCLR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abtointclr(&self) -> ABTOINTCLR_R {
        ABTOINTCLR_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Auto-baud restart bit."]
    #[inline(always)]
    pub fn autorestart(&mut self) -> _AUTORESTARTW {
        _AUTORESTARTW { w: self }
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abeointclr(&mut self) -> _ABEOINTCLRW {
        _ABEOINTCLRW { w: self }
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abtointclr(&mut self) -> _ABTOINTCLRW {
        _ABTOINTCLRW { w: self }
    }
}
