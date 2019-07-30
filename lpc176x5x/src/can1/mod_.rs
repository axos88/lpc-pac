#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD {
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
#[doc = "Possible values of the field `RM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMR {
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL_THE_CAN_CONTR,
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET_CAN_OPERATION,
}
impl crate::ToBits<bool> for RMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RMR::NORMAL_THE_CAN_CONTR => false,
            RMR::RESET_CAN_OPERATION => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RM_R = crate::FR<bool, RMR>;
impl RM_R {
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONTR`"]
    #[inline(always)]
    pub fn is_normal_the_can_contr(&self) -> bool {
        *self == RMR::NORMAL_THE_CAN_CONTR
    }
    #[doc = "Checks if the value of the field is `RESET_CAN_OPERATION`"]
    #[inline(always)]
    pub fn is_reset_can_operation(&self) -> bool {
        *self == RMR::RESET_CAN_OPERATION
    }
}
#[doc = "Values that can be written to the field `RM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMW {
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL_THE_CAN_CONTR,
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET_CAN_OPERATION,
}
impl RMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RMW::NORMAL_THE_CAN_CONTR => false,
            RMW::RESET_CAN_OPERATION => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RMW<'a> {
    w: &'a mut W,
}
impl<'a> _RMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn normal_the_can_contr(self) -> &'a mut W {
        self.variant(RMW::NORMAL_THE_CAN_CONTR)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn reset_can_operation(self) -> &'a mut W {
        self.variant(RMW::RESET_CAN_OPERATION)
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
#[doc = "Possible values of the field `LOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOMR {
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL_THE_CAN_CONT,
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY_THE_CON,
}
impl crate::ToBits<bool> for LOMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LOMR::NORMAL_THE_CAN_CONT => false,
            LOMR::LISTEN_ONLY_THE_CON => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOM_R = crate::FR<bool, LOMR>;
impl LOM_R {
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONT`"]
    #[inline(always)]
    pub fn is_normal_the_can_cont(&self) -> bool {
        *self == LOMR::NORMAL_THE_CAN_CONT
    }
    #[doc = "Checks if the value of the field is `LISTEN_ONLY_THE_CON`"]
    #[inline(always)]
    pub fn is_listen_only_the_con(&self) -> bool {
        *self == LOMR::LISTEN_ONLY_THE_CON
    }
}
#[doc = "Values that can be written to the field `LOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOMW {
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL_THE_CAN_CONT,
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY_THE_CON,
}
impl LOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LOMW::NORMAL_THE_CAN_CONT => false,
            LOMW::LISTEN_ONLY_THE_CON => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LOMW<'a> {
    w: &'a mut W,
}
impl<'a> _LOMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn normal_the_can_cont(self) -> &'a mut W {
        self.variant(LOMW::NORMAL_THE_CAN_CONT)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn listen_only_the_con(self) -> &'a mut W {
        self.variant(LOMW::LISTEN_ONLY_THE_CON)
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
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL_A_TRANSMITTE,
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST_THE_CONTR,
}
impl crate::ToBits<bool> for STMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STMR::NORMAL_A_TRANSMITTE => false,
            STMR::SELF_TEST_THE_CONTR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STM_R = crate::FR<bool, STMR>;
impl STM_R {
    #[doc = "Checks if the value of the field is `NORMAL_A_TRANSMITTE`"]
    #[inline(always)]
    pub fn is_normal_a_transmitte(&self) -> bool {
        *self == STMR::NORMAL_A_TRANSMITTE
    }
    #[doc = "Checks if the value of the field is `SELF_TEST_THE_CONTR`"]
    #[inline(always)]
    pub fn is_self_test_the_contr(&self) -> bool {
        *self == STMR::SELF_TEST_THE_CONTR
    }
}
#[doc = "Values that can be written to the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMW {
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL_A_TRANSMITTE,
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST_THE_CONTR,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            STMW::NORMAL_A_TRANSMITTE => false,
            STMW::SELF_TEST_THE_CONTR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn normal_a_transmitte(self) -> &'a mut W {
        self.variant(STMW::NORMAL_A_TRANSMITTE)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn self_test_the_contr(self) -> &'a mut W {
        self.variant(STMW::SELF_TEST_THE_CONTR)
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
#[doc = "Possible values of the field `TPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMR {
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID_THE_TRANSMIT,
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY_THE_,
}
impl crate::ToBits<bool> for TPMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TPMR::CAN_ID_THE_TRANSMIT => false,
            TPMR::LOCAL_PRIORITY_THE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TPM_R = crate::FR<bool, TPMR>;
impl TPM_R {
    #[doc = "Checks if the value of the field is `CAN_ID_THE_TRANSMIT`"]
    #[inline(always)]
    pub fn is_can_id_the_transmit(&self) -> bool {
        *self == TPMR::CAN_ID_THE_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `LOCAL_PRIORITY_THE_`"]
    #[inline(always)]
    pub fn is_local_priority_the_(&self) -> bool {
        *self == TPMR::LOCAL_PRIORITY_THE_
    }
}
#[doc = "Values that can be written to the field `TPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMW {
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID_THE_TRANSMIT,
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY_THE_,
}
impl TPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TPMW::CAN_ID_THE_TRANSMIT => false,
            TPMW::LOCAL_PRIORITY_THE_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TPMW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn can_id_the_transmit(self) -> &'a mut W {
        self.variant(TPMW::CAN_ID_THE_TRANSMIT)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn local_priority_the_(self) -> &'a mut W {
        self.variant(TPMW::LOCAL_PRIORITY_THE_)
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
#[doc = "Possible values of the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMR {
    #[doc = "Wake-up. Normal operation."]
    WAKE_UP_NORMAL_OPER,
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP_THE_CAN_CONTR,
}
impl crate::ToBits<bool> for SMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMR::WAKE_UP_NORMAL_OPER => false,
            SMR::SLEEP_THE_CAN_CONTR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SM_R = crate::FR<bool, SMR>;
impl SM_R {
    #[doc = "Checks if the value of the field is `WAKE_UP_NORMAL_OPER`"]
    #[inline(always)]
    pub fn is_wake_up_normal_oper(&self) -> bool {
        *self == SMR::WAKE_UP_NORMAL_OPER
    }
    #[doc = "Checks if the value of the field is `SLEEP_THE_CAN_CONTR`"]
    #[inline(always)]
    pub fn is_sleep_the_can_contr(&self) -> bool {
        *self == SMR::SLEEP_THE_CAN_CONTR
    }
}
#[doc = "Values that can be written to the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMW {
    #[doc = "Wake-up. Normal operation."]
    WAKE_UP_NORMAL_OPER,
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP_THE_CAN_CONTR,
}
impl SMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SMW::WAKE_UP_NORMAL_OPER => false,
            SMW::SLEEP_THE_CAN_CONTR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMW<'a> {
    w: &'a mut W,
}
impl<'a> _SMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn wake_up_normal_oper(self) -> &'a mut W {
        self.variant(SMW::WAKE_UP_NORMAL_OPER)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn sleep_the_can_contr(self) -> &'a mut W {
        self.variant(SMW::SLEEP_THE_CAN_CONTR)
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
#[doc = "Possible values of the field `RPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPMR {
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE_RD_INPUT,
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE_RD_INPU,
}
impl crate::ToBits<bool> for RPMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RPMR::LOW_ACTIVE_RD_INPUT => false,
            RPMR::HIGH_ACTIVE_RD_INPU => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RPM_R = crate::FR<bool, RPMR>;
impl RPM_R {
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_RD_INPUT`"]
    #[inline(always)]
    pub fn is_low_active_rd_input(&self) -> bool {
        *self == RPMR::LOW_ACTIVE_RD_INPUT
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_RD_INPU`"]
    #[inline(always)]
    pub fn is_high_active_rd_inpu(&self) -> bool {
        *self == RPMR::HIGH_ACTIVE_RD_INPU
    }
}
#[doc = "Values that can be written to the field `RPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPMW {
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE_RD_INPUT,
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE_RD_INPU,
}
impl RPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RPMW::LOW_ACTIVE_RD_INPUT => false,
            RPMW::HIGH_ACTIVE_RD_INPU => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RPMW<'a> {
    w: &'a mut W,
}
impl<'a> _RPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn low_active_rd_input(self) -> &'a mut W {
        self.variant(RPMW::LOW_ACTIVE_RD_INPUT)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn high_active_rd_inpu(self) -> &'a mut W {
        self.variant(RPMW::HIGH_ACTIVE_RD_INPU)
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
#[doc = "Possible values of the field `TM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR {
    #[doc = "Disabled. Normal operation."]
    DISABLED_NORMAL_OPE,
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED_THE_TD_PIN_,
}
impl crate::ToBits<bool> for TMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TMR::DISABLED_NORMAL_OPE => false,
            TMR::ENABLED_THE_TD_PIN_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TM_R = crate::FR<bool, TMR>;
impl TM_R {
    #[doc = "Checks if the value of the field is `DISABLED_NORMAL_OPE`"]
    #[inline(always)]
    pub fn is_disabled_normal_ope(&self) -> bool {
        *self == TMR::DISABLED_NORMAL_OPE
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_TD_PIN_`"]
    #[inline(always)]
    pub fn is_enabled_the_td_pin_(&self) -> bool {
        *self == TMR::ENABLED_THE_TD_PIN_
    }
}
#[doc = "Values that can be written to the field `TM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMW {
    #[doc = "Disabled. Normal operation."]
    DISABLED_NORMAL_OPE,
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED_THE_TD_PIN_,
}
impl TMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TMW::DISABLED_NORMAL_OPE => false,
            TMW::ENABLED_THE_TD_PIN_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TMW<'a> {
    w: &'a mut W,
}
impl<'a> _TMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn disabled_normal_ope(self) -> &'a mut W {
        self.variant(TMW::DISABLED_NORMAL_OPE)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn enabled_the_td_pin_(self) -> &'a mut W {
        self.variant(TMW::ENABLED_THE_TD_PIN_)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&self) -> RPM_R {
        RPM_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&mut self) -> _RMW {
        _RMW { w: self }
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&mut self) -> _LOMW {
        _LOMW { w: self }
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&mut self) -> _TPMW {
        _TPMW { w: self }
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&mut self) -> _SMW {
        _SMW { w: self }
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&mut self) -> _RPMW {
        _RPMW { w: self }
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&mut self) -> _TMW {
        _TMW { w: self }
    }
}
