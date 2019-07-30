#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE9 {
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
#[doc = "Possible values of the field `P4_28MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28MODER {
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P4_28MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P4_28MODER::PULL_UP => 0,
            P4_28MODER::REPEATER => 1,
            P4_28MODER::DISABLED => 2,
            P4_28MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P4_28MODE_R = crate::FR<u8, P4_28MODER>;
impl P4_28MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_28MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P4_28MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P4_28MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_28MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P4_28MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28MODEW {
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_28MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_28MODEW::PULL_UP => 0,
            P4_28MODEW::REPEATER => 1,
            P4_28MODEW::DISABLED => 2,
            P4_28MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P4_28MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_28MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_28MODEW::PULL_UP)
    }
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_28MODEW::REPEATER)
    }
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_28MODEW::DISABLED)
    }
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_28MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P4_29MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29MODER {
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P4_29MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P4_29MODER::PULL_UP => 0,
            P4_29MODER::REPEATER => 1,
            P4_29MODER::DISABLED => 2,
            P4_29MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P4_29MODE_R = crate::FR<u8, P4_29MODER>;
impl P4_29MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_29MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P4_29MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P4_29MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_29MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P4_29MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29MODEW {
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_29MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_29MODEW::PULL_UP => 0,
            P4_29MODEW::REPEATER => 1,
            P4_29MODEW::DISABLED => 2,
            P4_29MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P4_29MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_29MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_29MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_29MODEW::PULL_UP)
    }
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_29MODEW::REPEATER)
    }
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_29MODEW::DISABLED)
    }
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_29MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&self) -> P4_28MODE_R {
        P4_28MODE_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&self) -> P4_29MODE_R {
        P4_29MODE_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&mut self) -> _P4_28MODEW {
        _P4_28MODEW { w: self }
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&mut self) -> _P4_29MODEW {
        _P4_29MODEW { w: self }
    }
}
