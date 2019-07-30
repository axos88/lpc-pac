#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE3 {
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
#[doc = "Possible values of the field `P1_16MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16MODER {
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_16MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_16MODER::PULL_UP => 0,
            P1_16MODER::REPEATER => 1,
            P1_16MODER::DISABLED => 2,
            P1_16MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_16MODE_R = crate::FR<u8, P1_16MODER>;
impl P1_16MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_16MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_16MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_16MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_16MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_16MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16MODEW {
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_16MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_16MODEW::PULL_UP => 0,
            P1_16MODEW::REPEATER => 1,
            P1_16MODEW::DISABLED => 2,
            P1_16MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_16MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_16MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_16MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_16MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_16MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_16MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_16MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P1_17MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17MODER {
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_17MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_17MODER::PULL_UP => 0,
            P1_17MODER::REPEATER => 1,
            P1_17MODER::DISABLED => 2,
            P1_17MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_17MODE_R = crate::FR<u8, P1_17MODER>;
impl P1_17MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_17MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_17MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_17MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_17MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_17MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17MODEW {
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_17MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_17MODEW::PULL_UP => 0,
            P1_17MODEW::REPEATER => 1,
            P1_17MODEW::DISABLED => 2,
            P1_17MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_17MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_17MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_17MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_17MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_17MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_17MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_17MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P1_18MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18MODER {
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_18MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_18MODER::PULL_UP => 0,
            P1_18MODER::REPEATER => 1,
            P1_18MODER::DISABLED => 2,
            P1_18MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_18MODE_R = crate::FR<u8, P1_18MODER>;
impl P1_18MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_18MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_18MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_18MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_18MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_18MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18MODEW {
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_18MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_18MODEW::PULL_UP => 0,
            P1_18MODEW::REPEATER => 1,
            P1_18MODEW::DISABLED => 2,
            P1_18MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_18MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_18MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_18MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_18MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_18MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_18MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_18MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P1_19MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19MODER {
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_19MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_19MODER::PULL_UP => 0,
            P1_19MODER::REPEATER => 1,
            P1_19MODER::DISABLED => 2,
            P1_19MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_19MODE_R = crate::FR<u8, P1_19MODER>;
impl P1_19MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_19MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_19MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_19MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_19MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_19MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19MODEW {
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_19MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_19MODEW::PULL_UP => 0,
            P1_19MODEW::REPEATER => 1,
            P1_19MODEW::DISABLED => 2,
            P1_19MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_19MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_19MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_19MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_19MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_19MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_19MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_19MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P1_20MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20MODER {
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_20MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_20MODER::PULL_UP => 0,
            P1_20MODER::REPEATER => 1,
            P1_20MODER::DISABLED => 2,
            P1_20MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_20MODE_R = crate::FR<u8, P1_20MODER>;
impl P1_20MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_20MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_20MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_20MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_20MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_20MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20MODEW {
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_20MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_20MODEW::PULL_UP => 0,
            P1_20MODEW::REPEATER => 1,
            P1_20MODEW::DISABLED => 2,
            P1_20MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_20MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_20MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_20MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_20MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_20MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_20MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_20MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P1_21MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21MODER {
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_21MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_21MODER::PULL_UP => 0,
            P1_21MODER::REPEATER => 1,
            P1_21MODER::DISABLED => 2,
            P1_21MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_21MODE_R = crate::FR<u8, P1_21MODER>;
impl P1_21MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_21MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_21MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_21MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_21MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_21MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21MODEW {
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_21MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_21MODEW::PULL_UP => 0,
            P1_21MODEW::REPEATER => 1,
            P1_21MODEW::DISABLED => 2,
            P1_21MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_21MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_21MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_21MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_21MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_21MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_21MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_21MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P1_22MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22MODER {
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_22MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_22MODER::PULL_UP => 0,
            P1_22MODER::REPEATER => 1,
            P1_22MODER::DISABLED => 2,
            P1_22MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_22MODE_R = crate::FR<u8, P1_22MODER>;
impl P1_22MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_22MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_22MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_22MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_22MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_22MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22MODEW {
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_22MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_22MODEW::PULL_UP => 0,
            P1_22MODEW::REPEATER => 1,
            P1_22MODEW::DISABLED => 2,
            P1_22MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_22MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_22MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_22MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_22MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_22MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_22MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_22MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P1_23MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23MODER {
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_23MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_23MODER::PULL_UP => 0,
            P1_23MODER::REPEATER => 1,
            P1_23MODER::DISABLED => 2,
            P1_23MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_23MODE_R = crate::FR<u8, P1_23MODER>;
impl P1_23MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_23MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_23MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_23MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_23MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_23MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23MODEW {
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_23MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_23MODEW::PULL_UP => 0,
            P1_23MODEW::REPEATER => 1,
            P1_23MODEW::DISABLED => 2,
            P1_23MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_23MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_23MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_23MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_23MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_23MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_23MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_23MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P1_24MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24MODER {
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_24MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_24MODER::PULL_UP => 0,
            P1_24MODER::REPEATER => 1,
            P1_24MODER::DISABLED => 2,
            P1_24MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_24MODE_R = crate::FR<u8, P1_24MODER>;
impl P1_24MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_24MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_24MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_24MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_24MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_24MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24MODEW {
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_24MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_24MODEW::PULL_UP => 0,
            P1_24MODEW::REPEATER => 1,
            P1_24MODEW::DISABLED => 2,
            P1_24MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_24MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_24MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_24MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_24MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_24MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_24MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_24MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P1_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25MODER {
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_25MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_25MODER::PULL_UP => 0,
            P1_25MODER::REPEATER => 1,
            P1_25MODER::DISABLED => 2,
            P1_25MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_25MODE_R = crate::FR<u8, P1_25MODER>;
impl P1_25MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_25MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_25MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_25MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_25MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25MODEW {
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_25MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_25MODEW::PULL_UP => 0,
            P1_25MODEW::REPEATER => 1,
            P1_25MODEW::DISABLED => 2,
            P1_25MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_25MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_25MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_25MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_25MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_25MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_25MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_25MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P1_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26MODER {
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_26MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_26MODER::PULL_UP => 0,
            P1_26MODER::REPEATER => 1,
            P1_26MODER::DISABLED => 2,
            P1_26MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_26MODE_R = crate::FR<u8, P1_26MODER>;
impl P1_26MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_26MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_26MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_26MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_26MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26MODEW {
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_26MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_26MODEW::PULL_UP => 0,
            P1_26MODEW::REPEATER => 1,
            P1_26MODEW::DISABLED => 2,
            P1_26MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_26MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_26MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_26MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_26MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_26MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_26MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_26MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P1_27MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27MODER {
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_27MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_27MODER::PULL_UP => 0,
            P1_27MODER::REPEATER => 1,
            P1_27MODER::DISABLED => 2,
            P1_27MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_27MODE_R = crate::FR<u8, P1_27MODER>;
impl P1_27MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_27MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_27MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_27MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_27MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_27MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27MODEW {
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_27MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_27MODEW::PULL_UP => 0,
            P1_27MODEW::REPEATER => 1,
            P1_27MODEW::DISABLED => 2,
            P1_27MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_27MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_27MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_27MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_27MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_27MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_27MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_27MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P1_28MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28MODER {
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_28MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_28MODER::PULL_UP => 0,
            P1_28MODER::REPEATER => 1,
            P1_28MODER::DISABLED => 2,
            P1_28MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_28MODE_R = crate::FR<u8, P1_28MODER>;
impl P1_28MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_28MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_28MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_28MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_28MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_28MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28MODEW {
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_28MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_28MODEW::PULL_UP => 0,
            P1_28MODEW::REPEATER => 1,
            P1_28MODEW::DISABLED => 2,
            P1_28MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_28MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_28MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_28MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_28MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_28MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_28MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_28MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P1_29MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29MODER {
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_29MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_29MODER::PULL_UP => 0,
            P1_29MODER::REPEATER => 1,
            P1_29MODER::DISABLED => 2,
            P1_29MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_29MODE_R = crate::FR<u8, P1_29MODER>;
impl P1_29MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_29MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_29MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_29MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_29MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_29MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29MODEW {
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_29MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_29MODEW::PULL_UP => 0,
            P1_29MODEW::REPEATER => 1,
            P1_29MODEW::DISABLED => 2,
            P1_29MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_29MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_29MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_29MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_29MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_29MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_29MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_29MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `P1_30MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30MODER {
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_30MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_30MODER::PULL_UP => 0,
            P1_30MODER::REPEATER => 1,
            P1_30MODER::DISABLED => 2,
            P1_30MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_30MODE_R = crate::FR<u8, P1_30MODER>;
impl P1_30MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_30MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_30MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_30MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_30MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_30MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30MODEW {
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_30MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_30MODEW::PULL_UP => 0,
            P1_30MODEW::REPEATER => 1,
            P1_30MODEW::DISABLED => 2,
            P1_30MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_30MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_30MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_30MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_30MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_30MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_30MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_30MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `P1_31MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31MODER {
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_31MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_31MODER::PULL_UP => 0,
            P1_31MODER::REPEATER => 1,
            P1_31MODER::DISABLED => 2,
            P1_31MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_31MODE_R = crate::FR<u8, P1_31MODER>;
impl P1_31MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_31MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_31MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_31MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_31MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_31MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31MODEW {
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_31MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_31MODEW::PULL_UP => 0,
            P1_31MODEW::REPEATER => 1,
            P1_31MODEW::DISABLED => 2,
            P1_31MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_31MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_31MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_31MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_31MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_31MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_31MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_31MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&self) -> P1_16MODE_R {
        P1_16MODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&self) -> P1_17MODE_R {
        P1_17MODE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&self) -> P1_18MODE_R {
        P1_18MODE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&self) -> P1_19MODE_R {
        P1_19MODE_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&self) -> P1_20MODE_R {
        P1_20MODE_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&self) -> P1_21MODE_R {
        P1_21MODE_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&self) -> P1_22MODE_R {
        P1_22MODE_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&self) -> P1_23MODE_R {
        P1_23MODE_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&self) -> P1_24MODE_R {
        P1_24MODE_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&self) -> P1_25MODE_R {
        P1_25MODE_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&self) -> P1_26MODE_R {
        P1_26MODE_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&self) -> P1_27MODE_R {
        P1_27MODE_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&self) -> P1_28MODE_R {
        P1_28MODE_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&self) -> P1_29MODE_R {
        P1_29MODE_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&self) -> P1_30MODE_R {
        P1_30MODE_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&self) -> P1_31MODE_R {
        P1_31MODE_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&mut self) -> _P1_16MODEW {
        _P1_16MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&mut self) -> _P1_17MODEW {
        _P1_17MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&mut self) -> _P1_18MODEW {
        _P1_18MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&mut self) -> _P1_19MODEW {
        _P1_19MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&mut self) -> _P1_20MODEW {
        _P1_20MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&mut self) -> _P1_21MODEW {
        _P1_21MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&mut self) -> _P1_22MODEW {
        _P1_22MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&mut self) -> _P1_23MODEW {
        _P1_23MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&mut self) -> _P1_24MODEW {
        _P1_24MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&mut self) -> _P1_25MODEW {
        _P1_25MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&mut self) -> _P1_26MODEW {
        _P1_26MODEW { w: self }
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&mut self) -> _P1_27MODEW {
        _P1_27MODEW { w: self }
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&mut self) -> _P1_28MODEW {
        _P1_28MODEW { w: self }
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&mut self) -> _P1_29MODEW {
        _P1_29MODEW { w: self }
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&mut self) -> _P1_30MODEW {
        _P1_30MODEW { w: self }
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&mut self) -> _P1_31MODEW {
        _P1_31MODEW { w: self }
    }
}
