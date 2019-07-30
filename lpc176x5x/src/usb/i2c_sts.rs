#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::I2C_STS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIR {
    #[doc = "Transaction has not completed."]
    NOT_COMPLETE,
    #[doc = "Transaction completed."]
    COMPLETE,
}
impl crate::ToBits<bool> for TDIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TDIR::NOT_COMPLETE => false,
            TDIR::COMPLETE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TDI_R = crate::FR<bool, TDIR>;
impl TDI_R {
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TDIR::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TDIR::COMPLETE
    }
}
#[doc = "Possible values of the field `AFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIR {
    #[doc = "No arbitration failure on last transmission."]
    NO_ARBITRATION_FAILU,
    #[doc = "Arbitration failure occurred on last transmission."]
    ARBITRATION_FAILURE_,
}
impl crate::ToBits<bool> for AFIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AFIR::NO_ARBITRATION_FAILU => false,
            AFIR::ARBITRATION_FAILURE_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AFI_R = crate::FR<bool, AFIR>;
impl AFI_R {
    #[doc = "Checks if the value of the field is `NO_ARBITRATION_FAILU`"]
    #[inline(always)]
    pub fn is_no_arbitration_failu(&self) -> bool {
        *self == AFIR::NO_ARBITRATION_FAILU
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_FAILURE_`"]
    #[inline(always)]
    pub fn is_arbitration_failure_(&self) -> bool {
        *self == AFIR::ARBITRATION_FAILURE_
    }
}
#[doc = "Possible values of the field `NAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIR {
    #[doc = "Last transmission received an acknowledge."]
    ACKNOWLEDGE_RCVD,
    #[doc = "Last transmission did not receive an acknowledge."]
    NO_ACKNOWLEDGE_RCVD,
}
impl crate::ToBits<bool> for NAIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NAIR::ACKNOWLEDGE_RCVD => false,
            NAIR::NO_ACKNOWLEDGE_RCVD => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NAI_R = crate::FR<bool, NAIR>;
impl NAI_R {
    #[doc = "Checks if the value of the field is `ACKNOWLEDGE_RCVD`"]
    #[inline(always)]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == NAIR::ACKNOWLEDGE_RCVD
    }
    #[doc = "Checks if the value of the field is `NO_ACKNOWLEDGE_RCVD`"]
    #[inline(always)]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == NAIR::NO_ACKNOWLEDGE_RCVD
    }
}
#[doc = "Possible values of the field `DRMI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIR {
    #[doc = "Master transmitter does not need data."]
    BUSY,
    #[doc = "Master transmitter needs data."]
    NEED_DATA,
}
impl crate::ToBits<bool> for DRMIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DRMIR::BUSY => false,
            DRMIR::NEED_DATA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DRMI_R = crate::FR<bool, DRMIR>;
impl DRMI_R {
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRMIR::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRMIR::NEED_DATA
    }
}
#[doc = "Possible values of the field `DRSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIR {
    #[doc = "Slave transmitter does not need data."]
    BUSY,
    #[doc = "Slave transmitter needs data."]
    NEED_DATA,
}
impl crate::ToBits<bool> for DRSIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DRSIR::BUSY => false,
            DRSIR::NEED_DATA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DRSI_R = crate::FR<bool, DRSIR>;
impl DRSI_R {
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRSIR::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRSIR::NEED_DATA
    }
}
#[doc = r"Reader of the field"]
pub type ACTIVE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SCL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SDA_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFR {
    #[doc = "RX FIFO is not full"]
    RX_FIFO_IS_NOT_FULL,
    #[doc = "RX FIFO is full"]
    RX_FIFO_IS_FULL,
}
impl crate::ToBits<bool> for RFFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RFFR::RX_FIFO_IS_NOT_FULL => false,
            RFFR::RX_FIFO_IS_FULL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RFF_R = crate::FR<bool, RFFR>;
impl RFF_R {
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_NOT_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        *self == RFFR::RX_FIFO_IS_NOT_FULL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        *self == RFFR::RX_FIFO_IS_FULL
    }
}
#[doc = "Possible values of the field `RFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFER {
    #[doc = "RX FIFO contains data."]
    DATA,
    #[doc = "RX FIFO is empty"]
    EMPTY,
}
impl crate::ToBits<bool> for RFER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RFER::DATA => false,
            RFER::EMPTY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RFE_R = crate::FR<bool, RFER>;
impl RFE_R {
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RFER::DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFER::EMPTY
    }
}
#[doc = "Possible values of the field `TFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFR {
    #[doc = "TX FIFO is not full."]
    TX_FIFO_IS_NOT_FULL_,
    #[doc = "TX FIFO is full"]
    TX_FIFO_IS_FULL,
}
impl crate::ToBits<bool> for TFFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TFFR::TX_FIFO_IS_NOT_FULL_ => false,
            TFFR::TX_FIFO_IS_FULL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TFF_R = crate::FR<bool, TFFR>;
impl TFF_R {
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_NOT_FULL_`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        *self == TFFR::TX_FIFO_IS_NOT_FULL_
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        *self == TFFR::TX_FIFO_IS_FULL
    }
}
#[doc = "Possible values of the field `TFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFER {
    #[doc = "TX FIFO contains valid data."]
    VALID_DATA,
    #[doc = "TX FIFO is empty"]
    EMPTY,
}
impl crate::ToBits<bool> for TFER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TFER::VALID_DATA => false,
            TFER::EMPTY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TFE_R = crate::FR<bool, TFER>;
impl TFE_R {
    #[doc = "Checks if the value of the field is `VALID_DATA`"]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == TFER::VALID_DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFER::EMPTY
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AFI_R {
        AFI_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NAI_R {
        NAI_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DRMI_R {
        DRMI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DRSI_R {
        DRSI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
