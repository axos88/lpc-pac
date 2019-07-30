#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TSV1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TBC_R = crate::FR<u16, u16>;
#[doc = r"Reader of the field"]
pub type TCC_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
