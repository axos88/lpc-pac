#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFG {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0x303a
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMR {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK,
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK,
}
impl crate::ToBits<u8> for FLASHTIMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMR::_1CLK => 0,
            FLASHTIMR::_2CLK => 1,
            FLASHTIMR::_3CLK => 2,
            FLASHTIMR::_4CLK => 3,
            FLASHTIMR::_5CLK => 4,
            FLASHTIMR::_6CLK => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLASHTIM_R = crate::FR<u8, FLASHTIMR>;
impl FLASHTIM_R {
    #[doc = "Checks if the value of the field is `_1CLK`"]
    #[inline(always)]
    pub fn is_1clk(&self) -> bool {
        *self == FLASHTIMR::_1CLK
    }
    #[doc = "Checks if the value of the field is `_2CLK`"]
    #[inline(always)]
    pub fn is_2clk(&self) -> bool {
        *self == FLASHTIMR::_2CLK
    }
    #[doc = "Checks if the value of the field is `_3CLK`"]
    #[inline(always)]
    pub fn is_3clk(&self) -> bool {
        *self == FLASHTIMR::_3CLK
    }
    #[doc = "Checks if the value of the field is `_4CLK`"]
    #[inline(always)]
    pub fn is_4clk(&self) -> bool {
        *self == FLASHTIMR::_4CLK
    }
    #[doc = "Checks if the value of the field is `_5CLK`"]
    #[inline(always)]
    pub fn is_5clk(&self) -> bool {
        *self == FLASHTIMR::_5CLK
    }
    #[doc = "Checks if the value of the field is `_6CLK`"]
    #[inline(always)]
    pub fn is_6clk(&self) -> bool {
        *self == FLASHTIMR::_6CLK
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMW {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK,
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::_1CLK => 0,
            FLASHTIMW::_2CLK => 1,
            FLASHTIMW::_3CLK => 2,
            FLASHTIMW::_4CLK => 3,
            FLASHTIMW::_5CLK => 4,
            FLASHTIMW::_6CLK => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASHTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHTIMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn _1clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_1CLK)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn _2clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_2CLK)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn _3clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_3CLK)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn _4clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_4CLK)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn _5clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_5CLK)
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn _6clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_6CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
}
