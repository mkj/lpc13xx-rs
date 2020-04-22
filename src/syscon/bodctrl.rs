#[doc = "Reader of register BODCTRL"]
pub type R = crate::R<u32, super::BODCTRL>;
#[doc = "Writer for register BODCTRL"]
pub type W = crate::W<u32, super::BODCTRL>;
#[doc = "Register BODCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BODCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: The reset assertion threshold voltage is 1.49 V/1.46 V; the reset de-assertion threshold voltage is 1.64 V/1.63 V."]
    THE_RESET_ASSERTION_ = 0,
    #[doc = "1: The reset assertion threshold voltage is -/2.06 V; the reset de-assertion threshold voltage is -/2.15 V."]
    THE_RESET_ASSERTION_ = 1,
    #[doc = "2: The reset assertion threshold voltage is -/2.35 V; the reset de-assertion threshold voltage is -/2.43 V."]
    THE_RESET_ASSERTION_ = 2,
    #[doc = "3: The reset assertion threshold voltage is -/2.63 V; the reset de-assertion threshold voltage is -/2.71 V."]
    THE_RESET_ASSERTION_ = 3,
}
impl From<BODRSTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODRSTLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BODRSTLEV`"]
pub type BODRSTLEV_R = crate::R<u8, BODRSTLEV_A>;
impl BODRSTLEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTLEV_A {
        match self.bits {
            0 => BODRSTLEV_A::THE_RESET_ASSERTION_,
            1 => BODRSTLEV_A::THE_RESET_ASSERTION_,
            2 => BODRSTLEV_A::THE_RESET_ASSERTION_,
            3 => BODRSTLEV_A::THE_RESET_ASSERTION_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THE_RESET_ASSERTION_`"]
    #[inline(always)]
    pub fn is_the_reset_assertion_(&self) -> bool {
        *self == BODRSTLEV_A::THE_RESET_ASSERTION_
    }
    #[doc = "Checks if the value of the field is `THE_RESET_ASSERTION_`"]
    #[inline(always)]
    pub fn is_the_reset_assertion_(&self) -> bool {
        *self == BODRSTLEV_A::THE_RESET_ASSERTION_
    }
    #[doc = "Checks if the value of the field is `THE_RESET_ASSERTION_`"]
    #[inline(always)]
    pub fn is_the_reset_assertion_(&self) -> bool {
        *self == BODRSTLEV_A::THE_RESET_ASSERTION_
    }
    #[doc = "Checks if the value of the field is `THE_RESET_ASSERTION_`"]
    #[inline(always)]
    pub fn is_the_reset_assertion_(&self) -> bool {
        *self == BODRSTLEV_A::THE_RESET_ASSERTION_
    }
}
#[doc = "Write proxy for field `BODRSTLEV`"]
pub struct BODRSTLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTLEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The reset assertion threshold voltage is 1.49 V/1.46 V; the reset de-assertion threshold voltage is 1.64 V/1.63 V."]
    #[inline(always)]
    pub fn the_reset_assertion_(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::THE_RESET_ASSERTION_)
    }
    #[doc = "The reset assertion threshold voltage is -/2.06 V; the reset de-assertion threshold voltage is -/2.15 V."]
    #[inline(always)]
    pub fn the_reset_assertion_(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::THE_RESET_ASSERTION_)
    }
    #[doc = "The reset assertion threshold voltage is -/2.35 V; the reset de-assertion threshold voltage is -/2.43 V."]
    #[inline(always)]
    pub fn the_reset_assertion_(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::THE_RESET_ASSERTION_)
    }
    #[doc = "The reset assertion threshold voltage is -/2.63 V; the reset de-assertion threshold voltage is -/2.71 V."]
    #[inline(always)]
    pub fn the_reset_assertion_(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::THE_RESET_ASSERTION_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTVAL_A {
    #[doc = "0: The interrupt assertion threshold voltage is 1.69 V/1.65 V; the interrupt de-assertion threshold voltage is 1.84 V/1.8 V."]
    THE_INTERRUPT_ASSERT = 0,
    #[doc = "1: The interrupt assertion threshold voltage is 2.29 V/2.22 V; the interrupt de-assertion threshold voltage is 2.44 V/2.35 V."]
    THE_INTERRUPT_ASSERT = 1,
    #[doc = "2: The interrupt assertion threshold voltage is 2.59 V/ 2.52 V; the interrupt de-assertion threshold voltage is 2.74 V/2.66 V."]
    THE_INTERRUPT_ASSERT = 2,
    #[doc = "3: The interrupt assertion threshold voltage is 2.87 V/2.80 V; the interrupt de-assertion threshold voltage is 2.98 V/2.90 V."]
    THE_INTERRUPT_ASSERT = 3,
}
impl From<BODINTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BODINTVAL`"]
pub type BODINTVAL_R = crate::R<u8, BODINTVAL_A>;
impl BODINTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTVAL_A {
        match self.bits {
            0 => BODINTVAL_A::THE_INTERRUPT_ASSERT,
            1 => BODINTVAL_A::THE_INTERRUPT_ASSERT,
            2 => BODINTVAL_A::THE_INTERRUPT_ASSERT,
            3 => BODINTVAL_A::THE_INTERRUPT_ASSERT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THE_INTERRUPT_ASSERT`"]
    #[inline(always)]
    pub fn is_the_interrupt_assert(&self) -> bool {
        *self == BODINTVAL_A::THE_INTERRUPT_ASSERT
    }
    #[doc = "Checks if the value of the field is `THE_INTERRUPT_ASSERT`"]
    #[inline(always)]
    pub fn is_the_interrupt_assert(&self) -> bool {
        *self == BODINTVAL_A::THE_INTERRUPT_ASSERT
    }
    #[doc = "Checks if the value of the field is `THE_INTERRUPT_ASSERT`"]
    #[inline(always)]
    pub fn is_the_interrupt_assert(&self) -> bool {
        *self == BODINTVAL_A::THE_INTERRUPT_ASSERT
    }
    #[doc = "Checks if the value of the field is `THE_INTERRUPT_ASSERT`"]
    #[inline(always)]
    pub fn is_the_interrupt_assert(&self) -> bool {
        *self == BODINTVAL_A::THE_INTERRUPT_ASSERT
    }
}
#[doc = "Write proxy for field `BODINTVAL`"]
pub struct BODINTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTVAL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The interrupt assertion threshold voltage is 1.69 V/1.65 V; the interrupt de-assertion threshold voltage is 1.84 V/1.8 V."]
    #[inline(always)]
    pub fn the_interrupt_assert(self) -> &'a mut W {
        self.variant(BODINTVAL_A::THE_INTERRUPT_ASSERT)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.29 V/2.22 V; the interrupt de-assertion threshold voltage is 2.44 V/2.35 V."]
    #[inline(always)]
    pub fn the_interrupt_assert(self) -> &'a mut W {
        self.variant(BODINTVAL_A::THE_INTERRUPT_ASSERT)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.59 V/ 2.52 V; the interrupt de-assertion threshold voltage is 2.74 V/2.66 V."]
    #[inline(always)]
    pub fn the_interrupt_assert(self) -> &'a mut W {
        self.variant(BODINTVAL_A::THE_INTERRUPT_ASSERT)
    }
    #[doc = "The interrupt assertion threshold voltage is 2.87 V/2.80 V; the interrupt de-assertion threshold voltage is 2.98 V/2.90 V."]
    #[inline(always)]
    pub fn the_interrupt_assert(self) -> &'a mut W {
        self.variant(BODINTVAL_A::THE_INTERRUPT_ASSERT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE_RESET_FUNCTI = 0,
    #[doc = "1: Enable reset function."]
    ENABLE_RESET_FUNCTIO = 1,
}
impl From<BODRSTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BODRSTENA`"]
pub type BODRSTENA_R = crate::R<bool, BODRSTENA_A>;
impl BODRSTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENA_A {
        match self.bits {
            false => BODRSTENA_A::DISABLE_RESET_FUNCTI,
            true => BODRSTENA_A::ENABLE_RESET_FUNCTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_RESET_FUNCTI`"]
    #[inline(always)]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == BODRSTENA_A::DISABLE_RESET_FUNCTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_FUNCTIO`"]
    #[inline(always)]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == BODRSTENA_A::ENABLE_RESET_FUNCTIO
    }
}
#[doc = "Write proxy for field `BODRSTENA`"]
pub struct BODRSTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable_reset_functi(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE_RESET_FUNCTI)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable_reset_functio(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE_RESET_FUNCTIO)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodintval(&self) -> BODINTVAL_R {
        BODINTVAL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W {
        BODRSTLEV_W { w: self }
    }
    #[doc = "Bits 2:3 - BOD interrupt level. Trip values x/y refer to the LPC1300/LPC1300L series."]
    #[inline(always)]
    pub fn bodintval(&mut self) -> BODINTVAL_W {
        BODINTVAL_W { w: self }
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W {
        BODRSTENA_W { w: self }
    }
}
