#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CON {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RUN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN0R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl crate::ToBits<bool> for RUN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RUN0R::STOP_ => false,
            RUN0R::RUN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RUN0_R = crate::FR<bool, RUN0R>;
impl RUN0_R {
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == RUN0R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == RUN0R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER0R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl crate::ToBits<bool> for CENTER0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CENTER0R::EDGE_ALIGNED_ => false,
            CENTER0R::CENTER_ALIGNED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CENTER0_R = crate::FR<bool, CENTER0R>;
impl CENTER0_R {
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER0R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER0R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA0R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl crate::ToBits<bool> for POLA0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            POLA0R::PASSIVE_STATE_IS_LOW => false,
            POLA0R::PASSIVE_STATE_IS_HIG => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POLA0_R = crate::FR<bool, POLA0R>;
impl POLA0_R {
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA0R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA0R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE0R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl crate::ToBits<bool> for DTE0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DTE0R::DEAD_TIME_DISABLED_ => false,
            DTE0R::DEAD_TIME_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DTE0_R = crate::FR<bool, DTE0R>;
impl DTE0_R {
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE0R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE0R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP0R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl crate::ToBits<bool> for DISUP0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DISUP0R::UPDATE => false,
            DISUP0R::NOUPDATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DISUP0_R = crate::FR<bool, DISUP0R>;
impl DISUP0_R {
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP0R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP0R::NOUPDATE
    }
}
#[doc = "Possible values of the field `RUN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN1R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl crate::ToBits<bool> for RUN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RUN1R::STOP_ => false,
            RUN1R::RUN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RUN1_R = crate::FR<bool, RUN1R>;
impl RUN1_R {
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == RUN1R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == RUN1R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER1R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl crate::ToBits<bool> for CENTER1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CENTER1R::EDGE_ALIGNED_ => false,
            CENTER1R::CENTER_ALIGNED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CENTER1_R = crate::FR<bool, CENTER1R>;
impl CENTER1_R {
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER1R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER1R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA1R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl crate::ToBits<bool> for POLA1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            POLA1R::PASSIVE_STATE_IS_LOW => false,
            POLA1R::PASSIVE_STATE_IS_HIG => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POLA1_R = crate::FR<bool, POLA1R>;
impl POLA1_R {
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA1R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA1R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE1R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl crate::ToBits<bool> for DTE1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DTE1R::DEAD_TIME_DISABLED_ => false,
            DTE1R::DEAD_TIME_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DTE1_R = crate::FR<bool, DTE1R>;
impl DTE1_R {
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE1R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE1R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP1R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl crate::ToBits<bool> for DISUP1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DISUP1R::UPDATE => false,
            DISUP1R::NOUPDATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DISUP1_R = crate::FR<bool, DISUP1R>;
impl DISUP1_R {
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP1R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP1R::NOUPDATE
    }
}
#[doc = "Possible values of the field `RUN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN2R {
    #[doc = "Stop."]
    STOP_,
    #[doc = "Run."]
    RUN_,
}
impl crate::ToBits<bool> for RUN2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RUN2R::STOP_ => false,
            RUN2R::RUN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RUN2_R = crate::FR<bool, RUN2R>;
impl RUN2_R {
    #[doc = "Checks if the value of the field is `STOP_`"]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == RUN2R::STOP_
    }
    #[doc = "Checks if the value of the field is `RUN_`"]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == RUN2R::RUN_
    }
}
#[doc = "Possible values of the field `CENTER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER2R {
    #[doc = "Edge-aligned."]
    EDGE_ALIGNED_,
    #[doc = "Center-aligned."]
    CENTER_ALIGNED_,
}
impl crate::ToBits<bool> for CENTER2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CENTER2R::EDGE_ALIGNED_ => false,
            CENTER2R::CENTER_ALIGNED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CENTER2_R = crate::FR<bool, CENTER2R>;
impl CENTER2_R {
    #[doc = "Checks if the value of the field is `EDGE_ALIGNED_`"]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == CENTER2R::EDGE_ALIGNED_
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED_`"]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == CENTER2R::CENTER_ALIGNED_
    }
}
#[doc = "Possible values of the field `POLA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA2R {
    #[doc = "Passive state is LOW, active state is HIGH."]
    PASSIVE_STATE_IS_LOW,
    #[doc = "Passive state is HIGH, active state is LOW."]
    PASSIVE_STATE_IS_HIG,
}
impl crate::ToBits<bool> for POLA2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            POLA2R::PASSIVE_STATE_IS_LOW => false,
            POLA2R::PASSIVE_STATE_IS_HIG => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POLA2_R = crate::FR<bool, POLA2R>;
impl POLA2_R {
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_LOW`"]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == POLA2R::PASSIVE_STATE_IS_LOW
    }
    #[doc = "Checks if the value of the field is `PASSIVE_STATE_IS_HIG`"]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == POLA2R::PASSIVE_STATE_IS_HIG
    }
}
#[doc = "Possible values of the field `DTE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE2R {
    #[doc = "Dead-time disabled."]
    DEAD_TIME_DISABLED_,
    #[doc = "Dead-time enabled."]
    DEAD_TIME_ENABLED_,
}
impl crate::ToBits<bool> for DTE2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DTE2R::DEAD_TIME_DISABLED_ => false,
            DTE2R::DEAD_TIME_ENABLED_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DTE2_R = crate::FR<bool, DTE2R>;
impl DTE2_R {
    #[doc = "Checks if the value of the field is `DEAD_TIME_DISABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == DTE2R::DEAD_TIME_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DEAD_TIME_ENABLED_`"]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == DTE2R::DEAD_TIME_ENABLED_
    }
}
#[doc = "Possible values of the field `DISUP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISUP2R {
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    UPDATE,
    #[doc = "Functional registers remain the same as long as the timer is running."]
    NOUPDATE,
}
impl crate::ToBits<bool> for DISUP2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DISUP2R::UPDATE => false,
            DISUP2R::NOUPDATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DISUP2_R = crate::FR<bool, DISUP2R>;
impl DISUP2_R {
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == DISUP2R::UPDATE
    }
    #[doc = "Checks if the value of the field is `NOUPDATE`"]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == DISUP2R::NOUPDATE
    }
}
#[doc = "Possible values of the field `INVBDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVBDCR {
    #[doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    OPPOSITE,
    #[doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    SAME,
}
impl crate::ToBits<bool> for INVBDCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVBDCR::OPPOSITE => false,
            INVBDCR::SAME => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INVBDC_R = crate::FR<bool, INVBDCR>;
impl INVBDC_R {
    #[doc = "Checks if the value of the field is `OPPOSITE`"]
    #[inline(always)]
    pub fn is_opposite(&self) -> bool {
        *self == INVBDCR::OPPOSITE
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == INVBDCR::SAME
    }
}
#[doc = "Possible values of the field `ACMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMODER {
    #[doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    _3_PHASE_AC_MODE_OFF,
    #[doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    _3_PHASE_AC_MODE_ON_,
}
impl crate::ToBits<bool> for ACMODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACMODER::_3_PHASE_AC_MODE_OFF => false,
            ACMODER::_3_PHASE_AC_MODE_ON_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACMODE_R = crate::FR<bool, ACMODER>;
impl ACMODE_R {
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_OFF`"]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_off(&self) -> bool {
        *self == ACMODER::_3_PHASE_AC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_AC_MODE_ON_`"]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_on_(&self) -> bool {
        *self == ACMODER::_3_PHASE_AC_MODE_ON_
    }
}
#[doc = "Possible values of the field `DCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMODER {
    #[doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    _3_PHASE_DC_MODE_OFF,
    #[doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    _3_PHASE_DC_MODE_ON_,
}
impl crate::ToBits<bool> for DCMODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DCMODER::_3_PHASE_DC_MODE_OFF => false,
            DCMODER::_3_PHASE_DC_MODE_ON_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCMODE_R = crate::FR<bool, DCMODER>;
impl DCMODE_R {
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_OFF`"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_off(&self) -> bool {
        *self == DCMODER::_3_PHASE_DC_MODE_OFF
    }
    #[doc = "Checks if the value of the field is `_3_PHASE_DC_MODE_ON_`"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_on_(&self) -> bool {
        *self == DCMODER::_3_PHASE_DC_MODE_ON_
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline(always)]
    pub fn run0(&self) -> RUN0_R {
        RUN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline(always)]
    pub fn center0(&self) -> CENTER0_R {
        CENTER0_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline(always)]
    pub fn pola0(&self) -> POLA0_R {
        POLA0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline(always)]
    pub fn dte0(&self) -> DTE0_R {
        DTE0_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup0(&self) -> DISUP0_R {
        DISUP0_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline(always)]
    pub fn run1(&self) -> RUN1_R {
        RUN1_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline(always)]
    pub fn center1(&self) -> CENTER1_R {
        CENTER1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline(always)]
    pub fn pola1(&self) -> POLA1_R {
        POLA1_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte1(&self) -> DTE1_R {
        DTE1_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup1(&self) -> DISUP1_R {
        DISUP1_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline(always)]
    pub fn run2(&self) -> RUN2_R {
        RUN2_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline(always)]
    pub fn center2(&self) -> CENTER2_R {
        CENTER2_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline(always)]
    pub fn pola2(&self) -> POLA2_R {
        POLA2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte2(&self) -> DTE2_R {
        DTE2_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup2(&self) -> DISUP2_R {
        DISUP2_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline(always)]
    pub fn invbdc(&self) -> INVBDC_R {
        INVBDC_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline(always)]
    pub fn acmode(&self) -> ACMODE_R {
        ACMODE_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline(always)]
    pub fn dcmode(&self) -> DCMODE_R {
        DCMODE_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
