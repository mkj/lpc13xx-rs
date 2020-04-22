#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRIE_A {
    #[doc = "0: Disable the RDA interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the RDA interrupt."]
    ENABLE = 1,
}
impl From<RBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBRIE`"]
pub type RBRIE_R = crate::R<bool, RBRIE_A>;
impl RBRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRIE_A {
        match self.bits {
            false => RBRIE_A::DISABLE,
            true => RBRIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RBRIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RBRIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `RBRIE`"]
pub struct RBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBRIE_A::DISABLE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBRIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIE_A {
    #[doc = "0: Disable the THRE interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the THRE interrupt."]
    ENABLE = 1,
}
impl From<THREIE_A> for bool {
    #[inline(always)]
    fn from(variant: THREIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `THREIE`"]
pub type THREIE_R = crate::R<bool, THREIE_A>;
impl THREIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREIE_A {
        match self.bits {
            false => THREIE_A::DISABLE,
            true => THREIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THREIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THREIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `THREIE`"]
pub struct THREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(THREIE_A::DISABLE)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(THREIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLIE_A {
    #[doc = "0: Disable the RX line status\r\ninterrupts."]
    DISABLE = 0,
    #[doc = "1: Enable\r\nthe RX line status interrupts."]
    ENABLE = 1,
}
impl From<RXLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXLIE`"]
pub type RXLIE_R = crate::R<bool, RXLIE_A>;
impl RXLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLIE_A {
        match self.bits {
            false => RXLIE_A::DISABLE,
            true => RXLIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXLIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXLIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `RXLIE`"]
pub struct RXLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXLIE_A::DISABLE)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXLIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTEN_A {
    #[doc = "0: Disable end of auto-baud\r\nInterrupt."]
    DISABLE = 0,
    #[doc = "1: Enable\r\nend of auto-baud Interrupt."]
    ENABLE = 1,
}
impl From<ABEOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABEOINTEN`"]
pub type ABEOINTEN_R = crate::R<bool, ABEOINTEN_A>;
impl ABEOINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTEN_A {
        match self.bits {
            false => ABEOINTEN_A::DISABLE,
            true => ABEOINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ABEOINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABEOINTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ABEOINTEN`"]
pub struct ABEOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABEOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::DISABLE)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTEN_A {
    #[doc = "0: Disable auto-baud time-out\r\nInterrupt."]
    DISABLE = 0,
    #[doc = "1: Enable\r\nauto-baud time-out Interrupt."]
    ENABLE = 1,
}
impl From<ABTOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABTOINTEN`"]
pub type ABTOINTEN_R = crate::R<bool, ABTOINTEN_A>;
impl ABTOINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTEN_A {
        match self.bits {
            false => ABTOINTEN_A::DISABLE,
            true => ABTOINTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ABTOINTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABTOINTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ABTOINTEN`"]
pub struct ABTOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::DISABLE)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxlie(&self) -> RXLIE_R {
        RXLIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable. Enables the Receive Data Available interrupt for UART. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W {
        RBRIE_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable. Enables the THRE interrupt for UART. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&mut self) -> THREIE_W {
        THREIE_W { w: self }
    }
    #[doc = "Bit 2 - Line Interrupt Enable. Enables the UART RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxlie(&mut self) -> RXLIE_W {
        RXLIE_W { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> ABEOINTEN_W {
        ABEOINTEN_W { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> ABTOINTEN_W {
        ABTOINTEN_W { w: self }
    }
}
