#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE4 {
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
#[doc = "Possible values of the field `P2_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00MODER {
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_00MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_00MODER::PULL_UP => 0,
            P2_00MODER::REPEATER => 1,
            P2_00MODER::DISABLED => 2,
            P2_00MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_00MODE_R = crate::FR<u8, P2_00MODER>;
impl P2_00MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_00MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00MODEW {
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_00MODEW::PULL_UP => 0,
            P2_00MODEW::REPEATER => 1,
            P2_00MODEW::DISABLED => 2,
            P2_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_00MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_00MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `P2_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01MODER {
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_01MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_01MODER::PULL_UP => 0,
            P2_01MODER::REPEATER => 1,
            P2_01MODER::DISABLED => 2,
            P2_01MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_01MODE_R = crate::FR<u8, P2_01MODER>;
impl P2_01MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_01MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01MODEW {
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_01MODEW::PULL_UP => 0,
            P2_01MODEW::REPEATER => 1,
            P2_01MODEW::DISABLED => 2,
            P2_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_01MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_01MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `P2_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02MODER {
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_02MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_02MODER::PULL_UP => 0,
            P2_02MODER::REPEATER => 1,
            P2_02MODER::DISABLED => 2,
            P2_02MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_02MODE_R = crate::FR<u8, P2_02MODER>;
impl P2_02MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_02MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_02MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_02MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_02MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02MODEW {
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_02MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_02MODEW::PULL_UP => 0,
            P2_02MODEW::REPEATER => 1,
            P2_02MODEW::DISABLED => 2,
            P2_02MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_02MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_02MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_02MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_02MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_02MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_02MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_02MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `P2_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03MODER {
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_03MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_03MODER::PULL_UP => 0,
            P2_03MODER::REPEATER => 1,
            P2_03MODER::DISABLED => 2,
            P2_03MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_03MODE_R = crate::FR<u8, P2_03MODER>;
impl P2_03MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_03MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_03MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_03MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_03MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03MODEW {
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_03MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_03MODEW::PULL_UP => 0,
            P2_03MODEW::REPEATER => 1,
            P2_03MODEW::DISABLED => 2,
            P2_03MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_03MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_03MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_03MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_03MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_03MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_03MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_03MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `P2_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04MODER {
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_04MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_04MODER::PULL_UP => 0,
            P2_04MODER::REPEATER => 1,
            P2_04MODER::DISABLED => 2,
            P2_04MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_04MODE_R = crate::FR<u8, P2_04MODER>;
impl P2_04MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_04MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04MODEW {
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_04MODEW::PULL_UP => 0,
            P2_04MODEW::REPEATER => 1,
            P2_04MODEW::DISABLED => 2,
            P2_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_04MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_04MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `P2_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05MODER {
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_05MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_05MODER::PULL_UP => 0,
            P2_05MODER::REPEATER => 1,
            P2_05MODER::DISABLED => 2,
            P2_05MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_05MODE_R = crate::FR<u8, P2_05MODER>;
impl P2_05MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_05MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_05MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_05MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_05MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05MODEW {
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_05MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_05MODEW::PULL_UP => 0,
            P2_05MODEW::REPEATER => 1,
            P2_05MODEW::DISABLED => 2,
            P2_05MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_05MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_05MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_05MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_05MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_05MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_05MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_05MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `P2_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06MODER {
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_06MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_06MODER::PULL_UP => 0,
            P2_06MODER::REPEATER => 1,
            P2_06MODER::DISABLED => 2,
            P2_06MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_06MODE_R = crate::FR<u8, P2_06MODER>;
impl P2_06MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_06MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_06MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_06MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_06MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06MODEW {
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_06MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_06MODEW::PULL_UP => 0,
            P2_06MODEW::REPEATER => 1,
            P2_06MODEW::DISABLED => 2,
            P2_06MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_06MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_06MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_06MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_06MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_06MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_06MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_06MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `P2_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07MODER {
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_07MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_07MODER::PULL_UP => 0,
            P2_07MODER::REPEATER => 1,
            P2_07MODER::DISABLED => 2,
            P2_07MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_07MODE_R = crate::FR<u8, P2_07MODER>;
impl P2_07MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_07MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_07MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_07MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_07MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07MODEW {
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_07MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_07MODEW::PULL_UP => 0,
            P2_07MODEW::REPEATER => 1,
            P2_07MODEW::DISABLED => 2,
            P2_07MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_07MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_07MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_07MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_07MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_07MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_07MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_07MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `P2_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08MODER {
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_08MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_08MODER::PULL_UP => 0,
            P2_08MODER::REPEATER => 1,
            P2_08MODER::DISABLED => 2,
            P2_08MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_08MODE_R = crate::FR<u8, P2_08MODER>;
impl P2_08MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_08MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08MODEW {
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_08MODEW::PULL_UP => 0,
            P2_08MODEW::REPEATER => 1,
            P2_08MODEW::DISABLED => 2,
            P2_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_08MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_08MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `P2_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09MODER {
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_09MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_09MODER::PULL_UP => 0,
            P2_09MODER::REPEATER => 1,
            P2_09MODER::DISABLED => 2,
            P2_09MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_09MODE_R = crate::FR<u8, P2_09MODER>;
impl P2_09MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_09MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09MODEW {
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_09MODEW::PULL_UP => 0,
            P2_09MODEW::REPEATER => 1,
            P2_09MODEW::DISABLED => 2,
            P2_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_09MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_09MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `P2_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10MODER {
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_10MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_10MODER::PULL_UP => 0,
            P2_10MODER::REPEATER => 1,
            P2_10MODER::DISABLED => 2,
            P2_10MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_10MODE_R = crate::FR<u8, P2_10MODER>;
impl P2_10MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_10MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10MODEW {
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_10MODEW::PULL_UP => 0,
            P2_10MODEW::REPEATER => 1,
            P2_10MODEW::DISABLED => 2,
            P2_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_10MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `P2_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11MODER {
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_11MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_11MODER::PULL_UP => 0,
            P2_11MODER::REPEATER => 1,
            P2_11MODER::DISABLED => 2,
            P2_11MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_11MODE_R = crate::FR<u8, P2_11MODER>;
impl P2_11MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_11MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_11MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_11MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_11MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11MODEW {
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_11MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_11MODEW::PULL_UP => 0,
            P2_11MODEW::REPEATER => 1,
            P2_11MODEW::DISABLED => 2,
            P2_11MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_11MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_11MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_11MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_11MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_11MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_11MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `P2_12MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12MODER {
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_12MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_12MODER::PULL_UP => 0,
            P2_12MODER::REPEATER => 1,
            P2_12MODER::DISABLED => 2,
            P2_12MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_12MODE_R = crate::FR<u8, P2_12MODER>;
impl P2_12MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_12MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_12MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_12MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_12MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_12MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12MODEW {
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_12MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_12MODEW::PULL_UP => 0,
            P2_12MODEW::REPEATER => 1,
            P2_12MODEW::DISABLED => 2,
            P2_12MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_12MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_12MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_12MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_12MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_12MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_12MODEW::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `P2_13MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13MODER {
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl crate::ToBits<u8> for P2_13MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            P2_13MODER::PULL_UP => 0,
            P2_13MODER::REPEATER => 1,
            P2_13MODER::DISABLED => 2,
            P2_13MODER::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2_13MODE_R = crate::FR<u8, P2_13MODER>;
impl P2_13MODE_R {
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_13MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_13MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_13MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_13MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_13MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13MODEW {
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_13MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_13MODEW::PULL_UP => 0,
            P2_13MODEW::REPEATER => 1,
            P2_13MODEW::DISABLED => 2,
            P2_13MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _P2_13MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_13MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_13MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_13MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_13MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_13MODEW::PULL_DOWN)
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
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&self) -> P2_00MODE_R {
        P2_00MODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&self) -> P2_01MODE_R {
        P2_01MODE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&self) -> P2_02MODE_R {
        P2_02MODE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&self) -> P2_03MODE_R {
        P2_03MODE_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&self) -> P2_04MODE_R {
        P2_04MODE_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&self) -> P2_05MODE_R {
        P2_05MODE_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&self) -> P2_06MODE_R {
        P2_06MODE_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&self) -> P2_07MODE_R {
        P2_07MODE_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&self) -> P2_08MODE_R {
        P2_08MODE_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&self) -> P2_09MODE_R {
        P2_09MODE_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&self) -> P2_10MODE_R {
        P2_10MODE_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&self) -> P2_11MODE_R {
        P2_11MODE_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&self) -> P2_12MODE_R {
        P2_12MODE_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&self) -> P2_13MODE_R {
        P2_13MODE_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&mut self) -> _P2_00MODEW {
        _P2_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&mut self) -> _P2_01MODEW {
        _P2_01MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&mut self) -> _P2_02MODEW {
        _P2_02MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&mut self) -> _P2_03MODEW {
        _P2_03MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&mut self) -> _P2_04MODEW {
        _P2_04MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&mut self) -> _P2_05MODEW {
        _P2_05MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&mut self) -> _P2_06MODEW {
        _P2_06MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&mut self) -> _P2_07MODEW {
        _P2_07MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&mut self) -> _P2_08MODEW {
        _P2_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&mut self) -> _P2_09MODEW {
        _P2_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&mut self) -> _P2_10MODEW {
        _P2_10MODEW { w: self }
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&mut self) -> _P2_11MODEW {
        _P2_11MODEW { w: self }
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&mut self) -> _P2_12MODEW {
        _P2_12MODEW { w: self }
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&mut self) -> _P2_13MODEW {
        _P2_13MODEW { w: self }
    }
}
