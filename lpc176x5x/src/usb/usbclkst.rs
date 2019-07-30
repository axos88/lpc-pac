#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBCLKST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DEV_CLK_ON_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PORTSEL_CLK_ON_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AHB_CLK_ON_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Device clock on. The usbclk input to the device controller is active ."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port select register clock on."]
    #[inline(always)]
    pub fn portsel_clk_on(&self) -> PORTSEL_CLK_ON_R {
        PORTSEL_CLK_ON_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB clock on."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
