#[doc = "Reader of register IIR"]
pub type R = crate::R<u32, super::IIR>;
#[doc = "Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt\r\nis pending."]
    INT = 0,
    #[doc = "1: No\r\ninterrupt is pending."]
    NOINT = 1,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTSTATUS`"]
pub type INTSTATUS_R = crate::R<bool, INTSTATUS_A>;
impl INTSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::INT,
            true => INTSTATUS_A::NOINT,
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == INTSTATUS_A::INT
    }
    #[doc = "Checks if the value of the field is `NOINT`"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == INTSTATUS_A::NOINT
    }
}
#[doc = "Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART Rx FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTID_A {
    #[doc = "3: 1   - Receive Line Status\r\n(RLS)."]
    RECEIVE = 3,
    #[doc = "2: 2a\r\n- Receive Data Available (RDA)."]
    RDA = 2,
    #[doc = "6: 2b - Character Time-out Indicator\r\n(CTI)."]
    CTIMEOUT = 6,
    #[doc = "1: 3\r\n  - THRE Interrupt."]
    THRE = 1,
    #[doc = "0: 4   - Modem interrupt."]
    MODEM = 0,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u8, INTID_A>;
impl INTID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTID_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(INTID_A::RECEIVE),
            2 => Val(INTID_A::RDA),
            6 => Val(INTID_A::CTIMEOUT),
            1 => Val(INTID_A::THRE),
            0 => Val(INTID_A::MODEM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == INTID_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `RDA`"]
    #[inline(always)]
    pub fn is_rda(&self) -> bool {
        *self == INTID_A::RDA
    }
    #[doc = "Checks if the value of the field is `CTIMEOUT`"]
    #[inline(always)]
    pub fn is_ctimeout(&self) -> bool {
        *self == INTID_A::CTIMEOUT
    }
    #[doc = "Checks if the value of the field is `THRE`"]
    #[inline(always)]
    pub fn is_thre(&self) -> bool {
        *self == INTID_A::THRE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == INTID_A::MODEM
    }
}
#[doc = "Reader of field `FIFOEN`"]
pub type FIFOEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ABEOINT`"]
pub type ABEOINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABTOINT`"]
pub type ABTOINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART Rx FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - These bits are equivalent to FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
