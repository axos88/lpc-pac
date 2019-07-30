#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYSERRINTST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type EPERRINTST0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST12_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST13_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST14_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST15_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST16_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST17_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST18_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST19_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST20_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST21_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST22_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST23_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST24_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST25_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST26_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST27_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST28_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST29_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST30_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPERRINTST31_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst0(&self) -> EPERRINTST0_R {
        EPERRINTST0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst1(&self) -> EPERRINTST1_R {
        EPERRINTST1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst2(&self) -> EPERRINTST2_R {
        EPERRINTST2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst3(&self) -> EPERRINTST3_R {
        EPERRINTST3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst4(&self) -> EPERRINTST4_R {
        EPERRINTST4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst5(&self) -> EPERRINTST5_R {
        EPERRINTST5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst6(&self) -> EPERRINTST6_R {
        EPERRINTST6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst7(&self) -> EPERRINTST7_R {
        EPERRINTST7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst8(&self) -> EPERRINTST8_R {
        EPERRINTST8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst9(&self) -> EPERRINTST9_R {
        EPERRINTST9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst10(&self) -> EPERRINTST10_R {
        EPERRINTST10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst11(&self) -> EPERRINTST11_R {
        EPERRINTST11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst12(&self) -> EPERRINTST12_R {
        EPERRINTST12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst13(&self) -> EPERRINTST13_R {
        EPERRINTST13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst14(&self) -> EPERRINTST14_R {
        EPERRINTST14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst15(&self) -> EPERRINTST15_R {
        EPERRINTST15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst16(&self) -> EPERRINTST16_R {
        EPERRINTST16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst17(&self) -> EPERRINTST17_R {
        EPERRINTST17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst18(&self) -> EPERRINTST18_R {
        EPERRINTST18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst19(&self) -> EPERRINTST19_R {
        EPERRINTST19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst20(&self) -> EPERRINTST20_R {
        EPERRINTST20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst21(&self) -> EPERRINTST21_R {
        EPERRINTST21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst22(&self) -> EPERRINTST22_R {
        EPERRINTST22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst23(&self) -> EPERRINTST23_R {
        EPERRINTST23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst24(&self) -> EPERRINTST24_R {
        EPERRINTST24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst25(&self) -> EPERRINTST25_R {
        EPERRINTST25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst26(&self) -> EPERRINTST26_R {
        EPERRINTST26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst27(&self) -> EPERRINTST27_R {
        EPERRINTST27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst28(&self) -> EPERRINTST28_R {
        EPERRINTST28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst29(&self) -> EPERRINTST29_R {
        EPERRINTST29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst30(&self) -> EPERRINTST30_R {
        EPERRINTST30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = There is no System Error Interrupt request for endpoint xx. 1 = There is a System Error Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eperrintst31(&self) -> EPERRINTST31_R {
        EPERRINTST31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
