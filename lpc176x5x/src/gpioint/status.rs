#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `P0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0INTR {
    #[doc = "No pending interrupts on Port 0."]
    NO_PENDING_INTERRUPT,
    #[doc = "At least one pending interrupt on Port 0."]
    AT_LEAST_ONE_PENDING,
}
impl crate::ToBits<bool> for P0INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P0INTR::NO_PENDING_INTERRUPT => false,
            P0INTR::AT_LEAST_ONE_PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0INT_R = crate::FR<bool, P0INTR>;
impl P0INT_R {
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P0INTR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P0INTR::AT_LEAST_ONE_PENDING
    }
}
#[doc = "Possible values of the field `P2INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2INTR {
    #[doc = "No pending interrupts on Port 2."]
    NO_PENDING_INTERRUPT,
    #[doc = "At least one pending interrupt on Port 2."]
    AT_LEAST_ONE_PENDING,
}
impl crate::ToBits<bool> for P2INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            P2INTR::NO_PENDING_INTERRUPT => false,
            P2INTR::AT_LEAST_ONE_PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type P2INT_R = crate::FR<bool, P2INTR>;
impl P2INT_R {
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P2INTR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P2INTR::AT_LEAST_ONE_PENDING
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port 0 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
