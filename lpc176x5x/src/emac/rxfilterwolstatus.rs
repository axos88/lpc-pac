#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXFILTERWOLSTATUS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ABW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AMW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AUHW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AMHW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type APW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RFW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MPW_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
    #[inline(always)]
    pub fn auw(&self) -> AUW_R {
        AUW_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
    #[inline(always)]
    pub fn abw(&self) -> ABW_R {
        ABW_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
    #[inline(always)]
    pub fn amw(&self) -> AMW_R {
        AMW_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn auhw(&self) -> AUHW_R {
        AUHW_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn amhw(&self) -> AMHW_R {
        AMHW_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
    #[inline(always)]
    pub fn apw(&self) -> APW_R {
        APW_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
    #[inline(always)]
    pub fn mpw(&self) -> MPW_R {
        MPW_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
