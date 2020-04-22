#[doc = "Reader of register SCK0_LOC"]
pub type R = crate::R<u32, super::SCK0_LOC>;
#[doc = "Writer for register SCK0_LOC"]
pub type W = crate::W<u32, super::SCK0_LOC>;
#[doc = "Register SCK0_LOC `reset()`'s with value 0"]
impl crate::ResetValue for super::SCK0_LOC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin location for SCK0 pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCKLOC_A {
    #[doc = "0: Selects SCK0 function for pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 (see Table 121)."]
    SELECTS_SCK0_FUNCTIO = 0,
    #[doc = "1: Selects SCK0 function for pin PIO2_11/SCK0 (see Table 123"]
    SELECTS_SCK0_FUNCTIO = 1,
    #[doc = "2: Selects SCK0 function for pin PIO0_6/USB_CONNECT/SCK0 (see Table 114)."]
    SELECTS_SCK0_FUNCTIO = 2,
    #[doc = "3: Reserved."]
    RESERVED_ = 3,
}
impl From<SCKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCKLOC`"]
pub type SCKLOC_R = crate::R<u8, SCKLOC_A>;
impl SCKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKLOC_A {
        match self.bits {
            0 => SCKLOC_A::SELECTS_SCK0_FUNCTIO,
            1 => SCKLOC_A::SELECTS_SCK0_FUNCTIO,
            2 => SCKLOC_A::SELECTS_SCK0_FUNCTIO,
            3 => SCKLOC_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTIO`"]
    #[inline(always)]
    pub fn is_selects_sck0_functio(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTIO`"]
    #[inline(always)]
    pub fn is_selects_sck0_functio(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `SELECTS_SCK0_FUNCTIO`"]
    #[inline(always)]
    pub fn is_selects_sck0_functio(&self) -> bool {
        *self == SCKLOC_A::SELECTS_SCK0_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        *self == SCKLOC_A::RESERVED_
    }
}
#[doc = "Write proxy for field `SCKLOC`"]
pub struct SCKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKLOC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects SCK0 function for pin SWCLK/PIO0_10/SCK0/CT16B0_MAT2 (see Table 121)."]
    #[inline(always)]
    pub fn selects_sck0_functio(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTIO)
    }
    #[doc = "Selects SCK0 function for pin PIO2_11/SCK0 (see Table 123"]
    #[inline(always)]
    pub fn selects_sck0_functio(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTIO)
    }
    #[doc = "Selects SCK0 function for pin PIO0_6/USB_CONNECT/SCK0 (see Table 114)."]
    #[inline(always)]
    pub fn selects_sck0_functio(self) -> &'a mut W {
        self.variant(SCKLOC_A::SELECTS_SCK0_FUNCTIO)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(SCKLOC_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 pin."]
    #[inline(always)]
    pub fn sckloc(&self) -> SCKLOC_R {
        SCKLOC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects pin location for SCK0 pin."]
    #[inline(always)]
    pub fn sckloc(&mut self) -> SCKLOC_W {
        SCKLOC_W { w: self }
    }
}
