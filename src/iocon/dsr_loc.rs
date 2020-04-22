#[doc = "Reader of register DSR_LOC"]
pub type R = crate::R<u32, super::DSR_LOC>;
#[doc = "Writer for register DSR_LOC"]
pub type W = crate::W<u32, super::DSR_LOC>;
#[doc = "Register DSR_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::DSR_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSRLOC_A {
    #[doc = "0: Selects DSR function in pin location PIO2_1/DSR/SCK1."]
    SELECTS_DSR_FUNCTION = 0,
    #[doc = "1: Selects DSR function in pin location PIO3_1/DSR."]
    SELECTS_DSR_FUNCTION = 1,
    #[doc = "2: Reserved."]
    RESERVED_ = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<DSRLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSRLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSRLOC`"]
pub type DSRLOC_R = crate::R<u8, DSRLOC_A>;
impl DSRLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRLOC_A {
        match self.bits {
            0 => DSRLOC_A::SELECTS_DSR_FUNCTION,
            1 => DSRLOC_A::SELECTS_DSR_FUNCTION,
            2 => DSRLOC_A::RESERVED_,
            3 => DSRLOC_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_DSR_FUNCTION`"]
    #[inline(always)]
    pub fn is_selects_dsr_function(&self) -> bool {
        *self == DSRLOC_A::SELECTS_DSR_FUNCTION
    }
    #[doc = "Checks if the value of the field is `SELECTS_DSR_FUNCTION`"]
    #[inline(always)]
    pub fn is_selects_dsr_function(&self) -> bool {
        *self == DSRLOC_A::SELECTS_DSR_FUNCTION
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == DSRLOC_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == DSRLOC_A::RESERVED_
    }
}
#[doc = "Write proxy for field `DSRLOC`"]
pub struct DSRLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSRLOC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects DSR function in pin location PIO2_1/DSR/SCK1."]
    #[inline(always)]
    pub fn selects_dsr_function(self) -> &'a mut W {
        self.variant(DSRLOC_A::SELECTS_DSR_FUNCTION)
    }
    #[doc = "Selects DSR function in pin location PIO3_1/DSR."]
    #[inline(always)]
    pub fn selects_dsr_function(self) -> &'a mut W {
        self.variant(DSRLOC_A::SELECTS_DSR_FUNCTION)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(DSRLOC_A::RESERVED_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(DSRLOC_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dsrloc(&self) -> DSRLOC_R {
        DSRLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DSR0 pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dsrloc(&mut self) -> DSRLOC_W {
        DSRLOC_W { w: self }
    }
}
