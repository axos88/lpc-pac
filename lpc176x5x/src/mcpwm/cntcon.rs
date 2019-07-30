#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CNTCON {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TC0MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    RISING,
}
impl crate::ToBits<bool> for TC0MCI0_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI0_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI0_RE_R = crate::FR<bool, TC0MCI0_RER>;
impl TC0MCI0_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    FALLING,
}
impl crate::ToBits<bool> for TC0MCI0_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI0_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI0_FE_R = crate::FR<bool, TC0MCI0_FER>;
impl TC0MCI0_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC0MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    RISING,
}
impl crate::ToBits<bool> for TC0MCI1_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI1_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI1_RE_R = crate::FR<bool, TC0MCI1_RER>;
impl TC0MCI1_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_FER {
    #[doc = "A falling edge on MCI1 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    FALLING,
}
impl crate::ToBits<bool> for TC0MCI1_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI1_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI1_FE_R = crate::FR<bool, TC0MCI1_FER>;
impl TC0MCI1_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC0MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    RISING,
}
impl crate::ToBits<bool> for TC0MCI2_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI2_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI2_RE_R = crate::FR<bool, TC0MCI2_RER>;
impl TC0MCI2_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI2_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    FALLLING,
}
impl crate::ToBits<bool> for TC0MCI2_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC0MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI2_FER::FALLLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0MCI2_FE_R = crate::FR<bool, TC0MCI2_FER>;
impl TC0MCI2_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == TC0MCI2_FER::FALLLING
    }
}
#[doc = "Possible values of the field `TC1MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    RISING,
}
impl crate::ToBits<bool> for TC1MCI0_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI0_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI0_RE_R = crate::FR<bool, TC1MCI0_RER>;
impl TC1MCI0_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    FALLING,
}
impl crate::ToBits<bool> for TC1MCI0_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI0_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI0_FE_R = crate::FR<bool, TC1MCI0_FER>;
impl TC1MCI0_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC1MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    RISING,
}
impl crate::ToBits<bool> for TC1MCI1_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI1_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI1_RE_R = crate::FR<bool, TC1MCI1_RER>;
impl TC1MCI1_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    FALLING,
}
impl crate::ToBits<bool> for TC1MCI1_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI1_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI1_FE_R = crate::FR<bool, TC1MCI1_FER>;
impl TC1MCI1_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC1MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_RER {
    #[doc = "A rising edge on MCI2 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    RISING,
}
impl crate::ToBits<bool> for TC1MCI2_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI2_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI2_RE_R = crate::FR<bool, TC1MCI2_RER>;
impl TC1MCI2_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI2_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_FER {
    #[doc = "A falling edge on MCI2 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    FALLING,
}
impl crate::ToBits<bool> for TC1MCI2_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC1MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI2_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1MCI2_FE_R = crate::FR<bool, TC1MCI2_FER>;
impl TC1MCI2_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI2_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    RISING,
}
impl crate::ToBits<bool> for TC2MCI0_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI0_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI0_RE_R = crate::FR<bool, TC2MCI0_RER>;
impl TC2MCI0_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC2MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    FALLING,
}
impl crate::ToBits<bool> for TC2MCI0_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI0_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI0_FE_R = crate::FR<bool, TC2MCI0_FER>;
impl TC2MCI0_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    RISING,
}
impl crate::ToBits<bool> for TC2MCI1_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI1_RER::RISING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI1_RE_R = crate::FR<bool, TC2MCI1_RER>;
impl TC2MCI1_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC2MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_FER {
    #[doc = "A falling edge on MCI1 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    FALLING,
}
impl crate::ToBits<bool> for TC2MCI1_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI1_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI1_FE_R = crate::FR<bool, TC2MCI1_FER>;
impl TC2MCI1_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_RER {
    #[doc = "A rising edge on MCI2 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    RISIING,
}
impl crate::ToBits<bool> for TC2MCI2_RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI2_RER::RISIING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI2_RE_R = crate::FR<bool, TC2MCI2_RER>;
impl TC2MCI2_RE_R {
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISIING`"]
    #[inline(always)]
    pub fn is_risiing(&self) -> bool {
        *self == TC2MCI2_RER::RISIING
    }
}
#[doc = "Possible values of the field `TC2MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_FER {
    #[doc = "A falling edge on MCI2 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    FALLING,
}
impl crate::ToBits<bool> for TC2MCI2_FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TC2MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI2_FER::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2MCI2_FE_R = crate::FR<bool, TC2MCI2_FER>;
impl TC2MCI2_FE_R {
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI2_FER::FALLING
    }
}
#[doc = "Possible values of the field `CNTR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR0R {
    #[doc = "Channel 0 is in timer mode."]
    CHANNEL_0_IS_IN_TIME,
    #[doc = "Channel 0 is in counter mode."]
    CHANNEL_0_IS_IN_COUN,
}
impl crate::ToBits<bool> for CNTR0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CNTR0R::CHANNEL_0_IS_IN_TIME => false,
            CNTR0R::CHANNEL_0_IS_IN_COUN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CNTR0_R = crate::FR<bool, CNTR0R>;
impl CNTR0_R {
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_0_is_in_time(&self) -> bool {
        *self == CNTR0R::CHANNEL_0_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_0_is_in_coun(&self) -> bool {
        *self == CNTR0R::CHANNEL_0_IS_IN_COUN
    }
}
#[doc = "Possible values of the field `CNTR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR1R {
    #[doc = "Channel 1 is in timer mode."]
    CHANNEL_1_IS_IN_TIME,
    #[doc = "Channel 1 is in counter mode."]
    CHANNEL_1_IS_IN_COUN,
}
impl crate::ToBits<bool> for CNTR1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CNTR1R::CHANNEL_1_IS_IN_TIME => false,
            CNTR1R::CHANNEL_1_IS_IN_COUN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CNTR1_R = crate::FR<bool, CNTR1R>;
impl CNTR1_R {
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_1_is_in_time(&self) -> bool {
        *self == CNTR1R::CHANNEL_1_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_1_is_in_coun(&self) -> bool {
        *self == CNTR1R::CHANNEL_1_IS_IN_COUN
    }
}
#[doc = "Possible values of the field `CNTR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR2R {
    #[doc = "Channel 2 is in timer mode."]
    CHANNEL_2_IS_IN_TIME,
    #[doc = "Channel 2 is in counter mode."]
    CHANNEL_2_IS_IN_COUN,
}
impl crate::ToBits<bool> for CNTR2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CNTR2R::CHANNEL_2_IS_IN_TIME => false,
            CNTR2R::CHANNEL_2_IS_IN_COUN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CNTR2_R = crate::FR<bool, CNTR2R>;
impl CNTR2_R {
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_2_is_in_time(&self) -> bool {
        *self == CNTR2R::CHANNEL_2_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_2_is_in_coun(&self) -> bool {
        *self == CNTR2R::CHANNEL_2_IS_IN_COUN
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter 0 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_re(&self) -> TC0MCI0_RE_R {
        TC0MCI0_RE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter 0 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_fe(&self) -> TC0MCI0_FE_R {
        TC0MCI0_FE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter 0 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_re(&self) -> TC0MCI1_RE_R {
        TC0MCI1_RE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter 0 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_fe(&self) -> TC0MCI1_FE_R {
        TC0MCI1_FE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter 0 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_re(&self) -> TC0MCI2_RE_R {
        TC0MCI2_RE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter 0 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_fe(&self) -> TC0MCI2_FE_R {
        TC0MCI2_FE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter 1 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_re(&self) -> TC1MCI0_RE_R {
        TC1MCI0_RE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter 1 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_fe(&self) -> TC1MCI0_FE_R {
        TC1MCI0_FE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Counter 1 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_re(&self) -> TC1MCI1_RE_R {
        TC1MCI1_RE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter 1 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_fe(&self) -> TC1MCI1_FE_R {
        TC1MCI1_FE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter 1 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_re(&self) -> TC1MCI2_RE_R {
        TC1MCI2_RE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter 1 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_fe(&self) -> TC1MCI2_FE_R {
        TC1MCI2_FE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter 2 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_re(&self) -> TC2MCI0_RE_R {
        TC2MCI0_RE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter 2 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_fe(&self) -> TC2MCI0_FE_R {
        TC2MCI0_FE_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Counter 2 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_re(&self) -> TC2MCI1_RE_R {
        TC2MCI1_RE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Counter 2 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_fe(&self) -> TC2MCI1_FE_R {
        TC2MCI1_FE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Counter 2 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_re(&self) -> TC2MCI2_RE_R {
        TC2MCI2_RE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Counter 2 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_fe(&self) -> TC2MCI2_FE_R {
        TC2MCI2_FE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 0 counter/timer mode."]
    #[inline(always)]
    pub fn cntr0(&self) -> CNTR0_R {
        CNTR0_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 1 counter/timer mode."]
    #[inline(always)]
    pub fn cntr1(&self) -> CNTR1_R {
        CNTR1_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 2 counter/timer mode."]
    #[inline(always)]
    pub fn cntr2(&self) -> CNTR2_R {
        CNTR2_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
