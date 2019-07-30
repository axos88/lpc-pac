#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTF {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ILIM0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ILIM0_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM0_FR::THIS_INTERRUPT_SOURC => false,
            ILIM0_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM0_F_R = crate::FR<bool, ILIM0_FR>;
impl ILIM0_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for IMAT0_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT0_FR::THIS_INTERRUPT_SOURC => false,
            IMAT0_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT0_F_R = crate::FR<bool, IMAT0_FR>;
impl IMAT0_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ICAP0_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP0_FR::THIS_INTERRUPT_SOURC => false,
            ICAP0_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP0_F_R = crate::FR<bool, ICAP0_FR>;
impl ICAP0_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ILIM1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ILIM1_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM1_FR::THIS_INTERRUPT_SOURC => false,
            ILIM1_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM1_F_R = crate::FR<bool, ILIM1_FR>;
impl ILIM1_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for IMAT1_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT1_FR::THIS_INTERRUPT_SOURC => false,
            IMAT1_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT1_F_R = crate::FR<bool, IMAT1_FR>;
impl IMAT1_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ICAP1_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP1_FR::THIS_INTERRUPT_SOURC => false,
            ICAP1_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP1_F_R = crate::FR<bool, ICAP1_FR>;
impl ICAP1_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ILIM2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ILIM2_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ILIM2_FR::THIS_INTERRUPT_SOURC => false,
            ILIM2_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ILIM2_F_R = crate::FR<bool, ILIM2_FR>;
impl ILIM2_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for IMAT2_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMAT2_FR::THIS_INTERRUPT_SOURC => false,
            IMAT2_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMAT2_F_R = crate::FR<bool, IMAT2_FR>;
impl IMAT2_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ICAP2_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ICAP2_FR::THIS_INTERRUPT_SOURC => false,
            ICAP2_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ICAP2_F_R = crate::FR<bool, ICAP2_FR>;
impl ICAP2_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ABORT_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl crate::ToBits<bool> for ABORT_FR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ABORT_FR::THIS_INTERRUPT_SOURC => false,
            ABORT_FR::IF_THE_CORRESPONDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ABORT_F_R = crate::FR<bool, ABORT_FR>;
impl ABORT_F_R {
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ABORT_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ABORT_FR::IF_THE_CORRESPONDING
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> ILIM0_F_R {
        ILIM0_F_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> IMAT0_F_R {
        IMAT0_F_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> ICAP0_F_R {
        ICAP0_F_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> ILIM1_F_R {
        ILIM1_F_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> IMAT1_F_R {
        IMAT1_F_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> ICAP1_F_R {
        ICAP1_F_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> ILIM2_F_R {
        ILIM2_F_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> IMAT2_F_R {
        IMAT2_F_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> ICAP2_F_R {
        ICAP2_F_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> ABORT_F_R {
        ABORT_F_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
