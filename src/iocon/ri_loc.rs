#[doc = "Reader of register RI_LOC"]
pub type R = crate::R<u32, super::RI_LOC>;
#[doc = "Writer for register RI_LOC"]
pub type W = crate::W<u32, super::RI_LOC>;
#[doc = "Register RI_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::RI_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RILOC_A {
    #[doc = "0: Selects RI function in pin location PIO2_3/RI/MOSI1."]
    SELECTS_RI_FUNCTION_ = 0,
    #[doc = "1: Selects RI function in pin location PIO3_3/RI."]
    SELECTS_RI_FUNCTION_ = 1,
    #[doc = "2: Reserved."]
    RESERVED_ = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<RILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RILOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RILOC`"]
pub type RILOC_R = crate::R<u8, RILOC_A>;
impl RILOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RILOC_A {
        match self.bits {
            0 => RILOC_A::SELECTS_RI_FUNCTION_,
            1 => RILOC_A::SELECTS_RI_FUNCTION_,
            2 => RILOC_A::RESERVED_,
            3 => RILOC_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_RI_FUNCTION_`"]
    #[inline(always)]
    pub fn is_selects_ri_function_(&self) -> bool {
        *self == RILOC_A::SELECTS_RI_FUNCTION_
    }
    #[doc = "Checks if the value of the field is `SELECTS_RI_FUNCTION_`"]
    #[inline(always)]
    pub fn is_selects_ri_function_(&self) -> bool {
        *self == RILOC_A::SELECTS_RI_FUNCTION_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == RILOC_A::RESERVED_
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == RILOC_A::RESERVED_
    }
}
#[doc = "Write proxy for field `RILOC`"]
pub struct RILOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RILOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RILOC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects RI function in pin location PIO2_3/RI/MOSI1."]
    #[inline(always)]
    pub fn selects_ri_function_(self) -> &'a mut W {
        self.variant(RILOC_A::SELECTS_RI_FUNCTION_)
    }
    #[doc = "Selects RI function in pin location PIO3_3/RI."]
    #[inline(always)]
    pub fn selects_ri_function_(self) -> &'a mut W {
        self.variant(RILOC_A::SELECTS_RI_FUNCTION_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(RILOC_A::RESERVED_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(RILOC_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
    #[inline(always)]
    pub fn riloc(&self) -> RILOC_R {
        RILOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for RI pin (this register is only used for parts LPC1311/01 and LPC1313/01)"]
    #[inline(always)]
    pub fn riloc(&mut self) -> RILOC_W {
        RILOC_W { w: self }
    }
}
