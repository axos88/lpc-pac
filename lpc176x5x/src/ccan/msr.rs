#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type E1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type E2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BS2_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
    #[inline(always)]
    pub fn e1(&self) -> E1_R {
        E1_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
    #[inline(always)]
    pub fn e2(&self) -> E2_R {
        E2_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
