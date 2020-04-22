#[doc = "Reader of register DCD_LOC"]
pub type R = crate::R<u32, super::DCD_LOC>;
#[doc = "Writer for register DCD_LOC"]
pub type W = crate::W<u32, super::DCD_LOC>;
#[doc = "Register DCD_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::DCD_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCDLOC_A {
    #[doc = "0: Selects DCD function in pin location PIO2_2/DCD/MISO1."]
    SELECTS_DCD_FUNCTION = 0,
    #[doc = "1: Selects  DCD function in pin location PIO3_2/DCD."]
    SELECTS_DCD_FUNCTIO = 1,
    #[doc = "2: Reserved."]
    RESERVED_ = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<DCDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCDLOC`"]
pub type DCDLOC_R = crate::R<u8, DCDLOC_A>;
impl DCDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDLOC_A {
        match self.bits {
            0 => DCDLOC_A::SELECTS_DCD_FUNCTION,
            1 => DCDLOC_A::SELECTS_DCD_FUNCTIO,
            2 => DCDLOC_A::RESERVED_,
            3 => DCDLOC_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_DCD_FUNCTION`"]
    #[inline(always)]
    pub fn is_selects_dcd_function(&self) -> bool {
        *self == DCDLOC_A::SELECTS_DCD_FUNCTION
    }
    #[doc = "Checks if the value of the field is `SELECTS_DCD_FUNCTIO`"]
    #[inline(always)]
    pub fn is_selects_dcd_functio(&self) -> bool {
        *self == DCDLOC_A::SELECTS_DCD_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == DCDLOC_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == DCDLOC_A::RESERVED_
    }
}
#[doc = "Write proxy for field `DCDLOC`"]
pub struct DCDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDLOC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects DCD function in pin location PIO2_2/DCD/MISO1."]
    #[inline(always)]
    pub fn selects_dcd_function(self) -> &'a mut W {
        self.variant(DCDLOC_A::SELECTS_DCD_FUNCTION)
    }
    #[doc = "Selects DCD function in pin location PIO3_2/DCD."]
    #[inline(always)]
    pub fn selects_dcd_functio(self) -> &'a mut W {
        self.variant(DCDLOC_A::SELECTS_DCD_FUNCTIO)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(DCDLOC_A::RESERVED_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(DCDLOC_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dcdloc(&self) -> DCDLOC_R {
        DCDLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for DCD pin (this register is only used for parts LPC1311/01 and LPC1313/01)."]
    #[inline(always)]
    pub fn dcdloc(&mut self) -> DCDLOC_W {
        DCDLOC_W { w: self }
    }
}
