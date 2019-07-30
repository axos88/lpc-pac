#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRR {
    #[doc = "The UART1 receiver FIFO is empty."]
    EMPTY,
    #[doc = "The UART1 receiver FIFO is not empty."]
    NOTEMPTY,
}
impl crate::ToBits<bool> for RDRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RDRR::EMPTY => false,
            RDRR::NOTEMPTY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDR_R = crate::FR<bool, RDRR>;
impl RDR_R {
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RDRR::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == RDRR::NOTEMPTY
    }
}
#[doc = "Possible values of the field `OE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OER {
    #[doc = "Overrun error status is inactive."]
    INACTIVE,
    #[doc = "Overrun error status is active."]
    ACTIVE,
}
impl crate::ToBits<bool> for OER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OER::INACTIVE => false,
            OER::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OE_R = crate::FR<bool, OER>;
impl OE_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OER::ACTIVE
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Parity error status is inactive."]
    INACTIVE,
    #[doc = "Parity error status is active."]
    ACTIVE,
}
impl crate::ToBits<bool> for PER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PER::INACTIVE => false,
            PER::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PE_R = crate::FR<bool, PER>;
impl PE_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PER::ACTIVE
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "Framing error status is inactive."]
    INACTIVE,
    #[doc = "Framing error status is active."]
    ACTIVE,
}
impl crate::ToBits<bool> for FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FER::INACTIVE => false,
            FER::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FE_R = crate::FR<bool, FER>;
impl FE_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == FER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FER::ACTIVE
    }
}
#[doc = "Possible values of the field `BI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIR {
    #[doc = "Break interrupt status is inactive."]
    INACTIVE,
    #[doc = "Break interrupt status is active."]
    ACTIVE,
}
impl crate::ToBits<bool> for BIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BIR::INACTIVE => false,
            BIR::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BI_R = crate::FR<bool, BIR>;
impl BI_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BIR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BIR::ACTIVE
    }
}
#[doc = "Possible values of the field `THRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRER {
    #[doc = "THR contains valid data."]
    VALID,
    #[doc = "THR is empty."]
    THR_IS_EMPTY_,
}
impl crate::ToBits<bool> for THRER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            THRER::VALID => false,
            THRER::THR_IS_EMPTY_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THRE_R = crate::FR<bool, THRER>;
impl THRE_R {
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == THRER::VALID
    }
    #[doc = "Checks if the value of the field is `THR_IS_EMPTY_`"]
    #[inline(always)]
    pub fn is_thr_is_empty_(&self) -> bool {
        *self == THRER::THR_IS_EMPTY_
    }
}
#[doc = "Possible values of the field `TEMT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMTR {
    #[doc = "THR and/or the TSR contains valid data."]
    VALID,
    #[doc = "THR and the TSR are empty."]
    EMPTY,
}
impl crate::ToBits<bool> for TEMTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TEMTR::VALID => false,
            TEMTR::EMPTY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TEMT_R = crate::FR<bool, TEMTR>;
impl TEMT_R {
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == TEMTR::VALID
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TEMTR::EMPTY
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    NOERROR,
    #[doc = "UART1 RBR contains at least one UART1 RX error."]
    ERRORS,
}
impl crate::ToBits<bool> for RXFER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RXFER::NOERROR => false,
            RXFER::ERRORS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXFE_R = crate::FR<bool, RXFER>;
impl RXFE_R {
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == RXFER::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERRORS`"]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == RXFER::ERRORS
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
