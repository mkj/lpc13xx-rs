#[doc = "Reader of register SYSOSCCTRL"]
pub type R = crate::R<u32, super::SYSOSCCTRL>;
#[doc = "Writer for register SYSOSCCTRL"]
pub type W = crate::W<u32, super::SYSOSCCTRL>;
#[doc = "Register SYSOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bypass system oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Oscillator is not bypassed."]
    OSCILLATOR_IS_NOT_BY = 0,
    #[doc = "1: Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    BYPASS_ENABLED_PLL_ = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::OSCILLATOR_IS_NOT_BY,
            true => BYPASS_A::BYPASS_ENABLED_PLL_,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_IS_NOT_BY`"]
    #[inline(always)]
    pub fn is_oscillator_is_not_by(&self) -> bool {
        *self == BYPASS_A::OSCILLATOR_IS_NOT_BY
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED_PLL_`"]
    #[inline(always)]
    pub fn is_bypass_enabled_pll_(&self) -> bool {
        *self == BYPASS_A::BYPASS_ENABLED_PLL_
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn oscillator_is_not_by(self) -> &'a mut W {
        self.variant(BYPASS_A::OSCILLATOR_IS_NOT_BY)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    #[inline(always)]
    pub fn bypass_enabled_pll_(self) -> &'a mut W {
        self.variant(BYPASS_A::BYPASS_ENABLED_PLL_)
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
#[doc = "Determines frequency range for Low-power oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGE_A {
    #[doc = "0: 1 - 20 MHz frequency range."]
    _1__20_MHZ_FREQUENCY = 0,
    #[doc = "1: 15 - 25 MHz frequency range"]
    _15__25_MHZ_FREQUENC = 1,
}
impl From<FREQRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQRANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FREQRANGE`"]
pub type FREQRANGE_R = crate::R<bool, FREQRANGE_A>;
impl FREQRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQRANGE_A {
        match self.bits {
            false => FREQRANGE_A::_1__20_MHZ_FREQUENCY,
            true => FREQRANGE_A::_15__25_MHZ_FREQUENC,
        }
    }
    #[doc = "Checks if the value of the field is `_1__20_MHZ_FREQUENCY`"]
    #[inline(always)]
    pub fn is_1__20_mhz_frequency(&self) -> bool {
        *self == FREQRANGE_A::_1__20_MHZ_FREQUENCY
    }
    #[doc = "Checks if the value of the field is `_15__25_MHZ_FREQUENC`"]
    #[inline(always)]
    pub fn is_15__25_mhz_frequenc(&self) -> bool {
        *self == FREQRANGE_A::_15__25_MHZ_FREQUENC
    }
}
#[doc = "Write proxy for field `FREQRANGE`"]
pub struct FREQRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQRANGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn _1__20_mhz_frequency(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_1__20_MHZ_FREQUENCY)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn _15__25_mhz_frequenc(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_15__25_MHZ_FREQUENC)
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
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W {
        FREQRANGE_W { w: self }
    }
}
