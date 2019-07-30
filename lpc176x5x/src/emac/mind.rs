#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MIND {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type BUSY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SCANNING_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NOTVALID_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MIILINKFAIL_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
    #[inline(always)]
    pub fn scanning(&self) -> SCANNING_R {
        SCANNING_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
    #[inline(always)]
    pub fn notvalid(&self) -> NOTVALID_R {
        NOTVALID_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
    #[inline(always)]
    pub fn miilinkfail(&self) -> MIILINKFAIL_R {
        MIILINKFAIL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
