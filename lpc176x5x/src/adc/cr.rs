#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub type SEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLKDIV_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST,
    #[doc = "Conversions are software controlled and require 31 clocks."]
    SW,
}
impl crate::ToBits<bool> for BURSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BURSTR::BURST => true,
            BURSTR::SW => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BURST_R = crate::FR<bool, BURSTR>;
impl BURST_R {
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BURSTR::BURST
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == BURSTR::SW
    }
}
#[doc = "Values that can be written to the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTW {
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST,
    #[doc = "Conversions are software controlled and require 31 clocks."]
    SW,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTW::BURST => true,
            BURSTW::SW => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(BURSTW::BURST)
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(BURSTW::SW)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNR {
    #[doc = "The A/D converter is operational."]
    POWERED,
    #[doc = "The A/D converter is in power-down mode."]
    POWERDOWN,
}
impl crate::ToBits<bool> for PDNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PDNR::POWERED => true,
            PDNR::POWERDOWN => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PDN_R = crate::FR<bool, PDNR>;
impl PDN_R {
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == PDNR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == PDNR::POWERDOWN
    }
}
#[doc = "Values that can be written to the field `PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNW {
    #[doc = "The A/D converter is operational."]
    POWERED,
    #[doc = "The A/D converter is in power-down mode."]
    POWERDOWN,
}
impl PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PDNW::POWERED => true,
            PDNW::POWERDOWN => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _PDNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(PDNW::POWERED)
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PDNW::POWERDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START_THIS_VALUE,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    P2_10,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    P1_27,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    MAT0_1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    MAT0_3,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    MAT1_0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    MAT1_1,
}
impl crate::ToBits<u8> for STARTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            STARTR::NO_START_THIS_VALUE => 0,
            STARTR::START_CONVERSION_NOW => 1,
            STARTR::P2_10 => 2,
            STARTR::P1_27 => 3,
            STARTR::MAT0_1 => 4,
            STARTR::MAT0_3 => 5,
            STARTR::MAT1_0 => 6,
            STARTR::MAT1_1 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type START_R = crate::FR<u8, STARTR>;
impl START_R {
    #[doc = "Checks if the value of the field is `NO_START_THIS_VALUE`"]
    #[inline(always)]
    pub fn is_no_start_this_value(&self) -> bool {
        *self == STARTR::NO_START_THIS_VALUE
    }
    #[doc = "Checks if the value of the field is `START_CONVERSION_NOW`"]
    #[inline(always)]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == STARTR::START_CONVERSION_NOW
    }
    #[doc = "Checks if the value of the field is `P2_10`"]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == STARTR::P2_10
    }
    #[doc = "Checks if the value of the field is `P1_27`"]
    #[inline(always)]
    pub fn is_p1_27(&self) -> bool {
        *self == STARTR::P1_27
    }
    #[doc = "Checks if the value of the field is `MAT0_1`"]
    #[inline(always)]
    pub fn is_mat0_1(&self) -> bool {
        *self == STARTR::MAT0_1
    }
    #[doc = "Checks if the value of the field is `MAT0_3`"]
    #[inline(always)]
    pub fn is_mat0_3(&self) -> bool {
        *self == STARTR::MAT0_3
    }
    #[doc = "Checks if the value of the field is `MAT1_0`"]
    #[inline(always)]
    pub fn is_mat1_0(&self) -> bool {
        *self == STARTR::MAT1_0
    }
    #[doc = "Checks if the value of the field is `MAT1_1`"]
    #[inline(always)]
    pub fn is_mat1_1(&self) -> bool {
        *self == STARTR::MAT1_1
    }
}
#[doc = "Values that can be written to the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTW {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START_THIS_VALUE,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    P2_10,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    P1_27,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    MAT0_1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    MAT0_3,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    MAT1_0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    MAT1_1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::NO_START_THIS_VALUE => 0,
            STARTW::START_CONVERSION_NOW => 1,
            STARTW::P2_10 => 2,
            STARTW::P1_27 => 3,
            STARTW::MAT0_1 => 4,
            STARTW::MAT0_3 => 5,
            STARTW::MAT1_0 => 6,
            STARTW::MAT1_1 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn no_start_this_value(self) -> &'a mut W {
        self.variant(STARTW::NO_START_THIS_VALUE)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start_conversion_now(self) -> &'a mut W {
        self.variant(STARTW::START_CONVERSION_NOW)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut W {
        self.variant(STARTW::P2_10)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    #[inline(always)]
    pub fn p1_27(self) -> &'a mut W {
        self.variant(STARTW::P1_27)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat0_1(self) -> &'a mut W {
        self.variant(STARTW::MAT0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn mat0_3(self) -> &'a mut W {
        self.variant(STARTW::MAT0_3)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_0(self) -> &'a mut W {
        self.variant(STARTW::MAT1_0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_1(self) -> &'a mut W {
        self.variant(STARTW::MAT1_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLLING,
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING,
}
impl crate::ToBits<bool> for EDGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EDGER::FALLLING => true,
            EDGER::RISING => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EDGE_R = crate::FR<bool, EDGER>;
impl EDGE_R {
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == EDGER::FALLLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGER::RISING
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGEW {
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLLING,
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::FALLLING => true,
            EDGEW::RISING => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut W {
        self.variant(EDGEW::FALLLING)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGEW::RISING)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&self) -> PDN_R {
        PDN_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits() >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&mut self) -> _PDNW {
        _PDNW { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
