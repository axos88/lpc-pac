#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TXSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TS2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TBS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TBS2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TCS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TCS2_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
    #[inline(always)]
    pub fn tbs1(&self) -> TBS1_R {
        TBS1_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
    #[inline(always)]
    pub fn tbs2(&self) -> TBS2_R {
        TBS2_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
    #[inline(always)]
    pub fn tcs1(&self) -> TCS1_R {
        TCS1_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
    #[inline(always)]
    pub fn tcs2(&self) -> TCS2_R {
        TCS2_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
