#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::NDDRINTST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type EPNDDINTST0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST12_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST13_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST14_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST15_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST16_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST17_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST18_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST19_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST20_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST21_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST22_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST23_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST24_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST25_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST26_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST27_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST28_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST29_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST30_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EPNDDINTST31_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst0(&self) -> EPNDDINTST0_R {
        EPNDDINTST0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst1(&self) -> EPNDDINTST1_R {
        EPNDDINTST1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst2(&self) -> EPNDDINTST2_R {
        EPNDDINTST2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst3(&self) -> EPNDDINTST3_R {
        EPNDDINTST3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst4(&self) -> EPNDDINTST4_R {
        EPNDDINTST4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst5(&self) -> EPNDDINTST5_R {
        EPNDDINTST5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst6(&self) -> EPNDDINTST6_R {
        EPNDDINTST6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst7(&self) -> EPNDDINTST7_R {
        EPNDDINTST7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst8(&self) -> EPNDDINTST8_R {
        EPNDDINTST8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst9(&self) -> EPNDDINTST9_R {
        EPNDDINTST9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst10(&self) -> EPNDDINTST10_R {
        EPNDDINTST10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst11(&self) -> EPNDDINTST11_R {
        EPNDDINTST11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst12(&self) -> EPNDDINTST12_R {
        EPNDDINTST12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst13(&self) -> EPNDDINTST13_R {
        EPNDDINTST13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst14(&self) -> EPNDDINTST14_R {
        EPNDDINTST14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst15(&self) -> EPNDDINTST15_R {
        EPNDDINTST15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst16(&self) -> EPNDDINTST16_R {
        EPNDDINTST16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst17(&self) -> EPNDDINTST17_R {
        EPNDDINTST17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst18(&self) -> EPNDDINTST18_R {
        EPNDDINTST18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst19(&self) -> EPNDDINTST19_R {
        EPNDDINTST19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst20(&self) -> EPNDDINTST20_R {
        EPNDDINTST20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst21(&self) -> EPNDDINTST21_R {
        EPNDDINTST21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst22(&self) -> EPNDDINTST22_R {
        EPNDDINTST22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst23(&self) -> EPNDDINTST23_R {
        EPNDDINTST23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst24(&self) -> EPNDDINTST24_R {
        EPNDDINTST24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst25(&self) -> EPNDDINTST25_R {
        EPNDDINTST25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst26(&self) -> EPNDDINTST26_R {
        EPNDDINTST26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst27(&self) -> EPNDDINTST27_R {
        EPNDDINTST27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst28(&self) -> EPNDDINTST28_R {
        EPNDDINTST28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst29(&self) -> EPNDDINTST29_R {
        EPNDDINTST29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst30(&self) -> EPNDDINTST30_R {
        EPNDDINTST30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = There is no new DD interrupt request for endpoint xx. 1 = There is a new DD interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn epnddintst31(&self) -> EPNDDINTST31_R {
        EPNDDINTST31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
