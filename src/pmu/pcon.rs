#[doc = "Reader of register PCON"]
pub type R = crate::R<u32, super::PCON>;
#[doc = "Writer for register PCON"]
pub type W = crate::W<u32, super::PCON>;
#[doc = "Register PCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Deep power-down mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDEN_A {
    #[doc = "0: ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M3 core turned off)."]
    SLEEP_DEEPSLEEP = 0,
    #[doc = "1: ARM WFI will enter Deep-power down mode (ARM Cortex-M3 core powered-down)."]
    DEEPPOWERDOWN = 1,
}
impl From<DPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPDEN`"]
pub type DPDEN_R = crate::R<bool, DPDEN_A>;
impl DPDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDEN_A {
        match self.bits {
            false => DPDEN_A::SLEEP_DEEPSLEEP,
            true => DPDEN_A::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_sleep_deepsleep(&self) -> bool {
        *self == DPDEN_A::SLEEP_DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDEN_A::DEEPPOWERDOWN
    }
}
#[doc = "Write proxy for field `DPDEN`"]
pub struct DPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M3 core turned off)."]
    #[inline(always)]
    pub fn sleep_deepsleep(self) -> &'a mut W {
        self.variant(DPDEN_A::SLEEP_DEEPSLEEP)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M3 core powered-down)."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDEN_A::DEEPPOWERDOWN)
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
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAG_A {
    #[doc = "0: Read: No power-down mode entered. LPC13xx is in Run mode. Write: No effect."]
    NO_POWER_DOWN_ = 0,
    #[doc = "1: Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN = 1,
}
impl From<SLEEPFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEPFLAG`"]
pub type SLEEPFLAG_R = crate::R<bool, SLEEPFLAG_A>;
impl SLEEPFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPFLAG_A {
        match self.bits {
            false => SLEEPFLAG_A::NO_POWER_DOWN_,
            true => SLEEPFLAG_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POWER_DOWN_`"]
    #[inline(always)]
    pub fn is_no_power_down_(&self) -> bool {
        *self == SLEEPFLAG_A::NO_POWER_DOWN_
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAG_A::POWERDOWN
    }
}
#[doc = "Write proxy for field `SLEEPFLAG`"]
pub struct SLEEPFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: No power-down mode entered. LPC13xx is in Run mode. Write: No effect."]
    #[inline(always)]
    pub fn no_power_down_(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::NO_POWER_DOWN_)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::POWERDOWN)
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
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAG_A {
    #[doc = "0: Read: Deep power-down mode  not entered. Write: No effect."]
    NO_DEEPPOWERDOWN = 0,
    #[doc = "1: Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEPPOWERDOWN = 1,
}
impl From<DPDFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DPDFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPDFLAG`"]
pub type DPDFLAG_R = crate::R<bool, DPDFLAG_A>;
impl DPDFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDFLAG_A {
        match self.bits {
            false => DPDFLAG_A::NO_DEEPPOWERDOWN,
            true => DPDFLAG_A::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_no_deeppowerdown(&self) -> bool {
        *self == DPDFLAG_A::NO_DEEPPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDFLAG_A::DEEPPOWERDOWN
    }
}
#[doc = "Write proxy for field `DPDFLAG`"]
pub struct DPDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn no_deeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAG_A::NO_DEEPPOWERDOWN)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DEEPPOWERDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    pub fn dpden(&self) -> DPDEN_R {
        DPDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    pub fn dpden(&mut self) -> DPDEN_W {
        DPDEN_W { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&mut self) -> SLEEPFLAG_W {
        SLEEPFLAG_W { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W {
        DPDFLAG_W { w: self }
    }
}
