#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POSITION {
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
pub struct WRAPR {
    bits: bool,
}
impl WRAPR {
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
pub struct POINTERR {
    bits: u32,
}
impl POINTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _WRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _WRAPW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POINTERW<'a> {
    w: &'a mut W,
}
impl<'a> _POINTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline]
    pub fn wrap(&self) -> WRAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRAPR { bits }
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\] of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline]
    pub fn pointer(&self) -> POINTERR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        POINTERR { bits }
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
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline]
    pub fn wrap(&mut self) -> _WRAPW {
        _WRAPW { w: self }
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\] of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline]
    pub fn pointer(&mut self) -> _POINTERW {
        _POINTERW { w: self }
    }
}
