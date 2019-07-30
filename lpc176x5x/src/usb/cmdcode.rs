#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMDCODE {
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
#[doc = "Values that can be written to the field `CMD_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_PHASEW {
    #[doc = "Read"]
    READ,
    #[doc = "Write"]
    WRITE,
    #[doc = "Command"]
    COMMAND,
}
impl CMD_PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_PHASEW::READ => 2,
            CMD_PHASEW::WRITE => 1,
            CMD_PHASEW::COMMAND => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CMD_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_PHASEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_PHASEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_PHASEW::READ)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_PHASEW::WRITE)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut W {
        self.variant(CMD_PHASEW::COMMAND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _CMD_CODE_WDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CODE_WDATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline(always)]
    pub fn cmd_phase(&mut self) -> _CMD_PHASEW {
        _CMD_PHASEW { w: self }
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    pub fn cmd_code_wdata(&mut self) -> _CMD_CODE_WDATAW {
        _CMD_CODE_WDATAW { w: self }
    }
}
