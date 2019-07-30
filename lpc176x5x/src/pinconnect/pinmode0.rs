#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE0 {
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
#[doc = "Possible values of the field `P0_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00MODER {
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_00MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_00MODER::PULL_UP => 0,
            P0_00MODER::REPEATER => 1,
            P0_00MODER::DISABLED => 2,
            P0_00MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_00MODE_R = crate::FR<u8, P0_00MODER>;
impl P0_00MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_00MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00MODEW {
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_00MODEW::PULL_UP => 0,
            P0_00MODEW::REPEATER => 1,
            P0_00MODEW::DISABLED => 2,
            P0_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_00MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_00MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P0_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01MODER {
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_01MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_01MODER::PULL_UP => 0,
            P0_01MODER::REPEATER => 1,
            P0_01MODER::DISABLED => 2,
            P0_01MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_01MODE_R = crate::FR<u8, P0_01MODER>;
impl P0_01MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_01MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01MODEW {
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_01MODEW::PULL_UP => 0,
            P0_01MODEW::REPEATER => 1,
            P0_01MODEW::DISABLED => 2,
            P0_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_01MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_01MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P0_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02MODER {
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_02MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_02MODER::PULL_UP => 0,
            P0_02MODER::REPEATER => 1,
            P0_02MODER::DISABLED => 2,
            P0_02MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_02MODE_R = crate::FR<u8, P0_02MODER>;
impl P0_02MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_02MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_02MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_02MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_02MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02MODEW {
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_02MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_02MODEW::PULL_UP => 0,
            P0_02MODEW::REPEATER => 1,
            P0_02MODEW::DISABLED => 2,
            P0_02MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_02MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_02MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_02MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_02MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_02MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_02MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_02MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P0_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03MODER {
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_03MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_03MODER::PULL_UP => 0,
            P0_03MODER::REPEATER => 1,
            P0_03MODER::DISABLED => 2,
            P0_03MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_03MODE_R = crate::FR<u8, P0_03MODER>;
impl P0_03MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_03MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_03MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_03MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_03MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03MODEW {
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_03MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_03MODEW::PULL_UP => 0,
            P0_03MODEW::REPEATER => 1,
            P0_03MODEW::DISABLED => 2,
            P0_03MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_03MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_03MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_03MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_03MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_03MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_03MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_03MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P0_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04MODER {
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_04MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_04MODER::PULL_UP => 0,
            P0_04MODER::REPEATER => 1,
            P0_04MODER::DISABLED => 2,
            P0_04MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_04MODE_R = crate::FR<u8, P0_04MODER>;
impl P0_04MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_04MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04MODEW {
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_04MODEW::PULL_UP => 0,
            P0_04MODEW::REPEATER => 1,
            P0_04MODEW::DISABLED => 2,
            P0_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_04MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_04MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P0_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05MODER {
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_05MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_05MODER::PULL_UP => 0,
            P0_05MODER::REPEATER => 1,
            P0_05MODER::DISABLED => 2,
            P0_05MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_05MODE_R = crate::FR<u8, P0_05MODER>;
impl P0_05MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_05MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_05MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_05MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_05MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05MODEW {
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_05MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_05MODEW::PULL_UP => 0,
            P0_05MODEW::REPEATER => 1,
            P0_05MODEW::DISABLED => 2,
            P0_05MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_05MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_05MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_05MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_05MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_05MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_05MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_05MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P0_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06MODER {
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_06MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_06MODER::PULL_UP => 0,
            P0_06MODER::REPEATER => 1,
            P0_06MODER::DISABLED => 2,
            P0_06MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_06MODE_R = crate::FR<u8, P0_06MODER>;
impl P0_06MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_06MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_06MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_06MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_06MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06MODEW {
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_06MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_06MODEW::PULL_UP => 0,
            P0_06MODEW::REPEATER => 1,
            P0_06MODEW::DISABLED => 2,
            P0_06MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_06MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_06MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_06MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_06MODEW::PULL_UP)
    }
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_06MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_06MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_06MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P0_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07MODER {
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_07MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_07MODER::PULL_UP => 0,
            P0_07MODER::REPEATER => 1,
            P0_07MODER::DISABLED => 2,
            P0_07MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_07MODE_R = crate::FR<u8, P0_07MODER>;
impl P0_07MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_07MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_07MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_07MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_07MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07MODEW {
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_07MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_07MODEW::PULL_UP => 0,
            P0_07MODEW::REPEATER => 1,
            P0_07MODEW::DISABLED => 2,
            P0_07MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_07MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_07MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_07MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_07MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_07MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_07MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_07MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P0_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08MODER {
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_08MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_08MODER::PULL_UP => 0,
            P0_08MODER::REPEATER => 1,
            P0_08MODER::DISABLED => 2,
            P0_08MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_08MODE_R = crate::FR<u8, P0_08MODER>;
impl P0_08MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_08MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08MODEW {
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_08MODEW::PULL_UP => 0,
            P0_08MODEW::REPEATER => 1,
            P0_08MODEW::DISABLED => 2,
            P0_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_08MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_08MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P0_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09MODER {
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_09MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_09MODER::PULL_UP => 0,
            P0_09MODER::REPEATER => 1,
            P0_09MODER::DISABLED => 2,
            P0_09MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_09MODE_R = crate::FR<u8, P0_09MODER>;
impl P0_09MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_09MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09MODEW {
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_09MODEW::PULL_UP => 0,
            P0_09MODEW::REPEATER => 1,
            P0_09MODEW::DISABLED => 2,
            P0_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_09MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_09MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P0_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10MODER {
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_10MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_10MODER::PULL_UP => 0,
            P0_10MODER::REPEATER => 1,
            P0_10MODER::DISABLED => 2,
            P0_10MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_10MODE_R = crate::FR<u8, P0_10MODER>;
impl P0_10MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_10MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10MODEW {
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_10MODEW::PULL_UP => 0,
            P0_10MODEW::REPEATER => 1,
            P0_10MODEW::DISABLED => 2,
            P0_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_10MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P0_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11MODER {
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_11MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_11MODER::PULL_UP => 0,
            P0_11MODER::REPEATER => 1,
            P0_11MODER::DISABLED => 2,
            P0_11MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_11MODE_R = crate::FR<u8, P0_11MODER>;
impl P0_11MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_11MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_11MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_11MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_11MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11MODEW {
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_11MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_11MODEW::PULL_UP => 0,
            P0_11MODEW::REPEATER => 1,
            P0_11MODEW::DISABLED => 2,
            P0_11MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_11MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_11MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_11MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_11MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_11MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P0_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15MODER {
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P0_15MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P0_15MODER::PULL_UP => 0,
            P0_15MODER::REPEATER => 1,
            P0_15MODER::DISABLED => 2,
            P0_15MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_15MODE_R = crate::FR<u8, P0_15MODER>;
impl P0_15MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_15MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_15MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_15MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_15MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15MODEW {
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_15MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_15MODEW::PULL_UP => 0,
            P0_15MODEW::REPEATER => 1,
            P0_15MODEW::DISABLED => 2,
            P0_15MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P0_15MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_15MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_15MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_15MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_15MODEW::PULL_DOWN)
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
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&self) -> P0_00MODE_R {
        P0_00MODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&self) -> P0_01MODE_R {
        P0_01MODE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&self) -> P0_02MODE_R {
        P0_02MODE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&self) -> P0_03MODE_R {
        P0_03MODE_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&self) -> P0_04MODE_R {
        P0_04MODE_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&self) -> P0_05MODE_R {
        P0_05MODE_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&self) -> P0_06MODE_R {
        P0_06MODE_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&self) -> P0_07MODE_R {
        P0_07MODE_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&self) -> P0_08MODE_R {
        P0_08MODE_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&self) -> P0_09MODE_R {
        P0_09MODE_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&self) -> P0_10MODE_R {
        P0_10MODE_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&self) -> P0_11MODE_R {
        P0_11MODE_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&self) -> P0_15MODE_R {
        P0_15MODE_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&mut self) -> _P0_00MODEW {
        _P0_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&mut self) -> _P0_01MODEW {
        _P0_01MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&mut self) -> _P0_02MODEW {
        _P0_02MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&mut self) -> _P0_03MODEW {
        _P0_03MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&mut self) -> _P0_04MODEW {
        _P0_04MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&mut self) -> _P0_05MODEW {
        _P0_05MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&mut self) -> _P0_06MODEW {
        _P0_06MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&mut self) -> _P0_07MODEW {
        _P0_07MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&mut self) -> _P0_08MODEW {
        _P0_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&mut self) -> _P0_09MODEW {
        _P0_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&mut self) -> _P0_10MODEW {
        _P0_10MODEW { w: self }
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&mut self) -> _P0_11MODEW {
        _P0_11MODEW { w: self }
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&mut self) -> _P0_15MODEW {
        _P0_15MODEW { w: self }
    }
}
