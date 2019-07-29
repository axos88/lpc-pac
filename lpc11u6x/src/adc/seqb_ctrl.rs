#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEQB_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CHANNELSR {
    bits: u16,
}
impl CHANNELSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIGGERR {
    bits: u8,
}
impl TRIGGERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TRIGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOLR {
    #[doc = "Negative edge. A negative edge launches the conversion sequence on the selected trigger input."]
    NEGATIVE_EDGE,
    #[doc = "Positive edge. A positive edge launches the conversion sequence on the selected trigger input."]
    POSITIVE_EDGE,
}
impl TRIGPOLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRIGPOLR::NEGATIVE_EDGE => false,
            TRIGPOLR::POSITIVE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGPOLR {
        match value {
            false => TRIGPOLR::NEGATIVE_EDGE,
            true => TRIGPOLR::POSITIVE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_EDGE`"]
    #[inline]
    pub fn is_negative_edge(&self) -> bool {
        *self == TRIGPOLR::NEGATIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_EDGE`"]
    #[inline]
    pub fn is_positive_edge(&self) -> bool {
        *self == TRIGPOLR::POSITIVE_EDGE
    }
}
#[doc = "Possible values of the field `SYNCBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCBYPASSR {
    #[doc = "Enable synchronization. The hardware trigger bypass is not enabled."]
    ENABLE_SYNCHRONIZATI,
    #[doc = "Bypass synchronization. The hardware trigger bypass is enabled."]
    BYPASS_SYNCHRONIZATI,
}
impl SYNCBYPASSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYNCBYPASSR::ENABLE_SYNCHRONIZATI => false,
            SYNCBYPASSR::BYPASS_SYNCHRONIZATI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCBYPASSR {
        match value {
            false => SYNCBYPASSR::ENABLE_SYNCHRONIZATI,
            true => SYNCBYPASSR::BYPASS_SYNCHRONIZATI,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_SYNCHRONIZATI`"]
    #[inline]
    pub fn is_enable_synchronizati(&self) -> bool {
        *self == SYNCBYPASSR::ENABLE_SYNCHRONIZATI
    }
    #[doc = "Checks if the value of the field is `BYPASS_SYNCHRONIZATI`"]
    #[inline]
    pub fn is_bypass_synchronizati(&self) -> bool {
        *self == SYNCBYPASSR::BYPASS_SYNCHRONIZATI
    }
}
#[doc = r" Value of the field"]
pub struct STARTR {
    bits: bool,
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BURSTR {
    bits: bool,
}
impl BURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SINGLESTEPR {
    bits: bool,
}
impl SINGLESTEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "End of conversion. The sequence B interrupt/DMA flag will be set at the end of each individual A/D conversion performed under sequence B. This flag will mirror the DATAVALID bit in the SEQB_GDAT register.  The OVERRUN bit in the SEQB_GDAT register will contribute to generation of an overrun interrupt/DMA request if enabled."]
    END_OF_CONVERSION,
    #[doc = "End of sequence. The sequence B interrupt/DMA flag will be set when the entire set of sequence B conversions completes. This flag will need to be explicitly cleared by software or by the DMA-clear signal in this mode.   The OVERRUN bit in the SEQB_GDAT register will NOT contribute to generation of an overrun interrupt/DMA request since it is assumed this register will not be utilized in this mode."]
    END_OF_SEQUENCE,
}
impl MODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MODER::END_OF_CONVERSION => false,
            MODER::END_OF_SEQUENCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::END_OF_CONVERSION,
            true => MODER::END_OF_SEQUENCE,
        }
    }
    #[doc = "Checks if the value of the field is `END_OF_CONVERSION`"]
    #[inline]
    pub fn is_end_of_conversion(&self) -> bool {
        *self == MODER::END_OF_CONVERSION
    }
    #[doc = "Checks if the value of the field is `END_OF_SEQUENCE`"]
    #[inline]
    pub fn is_end_of_sequence(&self) -> bool {
        *self == MODER::END_OF_SEQUENCE
    }
}
#[doc = "Possible values of the field `SEQB_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_ENAR {
    #[doc = "Disabled. Sequence B is disabled. Sequence B triggers are ignored. If this bit is cleared while sequence B is in progress, the sequence will be halted at the end of the current conversion. After the sequence is re-enabled, a new trigger will be required to restart the sequence beginning with the next enabled channel."]
    DISABLED,
    #[doc = "Enabled. Sequence B is enabled."]
    ENABLED,
}
impl SEQB_ENAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SEQB_ENAR::DISABLED => false,
            SEQB_ENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQB_ENAR {
        match value {
            false => SEQB_ENAR::DISABLED,
            true => SEQB_ENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_ENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_ENAR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _CHANNELSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNELSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGPOL`"]
pub enum TRIGPOLW {
    #[doc = "Negative edge. A negative edge launches the conversion sequence on the selected trigger input."]
    NEGATIVE_EDGE,
    #[doc = "Positive edge. A positive edge launches the conversion sequence on the selected trigger input."]
    POSITIVE_EDGE,
}
impl TRIGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGPOLW::NEGATIVE_EDGE => false,
            TRIGPOLW::POSITIVE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negative edge. A negative edge launches the conversion sequence on the selected trigger input."]
    #[inline]
    pub fn negative_edge(self) -> &'a mut W {
        self.variant(TRIGPOLW::NEGATIVE_EDGE)
    }
    #[doc = "Positive edge. A positive edge launches the conversion sequence on the selected trigger input."]
    #[inline]
    pub fn positive_edge(self) -> &'a mut W {
        self.variant(TRIGPOLW::POSITIVE_EDGE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCBYPASS`"]
pub enum SYNCBYPASSW {
    #[doc = "Enable synchronization. The hardware trigger bypass is not enabled."]
    ENABLE_SYNCHRONIZATI,
    #[doc = "Bypass synchronization. The hardware trigger bypass is enabled."]
    BYPASS_SYNCHRONIZATI,
}
impl SYNCBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCBYPASSW::ENABLE_SYNCHRONIZATI => false,
            SYNCBYPASSW::BYPASS_SYNCHRONIZATI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable synchronization. The hardware trigger bypass is not enabled."]
    #[inline]
    pub fn enable_synchronizati(self) -> &'a mut W {
        self.variant(SYNCBYPASSW::ENABLE_SYNCHRONIZATI)
    }
    #[doc = "Bypass synchronization. The hardware trigger bypass is enabled."]
    #[inline]
    pub fn bypass_synchronizati(self) -> &'a mut W {
        self.variant(SYNCBYPASSW::BYPASS_SYNCHRONIZATI)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SINGLESTEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLESTEPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "End of conversion. The sequence B interrupt/DMA flag will be set at the end of each individual A/D conversion performed under sequence B. This flag will mirror the DATAVALID bit in the SEQB_GDAT register.  The OVERRUN bit in the SEQB_GDAT register will contribute to generation of an overrun interrupt/DMA request if enabled."]
    END_OF_CONVERSION,
    #[doc = "End of sequence. The sequence B interrupt/DMA flag will be set when the entire set of sequence B conversions completes. This flag will need to be explicitly cleared by software or by the DMA-clear signal in this mode.   The OVERRUN bit in the SEQB_GDAT register will NOT contribute to generation of an overrun interrupt/DMA request since it is assumed this register will not be utilized in this mode."]
    END_OF_SEQUENCE,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::END_OF_CONVERSION => false,
            MODEW::END_OF_SEQUENCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "End of conversion. The sequence B interrupt/DMA flag will be set at the end of each individual A/D conversion performed under sequence B. This flag will mirror the DATAVALID bit in the SEQB_GDAT register. The OVERRUN bit in the SEQB_GDAT register will contribute to generation of an overrun interrupt/DMA request if enabled."]
    #[inline]
    pub fn end_of_conversion(self) -> &'a mut W {
        self.variant(MODEW::END_OF_CONVERSION)
    }
    #[doc = "End of sequence. The sequence B interrupt/DMA flag will be set when the entire set of sequence B conversions completes. This flag will need to be explicitly cleared by software or by the DMA-clear signal in this mode. The OVERRUN bit in the SEQB_GDAT register will NOT contribute to generation of an overrun interrupt/DMA request since it is assumed this register will not be utilized in this mode."]
    #[inline]
    pub fn end_of_sequence(self) -> &'a mut W {
        self.variant(MODEW::END_OF_SEQUENCE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEQB_ENA`"]
pub enum SEQB_ENAW {
    #[doc = "Disabled. Sequence B is disabled. Sequence B triggers are ignored. If this bit is cleared while sequence B is in progress, the sequence will be halted at the end of the current conversion. After the sequence is re-enabled, a new trigger will be required to restart the sequence beginning with the next enabled channel."]
    DISABLED,
    #[doc = "Enabled. Sequence B is enabled."]
    ENABLED,
}
impl SEQB_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQB_ENAW::DISABLED => false,
            SEQB_ENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQB_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQB_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQB_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Sequence B is disabled. Sequence B triggers are ignored. If this bit is cleared while sequence B is in progress, the sequence will be halted at the end of the current conversion. After the sequence is re-enabled, a new trigger will be required to restart the sequence beginning with the next enabled channel."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_ENAW::DISABLED)
    }
    #[doc = "Enabled. Sequence B is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_ENAW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Selects which one or more of the twelve channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, A/D conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while the SEQB_ENA bit (bit 31) is LOW. It is permissible to change this field and set bit 31 in the same write."]
    #[inline]
    pub fn channels(&self) -> CHANNELSR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CHANNELSR { bits }
    }
    #[doc = "Bits 12:14 - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field."]
    #[inline]
    pub fn trigger(&self) -> TRIGGERR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIGGERR { bits }
    }
    #[doc = "Bit 18 - Select the polarity of the selected input trigger for this conversion sequence."]
    #[inline]
    pub fn trigpol(&self) -> TRIGPOLR {
        TRIGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Setting this bit allows the hardware trigger input to bypass synchronization flip-flops stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode: Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode: Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
    #[inline]
    pub fn syncbypass(&self) -> SYNCBYPASSR {
        SYNCBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write a 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written-to to launch a conversion sequence. It will consequently always read-back as a zero."]
    #[inline]
    pub fn start(&self) -> STARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STARTR { bits }
    }
    #[doc = "Bit 27 - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other B triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated."]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURSTR { bits }
    }
    #[doc = "Bit 28 - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
    #[inline]
    pub fn singlestep(&self) -> SINGLESTEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLESTEPR { bits }
    }
    #[doc = "Bit 30 - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQB_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA requests for sequence-B will be generated and which overrun conditions contribute to an overrun interrupt as described below:"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Sequence Enable"]
    #[inline]
    pub fn seqb_ena(&self) -> SEQB_ENAR {
        SEQB_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Selects which one or more of the twelve channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, A/D conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while the SEQB_ENA bit (bit 31) is LOW. It is permissible to change this field and set bit 31 in the same write."]
    #[inline]
    pub fn channels(&mut self) -> _CHANNELSW {
        _CHANNELSW { w: self }
    }
    #[doc = "Bits 12:14 - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field."]
    #[inline]
    pub fn trigger(&mut self) -> _TRIGGERW {
        _TRIGGERW { w: self }
    }
    #[doc = "Bit 18 - Select the polarity of the selected input trigger for this conversion sequence."]
    #[inline]
    pub fn trigpol(&mut self) -> _TRIGPOLW {
        _TRIGPOLW { w: self }
    }
    #[doc = "Bit 19 - Setting this bit allows the hardware trigger input to bypass synchronization flip-flops stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode: Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode: Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
    #[inline]
    pub fn syncbypass(&mut self) -> _SYNCBYPASSW {
        _SYNCBYPASSW { w: self }
    }
    #[doc = "Bit 26 - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write a 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written-to to launch a conversion sequence. It will consequently always read-back as a zero."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 27 - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other B triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated."]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 28 - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
    #[inline]
    pub fn singlestep(&mut self) -> _SINGLESTEPW {
        _SINGLESTEPW { w: self }
    }
    #[doc = "Bit 30 - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQB_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA requests for sequence-B will be generated and which overrun conditions contribute to an overrun interrupt as described below:"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 31 - Sequence Enable"]
    #[inline]
    pub fn seqb_ena(&mut self) -> _SEQB_ENAW {
        _SEQB_ENAW { w: self }
    }
}
