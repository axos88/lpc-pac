#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFMR {
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
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type ACCOFF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ACCOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCOFFW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ACCBP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ACCBPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCBPW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `EFCAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFCANR {
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A,
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE,
}
impl crate::ToBits<bool> for EFCANR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EFCANR::SOFTWARE_MUST_READ_A => false,
            EFCANR::THE_ACCEPTANCE_FILTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EFCAN_R = crate::FR<bool, EFCANR>;
impl EFCAN_R {
    #[doc = "Checks if the value of the field is `SOFTWARE_MUST_READ_A`"]
    #[inline(always)]
    pub fn is_software_must_read_a(&self) -> bool {
        *self == EFCANR::SOFTWARE_MUST_READ_A
    }
    #[doc = "Checks if the value of the field is `THE_ACCEPTANCE_FILTE`"]
    #[inline(always)]
    pub fn is_the_acceptance_filte(&self) -> bool {
        *self == EFCANR::THE_ACCEPTANCE_FILTE
    }
}
#[doc = "Values that can be written to the field `EFCAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFCANW {
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    SOFTWARE_MUST_READ_A,
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    THE_ACCEPTANCE_FILTE,
}
impl EFCANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EFCANW::SOFTWARE_MUST_READ_A => false,
            EFCANW::THE_ACCEPTANCE_FILTE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EFCANW<'a> {
    w: &'a mut W,
}
impl<'a> _EFCANW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFCANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn software_must_read_a(self) -> &'a mut W {
        self.variant(EFCANW::SOFTWARE_MUST_READ_A)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn the_acceptance_filte(self) -> &'a mut W {
        self.variant(EFCANW::THE_ACCEPTANCE_FILTE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&self) -> ACCOFF_R {
        ACCOFF_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&self) -> ACCBP_R {
        ACCBP_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&self) -> EFCAN_R {
        EFCAN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&mut self) -> _ACCOFFW {
        _ACCOFFW { w: self }
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&mut self) -> _ACCBPW {
        _ACCBPW { w: self }
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&mut self) -> _EFCANW {
        _EFCANW { w: self }
    }
}
