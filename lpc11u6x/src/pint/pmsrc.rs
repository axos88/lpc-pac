#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMSRC {
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
#[doc = "Possible values of the field `SRC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC0R {
    #[doc = "Input 0. Selects the output of pin interrupt select register 0 as the source to bit slice 0."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects the output of pin interrupt select register 1 as the source to bit slice 0."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects the output of pin interrupt select register 2 as the source to bit slice 0."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects the output of pin interrupt select register 3 as the source to bit slice 0."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects the output of pin interrupt select register 4 as the source to bit slice 0."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects the output of pin interrupt select register 5 as the source to bit slice 0."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects the output of pin interrupt select register 6 as the source to bit slice 0."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects the output of pin interrupt select register 7 as the source to bit slice 0."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC0R::INPUT_0_SELECTS_PIN => 0,
            SRC0R::INPUT_1_SELECTS_PIN => 1,
            SRC0R::INPUT_2_SELECTS_PIN => 2,
            SRC0R::INPUT_3_SELECTS_PIN => 3,
            SRC0R::INPUT_4_SELECTS_PIN => 4,
            SRC0R::INPUT_5_SELECTS_PIN => 5,
            SRC0R::INPUT_6_SELECTS_PIN => 6,
            SRC0R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC0_R = crate::FR<u8, SRC0R>;
impl SRC0_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC0R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC0W {
    #[doc = "Input 0. Selects the output of pin interrupt select register 0 as the source to bit slice 0."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects the output of pin interrupt select register 1 as the source to bit slice 0."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects the output of pin interrupt select register 2 as the source to bit slice 0."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects the output of pin interrupt select register 3 as the source to bit slice 0."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects the output of pin interrupt select register 4 as the source to bit slice 0."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects the output of pin interrupt select register 5 as the source to bit slice 0."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects the output of pin interrupt select register 6 as the source to bit slice 0."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects the output of pin interrupt select register 7 as the source to bit slice 0."]
    INPUT_7_SELECTS_PIN,
}
impl SRC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC0W::INPUT_0_SELECTS_PIN => 0,
            SRC0W::INPUT_1_SELECTS_PIN => 1,
            SRC0W::INPUT_2_SELECTS_PIN => 2,
            SRC0W::INPUT_3_SELECTS_PIN => 3,
            SRC0W::INPUT_4_SELECTS_PIN => 4,
            SRC0W::INPUT_5_SELECTS_PIN => 5,
            SRC0W::INPUT_6_SELECTS_PIN => 6,
            SRC0W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects the output of pin interrupt select register 0 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects the output of pin interrupt select register 1 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects the output of pin interrupt select register 2 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects the output of pin interrupt select register 3 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects the output of pin interrupt select register 4 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects the output of pin interrupt select register 5 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects the output of pin interrupt select register 6 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects the output of pin interrupt select register 7 as the source to bit slice 0."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC0W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `SRC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC1R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 1."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 1."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 1."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 1."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 1."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 1."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 1."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 1."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC1R::INPUT_0_SELECTS_PIN => 0,
            SRC1R::INPUT_1_SELECTS_PIN => 1,
            SRC1R::INPUT_2_SELECTS_PIN => 2,
            SRC1R::INPUT_3_SELECTS_PIN => 3,
            SRC1R::INPUT_4_SELECTS_PIN => 4,
            SRC1R::INPUT_5_SELECTS_PIN => 5,
            SRC1R::INPUT_6_SELECTS_PIN => 6,
            SRC1R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC1_R = crate::FR<u8, SRC1R>;
impl SRC1_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC1R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC1W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 1."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 1."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 1."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 1."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 1."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 1."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 1."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 1."]
    INPUT_7_SELECTS_PIN,
}
impl SRC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC1W::INPUT_0_SELECTS_PIN => 0,
            SRC1W::INPUT_1_SELECTS_PIN => 1,
            SRC1W::INPUT_2_SELECTS_PIN => 2,
            SRC1W::INPUT_3_SELECTS_PIN => 3,
            SRC1W::INPUT_4_SELECTS_PIN => 4,
            SRC1W::INPUT_5_SELECTS_PIN => 5,
            SRC1W::INPUT_6_SELECTS_PIN => 6,
            SRC1W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 1."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC1W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `SRC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC2R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 2."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 2."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 2."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 2."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 2."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 2."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 2."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 2."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC2R::INPUT_0_SELECTS_PIN => 0,
            SRC2R::INPUT_1_SELECTS_PIN => 1,
            SRC2R::INPUT_2_SELECTS_PIN => 2,
            SRC2R::INPUT_3_SELECTS_PIN => 3,
            SRC2R::INPUT_4_SELECTS_PIN => 4,
            SRC2R::INPUT_5_SELECTS_PIN => 5,
            SRC2R::INPUT_6_SELECTS_PIN => 6,
            SRC2R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC2_R = crate::FR<u8, SRC2R>;
impl SRC2_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC2R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC2W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 2."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 2."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 2."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 2."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 2."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 2."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 2."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 2."]
    INPUT_7_SELECTS_PIN,
}
impl SRC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC2W::INPUT_0_SELECTS_PIN => 0,
            SRC2W::INPUT_1_SELECTS_PIN => 1,
            SRC2W::INPUT_2_SELECTS_PIN => 2,
            SRC2W::INPUT_3_SELECTS_PIN => 3,
            SRC2W::INPUT_4_SELECTS_PIN => 4,
            SRC2W::INPUT_5_SELECTS_PIN => 5,
            SRC2W::INPUT_6_SELECTS_PIN => 6,
            SRC2W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC2W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 2."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC2W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `SRC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC3R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 3."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 3."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 3."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 3."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 3."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 3."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 3."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 3."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC3R::INPUT_0_SELECTS_PIN => 0,
            SRC3R::INPUT_1_SELECTS_PIN => 1,
            SRC3R::INPUT_2_SELECTS_PIN => 2,
            SRC3R::INPUT_3_SELECTS_PIN => 3,
            SRC3R::INPUT_4_SELECTS_PIN => 4,
            SRC3R::INPUT_5_SELECTS_PIN => 5,
            SRC3R::INPUT_6_SELECTS_PIN => 6,
            SRC3R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC3_R = crate::FR<u8, SRC3R>;
impl SRC3_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC3R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC3W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 3."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 3."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 3."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 3."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 3."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 3."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 3."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 3."]
    INPUT_7_SELECTS_PIN,
}
impl SRC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC3W::INPUT_0_SELECTS_PIN => 0,
            SRC3W::INPUT_1_SELECTS_PIN => 1,
            SRC3W::INPUT_2_SELECTS_PIN => 2,
            SRC3W::INPUT_3_SELECTS_PIN => 3,
            SRC3W::INPUT_4_SELECTS_PIN => 4,
            SRC3W::INPUT_5_SELECTS_PIN => 5,
            SRC3W::INPUT_6_SELECTS_PIN => 6,
            SRC3W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC3W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 3."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC3W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `SRC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC4R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 4."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 4."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 4."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 4."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 4."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 4."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 4."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 4."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC4R::INPUT_0_SELECTS_PIN => 0,
            SRC4R::INPUT_1_SELECTS_PIN => 1,
            SRC4R::INPUT_2_SELECTS_PIN => 2,
            SRC4R::INPUT_3_SELECTS_PIN => 3,
            SRC4R::INPUT_4_SELECTS_PIN => 4,
            SRC4R::INPUT_5_SELECTS_PIN => 5,
            SRC4R::INPUT_6_SELECTS_PIN => 6,
            SRC4R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC4_R = crate::FR<u8, SRC4R>;
impl SRC4_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC4R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC4W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 4."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 4."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 4."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 4."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 4."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 4."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 4."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 4."]
    INPUT_7_SELECTS_PIN,
}
impl SRC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC4W::INPUT_0_SELECTS_PIN => 0,
            SRC4W::INPUT_1_SELECTS_PIN => 1,
            SRC4W::INPUT_2_SELECTS_PIN => 2,
            SRC4W::INPUT_3_SELECTS_PIN => 3,
            SRC4W::INPUT_4_SELECTS_PIN => 4,
            SRC4W::INPUT_5_SELECTS_PIN => 5,
            SRC4W::INPUT_6_SELECTS_PIN => 6,
            SRC4W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC4W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 4."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC4W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `SRC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC5R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 5."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 5."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 5."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 5."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 5."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 5."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 5."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 5."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC5R::INPUT_0_SELECTS_PIN => 0,
            SRC5R::INPUT_1_SELECTS_PIN => 1,
            SRC5R::INPUT_2_SELECTS_PIN => 2,
            SRC5R::INPUT_3_SELECTS_PIN => 3,
            SRC5R::INPUT_4_SELECTS_PIN => 4,
            SRC5R::INPUT_5_SELECTS_PIN => 5,
            SRC5R::INPUT_6_SELECTS_PIN => 6,
            SRC5R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC5_R = crate::FR<u8, SRC5R>;
impl SRC5_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC5R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC5W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 5."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 5."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 5."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 5."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 5."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 5."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 5."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 5."]
    INPUT_7_SELECTS_PIN,
}
impl SRC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC5W::INPUT_0_SELECTS_PIN => 0,
            SRC5W::INPUT_1_SELECTS_PIN => 1,
            SRC5W::INPUT_2_SELECTS_PIN => 2,
            SRC5W::INPUT_3_SELECTS_PIN => 3,
            SRC5W::INPUT_4_SELECTS_PIN => 4,
            SRC5W::INPUT_5_SELECTS_PIN => 5,
            SRC5W::INPUT_6_SELECTS_PIN => 6,
            SRC5W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC5W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 5."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC5W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `SRC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC6R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 6."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 6."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 6."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 6."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 6."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 6."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 6."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 6."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC6R::INPUT_0_SELECTS_PIN => 0,
            SRC6R::INPUT_1_SELECTS_PIN => 1,
            SRC6R::INPUT_2_SELECTS_PIN => 2,
            SRC6R::INPUT_3_SELECTS_PIN => 3,
            SRC6R::INPUT_4_SELECTS_PIN => 4,
            SRC6R::INPUT_5_SELECTS_PIN => 5,
            SRC6R::INPUT_6_SELECTS_PIN => 6,
            SRC6R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC6_R = crate::FR<u8, SRC6R>;
impl SRC6_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC6R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC6W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 6."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 6."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 6."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 6."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 6."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 6."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 6."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 6."]
    INPUT_7_SELECTS_PIN,
}
impl SRC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC6W::INPUT_0_SELECTS_PIN => 0,
            SRC6W::INPUT_1_SELECTS_PIN => 1,
            SRC6W::INPUT_2_SELECTS_PIN => 2,
            SRC6W::INPUT_3_SELECTS_PIN => 3,
            SRC6W::INPUT_4_SELECTS_PIN => 4,
            SRC6W::INPUT_5_SELECTS_PIN => 5,
            SRC6W::INPUT_6_SELECTS_PIN => 6,
            SRC6W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC6W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 6."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC6W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `SRC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC7R {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 7."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 7."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 7."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 7."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 7."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 7."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 7."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 7."]
    INPUT_7_SELECTS_PIN,
}
impl crate::ToBits<u8> for SRC7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRC7R::INPUT_0_SELECTS_PIN => 0,
            SRC7R::INPUT_1_SELECTS_PIN => 1,
            SRC7R::INPUT_2_SELECTS_PIN => 2,
            SRC7R::INPUT_3_SELECTS_PIN => 3,
            SRC7R::INPUT_4_SELECTS_PIN => 4,
            SRC7R::INPUT_5_SELECTS_PIN => 5,
            SRC7R::INPUT_6_SELECTS_PIN => 6,
            SRC7R::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRC7_R = crate::FR<u8, SRC7R>;
impl SRC7_R {
    #[doc = "Checks if the value of the field is `INPUT_0_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_0_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_0_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_1_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_1_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_1_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_2_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_2_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_2_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_3_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_3_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_3_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_4_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_4_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_4_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_5_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_5_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_5_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_6_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_6_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_6_SELECTS_PIN
    }
    #[doc = "Checks if the value of the field is `INPUT_7_SELECTS_PIN`"]
    #[inline(always)]
    pub fn is_input_7_selects_pin(&self) -> bool {
        *self == SRC7R::INPUT_7_SELECTS_PIN
    }
}
#[doc = "Values that can be written to the field `SRC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC7W {
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 7."]
    INPUT_0_SELECTS_PIN,
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 7."]
    INPUT_1_SELECTS_PIN,
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 7."]
    INPUT_2_SELECTS_PIN,
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 7."]
    INPUT_3_SELECTS_PIN,
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 7."]
    INPUT_4_SELECTS_PIN,
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 7."]
    INPUT_5_SELECTS_PIN,
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 7."]
    INPUT_6_SELECTS_PIN,
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 7."]
    INPUT_7_SELECTS_PIN,
}
impl SRC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC7W::INPUT_0_SELECTS_PIN => 0,
            SRC7W::INPUT_1_SELECTS_PIN => 1,
            SRC7W::INPUT_2_SELECTS_PIN => 2,
            SRC7W::INPUT_3_SELECTS_PIN => 3,
            SRC7W::INPUT_4_SELECTS_PIN => 4,
            SRC7W::INPUT_5_SELECTS_PIN => 5,
            SRC7W::INPUT_6_SELECTS_PIN => 6,
            SRC7W::INPUT_7_SELECTS_PIN => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRC7W<'a> {
    w: &'a mut W,
}
impl<'a> _SRC7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input 0. Selects pin interrupt input 0 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_0_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_0_SELECTS_PIN)
    }
    #[doc = "Input 1. Selects pin interrupt input 1 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_1_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_1_SELECTS_PIN)
    }
    #[doc = "Input 2. Selects pin interrupt input 2 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_2_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_2_SELECTS_PIN)
    }
    #[doc = "Input 3. Selects pin interrupt input 3 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_3_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_3_SELECTS_PIN)
    }
    #[doc = "Input 4. Selects pin interrupt input 4 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_4_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_4_SELECTS_PIN)
    }
    #[doc = "Input 5. Selects pin interrupt input 5 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_5_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_5_SELECTS_PIN)
    }
    #[doc = "Input 6. Selects pin interrupt input 6 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_6_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_6_SELECTS_PIN)
    }
    #[doc = "Input 7. Selects pin interrupt input 7 as the source to bit slice 7."]
    #[inline(always)]
    pub fn input_7_selects_pin(self) -> &'a mut W {
        self.variant(SRC7W::INPUT_7_SELECTS_PIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    pub fn src0(&self) -> SRC0_R {
        SRC0_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    pub fn src1(&self) -> SRC1_R {
        SRC1_R::new(((self.bits() >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    pub fn src2(&self) -> SRC2_R {
        SRC2_R::new(((self.bits() >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    pub fn src3(&self) -> SRC3_R {
        SRC3_R::new(((self.bits() >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    pub fn src4(&self) -> SRC4_R {
        SRC4_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    pub fn src5(&self) -> SRC5_R {
        SRC5_R::new(((self.bits() >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    pub fn src6(&self) -> SRC6_R {
        SRC6_R::new(((self.bits() >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    pub fn src7(&self) -> SRC7_R {
        SRC7_R::new(((self.bits() >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:10 - Selects the input source for bit slice 0"]
    #[inline(always)]
    pub fn src0(&mut self) -> _SRC0W {
        _SRC0W { w: self }
    }
    #[doc = "Bits 11:13 - Selects the input source for bit slice 1"]
    #[inline(always)]
    pub fn src1(&mut self) -> _SRC1W {
        _SRC1W { w: self }
    }
    #[doc = "Bits 14:16 - Selects the input source for bit slice 2"]
    #[inline(always)]
    pub fn src2(&mut self) -> _SRC2W {
        _SRC2W { w: self }
    }
    #[doc = "Bits 17:19 - Selects the input source for bit slice 3"]
    #[inline(always)]
    pub fn src3(&mut self) -> _SRC3W {
        _SRC3W { w: self }
    }
    #[doc = "Bits 20:22 - Selects the input source for bit slice 4"]
    #[inline(always)]
    pub fn src4(&mut self) -> _SRC4W {
        _SRC4W { w: self }
    }
    #[doc = "Bits 23:25 - Selects the input source for bit slice 5"]
    #[inline(always)]
    pub fn src5(&mut self) -> _SRC5W {
        _SRC5W { w: self }
    }
    #[doc = "Bits 26:28 - Selects the input source for bit slice 6"]
    #[inline(always)]
    pub fn src6(&mut self) -> _SRC6W {
        _SRC6W { w: self }
    }
    #[doc = "Bits 29:31 - Selects the input source for bit slice 7"]
    #[inline(always)]
    pub fn src7(&mut self) -> _SRC7W {
        _SRC7W { w: self }
    }
}
