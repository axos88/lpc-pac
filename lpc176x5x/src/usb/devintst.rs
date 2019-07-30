#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVINTST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRAME_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EP_FAST_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EP_SLOW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DEV_STAT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CCEMPTY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CDFULL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXENDPKT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXENDPKT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EP_RLZED_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ERR_INT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_fast(&self) -> EP_FAST_R {
        EP_FAST_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_slow(&self) -> EP_SLOW_R {
        EP_SLOW_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DEV_STAT_R {
        DEV_STAT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The command code register (USBCmdCode) is empty (New command can be written)."]
    #[inline(always)]
    pub fn ccempty(&self) -> CCEMPTY_R {
        CCEMPTY_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Command data register (USBCmdData) is full (Data can be read now)."]
    #[inline(always)]
    pub fn cdfull(&self) -> CDFULL_R {
        CDFULL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The current packet in the endpoint buffer is transferred to the CPU."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RXENDPKT_R {
        RXENDPKT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TXENDPKT_R {
        TXENDPKT_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
    #[inline(always)]
    pub fn ep_rlzed(&self) -> EP_RLZED_R {
        EP_RLZED_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
