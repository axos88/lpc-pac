#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE7 {
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
#[doc = "Possible values of the field `P3_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25MODER {
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P3_25MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P3_25MODER::PULL_UP => 0,
            P3_25MODER::REPEATER => 1,
            P3_25MODER::DISABLED => 2,
            P3_25MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P3_25MODE_R = crate::FR<u8, P3_25MODER>;
impl P3_25MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_25MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_25MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_25MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_25MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P3_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25MODEW {
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_25MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_25MODEW::PULL_UP => 0,
            P3_25MODEW::REPEATER => 1,
            P3_25MODEW::DISABLED => 2,
            P3_25MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P3_25MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_25MODEW::PULL_UP)
    }
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_25MODEW::REPEATER)
    }
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_25MODEW::DISABLED)
    }
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_25MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P3_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26MODER {
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P3_26MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P3_26MODER::PULL_UP => 0,
            P3_26MODER::REPEATER => 1,
            P3_26MODER::DISABLED => 2,
            P3_26MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P3_26MODE_R = crate::FR<u8, P3_26MODER>;
impl P3_26MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_26MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_26MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_26MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_26MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P3_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26MODEW {
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_26MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_26MODEW::PULL_UP => 0,
            P3_26MODEW::REPEATER => 1,
            P3_26MODEW::DISABLED => 2,
            P3_26MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P3_26MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_26MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_26MODEW::PULL_UP)
    }
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_26MODEW::REPEATER)
    }
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_26MODEW::DISABLED)
    }
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_26MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&self) -> P3_25MODE_R {
        P3_25MODE_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&self) -> P3_26MODE_R {
        P3_26MODE_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&mut self) -> _P3_25MODEW {
        _P3_25MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&mut self) -> _P3_26MODEW {
        _P3_26MODEW { w: self }
    }
}
