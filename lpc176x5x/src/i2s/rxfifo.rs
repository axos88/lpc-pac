#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXFIFO {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2SRXFIFO_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2srxfifo(&self) -> I2SRXFIFO_R {
        I2SRXFIFO_R::new((self.bits() & 0xffff_ffff) as u32)
    }
}
