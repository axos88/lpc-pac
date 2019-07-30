#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENR2 {
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
pub type P2_0ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_0ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0ERW<'a> {
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
pub type P2_1ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_1ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1ERW<'a> {
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
#[doc = r"Reader of the field"]
pub type P2_2ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_2ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2ERW<'a> {
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
#[doc = r"Reader of the field"]
pub type P2_3ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_3ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_4ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_4ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_5ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_5ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_6ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_6ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_7ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_7ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_8ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_8ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_9ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_9ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_10ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_10ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_11ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_11ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_12ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_12ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P2_13ER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _P2_13ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13ERW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&self) -> P2_0ER_R {
        P2_0ER_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&self) -> P2_1ER_R {
        P2_1ER_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&self) -> P2_2ER_R {
        P2_2ER_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&self) -> P2_3ER_R {
        P2_3ER_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&self) -> P2_4ER_R {
        P2_4ER_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&self) -> P2_5ER_R {
        P2_5ER_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&self) -> P2_6ER_R {
        P2_6ER_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&self) -> P2_7ER_R {
        P2_7ER_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&self) -> P2_8ER_R {
        P2_8ER_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&self) -> P2_9ER_R {
        P2_9ER_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&self) -> P2_10ER_R {
        P2_10ER_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&self) -> P2_11ER_R {
        P2_11ER_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&self) -> P2_12ER_R {
        P2_12ER_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&self) -> P2_13ER_R {
        P2_13ER_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&mut self) -> _P2_0ERW {
        _P2_0ERW { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&mut self) -> _P2_1ERW {
        _P2_1ERW { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&mut self) -> _P2_2ERW {
        _P2_2ERW { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&mut self) -> _P2_3ERW {
        _P2_3ERW { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&mut self) -> _P2_4ERW {
        _P2_4ERW { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&mut self) -> _P2_5ERW {
        _P2_5ERW { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&mut self) -> _P2_6ERW {
        _P2_6ERW { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&mut self) -> _P2_7ERW {
        _P2_7ERW { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&mut self) -> _P2_8ERW {
        _P2_8ERW { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&mut self) -> _P2_9ERW {
        _P2_9ERW { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&mut self) -> _P2_10ERW {
        _P2_10ERW { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&mut self) -> _P2_11ERW {
        _P2_11ERW { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&mut self) -> _P2_12ERW {
        _P2_12ERW { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&mut self) -> _P2_13ERW {
        _P2_13ERW { w: self }
    }
}
