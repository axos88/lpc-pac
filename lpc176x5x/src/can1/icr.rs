#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for RIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RIR::RESET => false,
            RIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RI_R = crate::FR<bool, RIR>;
impl RI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RIR::SET
    }
}
#[doc = "Possible values of the field `TI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for TI1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TI1R::RESET => false,
            TI1R::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TI1_R = crate::FR<bool, TI1R>;
impl TI1_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI1R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI1R::SET
    }
}
#[doc = "Possible values of the field `EI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for EIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EIR::RESET => false,
            EIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EI_R = crate::FR<bool, EIR>;
impl EI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == EIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EIR::SET
    }
}
#[doc = "Possible values of the field `DOI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for DOIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DOIR::RESET => false,
            DOIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DOI_R = crate::FR<bool, DOIR>;
impl DOI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DOIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOIR::SET
    }
}
#[doc = "Possible values of the field `WUI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for WUIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WUIR::RESET => false,
            WUIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WUI_R = crate::FR<bool, WUIR>;
impl WUI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WUIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUIR::SET
    }
}
#[doc = "Possible values of the field `EPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for EPIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EPIR::RESET => false,
            EPIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EPI_R = crate::FR<bool, EPIR>;
impl EPI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == EPIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EPIR::SET
    }
}
#[doc = "Possible values of the field `ALI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for ALIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ALIR::RESET => false,
            ALIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ALI_R = crate::FR<bool, ALIR>;
impl ALI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ALIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ALIR::SET
    }
}
#[doc = "Possible values of the field `BEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for BEIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BEIR::RESET => false,
            BEIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BEI_R = crate::FR<bool, BEIR>;
impl BEI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BEIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BEIR::SET
    }
}
#[doc = "Possible values of the field `IDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for IDIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IDIR::RESET => false,
            IDIR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IDI_R = crate::FR<bool, IDIR>;
impl IDI_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IDIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDIR::SET
    }
}
#[doc = "Possible values of the field `TI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI2R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for TI2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TI2R::RESET => false,
            TI2R::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TI2_R = crate::FR<bool, TI2R>;
impl TI2_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI2R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI2R::SET
    }
}
#[doc = "Possible values of the field `TI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI3R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl crate::ToBits<bool> for TI3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TI3R::RESET => false,
            TI3R::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TI3_R = crate::FR<bool, TI3R>;
impl TI3_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI3R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI3R::SET
    }
}
#[doc = r"Reader of the field"]
pub type ERRBIT4_0_R = crate::FR<u8, u8>;
#[doc = "Possible values of the field `ERRDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRDIRR {
    #[doc = "Error occurred during transmitting."]
    ERROR_OCCURRED_DURING_TRANSMIT,
    #[doc = "Error occurred during receiving."]
    ERROR_OCCURRED_DURING_RECEIVE,
}
impl crate::ToBits<bool> for ERRDIRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ERRDIRR::ERROR_OCCURRED_DURING_TRANSMIT => false,
            ERRDIRR::ERROR_OCCURRED_DURING_RECEIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ERRDIR_R = crate::FR<bool, ERRDIRR>;
impl ERRDIR_R {
    #[doc = "Checks if the value of the field is `ERROR_OCCURRED_DURING_TRANSMIT`"]
    #[inline(always)]
    pub fn is_error_occurred_during_transmit(&self) -> bool {
        *self == ERRDIRR::ERROR_OCCURRED_DURING_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `ERROR_OCCURRED_DURING_RECEIVE`"]
    #[inline(always)]
    pub fn is_error_occurred_during_receive(&self) -> bool {
        *self == ERRDIRR::ERROR_OCCURRED_DURING_RECEIVE
    }
}
#[doc = "Possible values of the field `ERRC1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRC1_0R {
    #[doc = "Bit error"]
    BIT_ERROR,
    #[doc = "Form error"]
    FORM_ERROR,
    #[doc = "Stuff error"]
    STUFF_ERROR,
    #[doc = "Other error"]
    OTHER_ERROR,
}
impl crate::ToBits<u8> for ERRC1_0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ERRC1_0R::BIT_ERROR => 0,
            ERRC1_0R::FORM_ERROR => 1,
            ERRC1_0R::STUFF_ERROR => 2,
            ERRC1_0R::OTHER_ERROR => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ERRC1_0_R = crate::FR<u8, ERRC1_0R>;
impl ERRC1_0_R {
    #[doc = "Checks if the value of the field is `BIT_ERROR`"]
    #[inline(always)]
    pub fn is_bit_error(&self) -> bool {
        *self == ERRC1_0R::BIT_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == ERRC1_0R::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == ERRC1_0R::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `OTHER_ERROR`"]
    #[inline(always)]
    pub fn is_other_error(&self) -> bool {
        *self == ERRC1_0R::OTHER_ERROR
    }
}
#[doc = r"Reader of the field"]
pub type ALCBIT_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
    #[inline(always)]
    pub fn doi(&self) -> DOI_R {
        DOI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
    #[inline(always)]
    pub fn wui(&self) -> WUI_R {
        WUI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
    #[inline(always)]
    pub fn epi(&self) -> EPI_R {
        EPI_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
    #[inline(always)]
    pub fn ali(&self) -> ALI_R {
        ALI_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
    #[inline(always)]
    pub fn bei(&self) -> BEI_R {
        BEI_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
    #[inline(always)]
    pub fn idi(&self) -> IDI_R {
        IDI_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti2(&self) -> TI2_R {
        TI2_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti3(&self) -> TI3_R {
        TI3_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
    #[inline(always)]
    pub fn errbit4_0(&self) -> ERRBIT4_0_R {
        ERRBIT4_0_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
    #[inline(always)]
    pub fn errdir(&self) -> ERRDIR_R {
        ERRDIR_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - When the CAN controller detects a bus error, the type of error is captured in this field:"]
    #[inline(always)]
    pub fn errc1_0(&self) -> ERRC1_0_R {
        ERRC1_0_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
    #[inline(always)]
    pub fn alcbit(&self) -> ALCBIT_R {
        ALCBIT_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
