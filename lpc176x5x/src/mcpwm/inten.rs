#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTEN {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ILIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ILIM0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM0R::INTERRUPT_DISABLED_ => false,
            ILIM0R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM0_R = crate::FR<bool, ILIM0R>;
impl ILIM0_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for IMAT0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT0R::INTERRUPT_DISABLED_ => false,
            IMAT0R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT0_R = crate::FR<bool, IMAT0R>;
impl IMAT0_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ICAP0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP0R::INTERRUPT_DISABLED_ => false,
            ICAP0R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP0_R = crate::FR<bool, ICAP0R>;
impl ICAP0_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ILIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ILIM1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM1R::INTERRUPT_DISABLED_ => false,
            ILIM1R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM1_R = crate::FR<bool, ILIM1R>;
impl ILIM1_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for IMAT1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT1R::INTERRUPT_DISABLED_ => false,
            IMAT1R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT1_R = crate::FR<bool, IMAT1R>;
impl IMAT1_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ICAP1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP1R::INTERRUPT_DISABLED_ => false,
            ICAP1R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP1_R = crate::FR<bool, ICAP1R>;
impl ICAP1_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ILIM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ILIM2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM2R::INTERRUPT_DISABLED_ => false,
            ILIM2R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM2_R = crate::FR<bool, ILIM2R>;
impl ILIM2_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for IMAT2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT2R::INTERRUPT_DISABLED_ => false,
            IMAT2R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT2_R = crate::FR<bool, IMAT2R>;
impl IMAT2_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ICAP2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP2R::INTERRUPT_DISABLED_ => false,
            ICAP2R::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP2_R = crate::FR<bool, ICAP2R>;
impl ICAP2_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORTR {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl crate::ToBits<bool> for ABORTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABORTR::INTERRUPT_DISABLED_ => false,
            ABORTR::INTERRUPT_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABORT_R = crate::FR<bool, ABORTR>;
impl ABORT_R {
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ABORTR::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ABORTR::INTERRUPT_ENABLED_
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline(always)]
    pub fn ilim0(&self) -> ILIM0_R {
        ILIM0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline(always)]
    pub fn imat0(&self) -> IMAT0_R {
        IMAT0_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline(always)]
    pub fn icap0(&self) -> ICAP0_R {
        ICAP0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline(always)]
    pub fn ilim1(&self) -> ILIM1_R {
        ILIM1_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline(always)]
    pub fn imat1(&self) -> IMAT1_R {
        IMAT1_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline(always)]
    pub fn icap1(&self) -> ICAP1_R {
        ICAP1_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline(always)]
    pub fn ilim2(&self) -> ILIM2_R {
        ILIM2_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline(always)]
    pub fn imat2(&self) -> IMAT2_R {
        IMAT2_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline(always)]
    pub fn icap2(&self) -> ICAP2_R {
        ICAP2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
