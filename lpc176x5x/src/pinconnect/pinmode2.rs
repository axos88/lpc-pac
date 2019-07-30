#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE2 {
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
#[doc = "Possible values of the field `P1_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_00MODER {
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_00MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_00MODER::PULL_UP => 0,
            P1_00MODER::REPEATER => 1,
            P1_00MODER::DISABLED => 2,
            P1_00MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_00MODE_R = crate::FR<u8, P1_00MODER>;
impl P1_00MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_00MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_00MODEW {
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_00MODEW::PULL_UP => 0,
            P1_00MODEW::REPEATER => 1,
            P1_00MODEW::DISABLED => 2,
            P1_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_00MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_00MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P1_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_01MODER {
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_01MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_01MODER::PULL_UP => 0,
            P1_01MODER::REPEATER => 1,
            P1_01MODER::DISABLED => 2,
            P1_01MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_01MODE_R = crate::FR<u8, P1_01MODER>;
impl P1_01MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_01MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_01MODEW {
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_01MODEW::PULL_UP => 0,
            P1_01MODEW::REPEATER => 1,
            P1_01MODEW::DISABLED => 2,
            P1_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_01MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_01MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P1_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_04MODER {
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_04MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_04MODER::PULL_UP => 0,
            P1_04MODER::REPEATER => 1,
            P1_04MODER::DISABLED => 2,
            P1_04MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_04MODE_R = crate::FR<u8, P1_04MODER>;
impl P1_04MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_04MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_04MODEW {
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_04MODEW::PULL_UP => 0,
            P1_04MODEW::REPEATER => 1,
            P1_04MODEW::DISABLED => 2,
            P1_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_04MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_04MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P1_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_08MODER {
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_08MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_08MODER::PULL_UP => 0,
            P1_08MODER::REPEATER => 1,
            P1_08MODER::DISABLED => 2,
            P1_08MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_08MODE_R = crate::FR<u8, P1_08MODER>;
impl P1_08MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_08MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_08MODEW {
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_08MODEW::PULL_UP => 0,
            P1_08MODEW::REPEATER => 1,
            P1_08MODEW::DISABLED => 2,
            P1_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_08MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_08MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P1_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_09MODER {
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_09MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_09MODER::PULL_UP => 0,
            P1_09MODER::REPEATER => 1,
            P1_09MODER::DISABLED => 2,
            P1_09MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_09MODE_R = crate::FR<u8, P1_09MODER>;
impl P1_09MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_09MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_09MODEW {
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_09MODEW::PULL_UP => 0,
            P1_09MODEW::REPEATER => 1,
            P1_09MODEW::DISABLED => 2,
            P1_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_09MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_09MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P1_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10MODER {
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_10MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_10MODER::PULL_UP => 0,
            P1_10MODER::REPEATER => 1,
            P1_10MODER::DISABLED => 2,
            P1_10MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_10MODE_R = crate::FR<u8, P1_10MODER>;
impl P1_10MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_10MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10MODEW {
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_10MODEW::PULL_UP => 0,
            P1_10MODEW::REPEATER => 1,
            P1_10MODEW::DISABLED => 2,
            P1_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_10MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_10MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P1_14MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14MODER {
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_14MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_14MODER::PULL_UP => 0,
            P1_14MODER::REPEATER => 1,
            P1_14MODER::DISABLED => 2,
            P1_14MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_14MODE_R = crate::FR<u8, P1_14MODER>;
impl P1_14MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_14MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_14MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_14MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_14MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_14MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14MODEW {
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_14MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_14MODEW::PULL_UP => 0,
            P1_14MODEW::REPEATER => 1,
            P1_14MODEW::DISABLED => 2,
            P1_14MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_14MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_14MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_14MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_14MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_14MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_14MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_14MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `P1_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15MODER {
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P1_15MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P1_15MODER::PULL_UP => 0,
            P1_15MODER::REPEATER => 1,
            P1_15MODER::DISABLED => 2,
            P1_15MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P1_15MODE_R = crate::FR<u8, P1_15MODER>;
impl P1_15MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_15MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_15MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_15MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_15MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15MODEW {
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_15MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_15MODEW::PULL_UP => 0,
            P1_15MODEW::REPEATER => 1,
            P1_15MODEW::DISABLED => 2,
            P1_15MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P1_15MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_15MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_15MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_15MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_15MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_15MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_15MODEW::PULL_DOWN)
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
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&self) -> P1_00MODE_R {
        P1_00MODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&self) -> P1_01MODE_R {
        P1_01MODE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&self) -> P1_04MODE_R {
        P1_04MODE_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&self) -> P1_08MODE_R {
        P1_08MODE_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&self) -> P1_09MODE_R {
        P1_09MODE_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&self) -> P1_10MODE_R {
        P1_10MODE_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&self) -> P1_14MODE_R {
        P1_14MODE_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&self) -> P1_15MODE_R {
        P1_15MODE_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&mut self) -> _P1_00MODEW {
        _P1_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&mut self) -> _P1_01MODEW {
        _P1_01MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&mut self) -> _P1_04MODEW {
        _P1_04MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&mut self) -> _P1_08MODEW {
        _P1_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&mut self) -> _P1_09MODEW {
        _P1_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&mut self) -> _P1_10MODEW {
        _P1_10MODEW { w: self }
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&mut self) -> _P1_14MODEW {
        _P1_14MODEW { w: self }
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&mut self) -> _P1_15MODEW {
        _P1_15MODEW { w: self }
    }
}
