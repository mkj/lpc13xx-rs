#[doc = "Reader of register PRESETCTRL"]
pub type R = crate::R<u32, super::PRESETCTRL>;
#[doc = "Writer for register PRESETCTRL"]
pub type W = crate::W<u32, super::PRESETCTRL>;
#[doc = "Register PRESETCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SSP0 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_N_A {
    #[doc = "0: Reset SSP0."]
    RESET_SSP0_ = 0,
    #[doc = "1: De-assert SSP0 reset."]
    DE_ASSERT_SSP0_RESET = 1,
}
impl From<SSP0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSP0_RST_N`"]
pub type SSP0_RST_N_R = crate::R<bool, SSP0_RST_N_A>;
impl SSP0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_RST_N_A {
        match self.bits {
            false => SSP0_RST_N_A::RESET_SSP0_,
            true => SSP0_RST_N_A::DE_ASSERT_SSP0_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_SSP0_`"]
    #[inline(always)]
    pub fn is_reset_ssp0_(&self) -> bool {
        *self == SSP0_RST_N_A::RESET_SSP0_
    }
    #[doc = "Checks if the value of the field is `DE_ASSERT_SSP0_RESET`"]
    #[inline(always)]
    pub fn is_de_assert_ssp0_reset(&self) -> bool {
        *self == SSP0_RST_N_A::DE_ASSERT_SSP0_RESET
    }
}
#[doc = "Write proxy for field `SSP0_RST_N`"]
pub struct SSP0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset SSP0."]
    #[inline(always)]
    pub fn reset_ssp0_(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::RESET_SSP0_)
    }
    #[doc = "De-assert SSP0 reset."]
    #[inline(always)]
    pub fn de_assert_ssp0_reset(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::DE_ASSERT_SSP0_RESET)
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
#[doc = "I2C reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_N_A {
    #[doc = "0: Reset I2C."]
    RESET_I2C_ = 0,
    #[doc = "1: De-asset I2C reset."]
    DE_ASSET_I2C_RESET_ = 1,
}
impl From<I2C_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_RST_N`"]
pub type I2C_RST_N_R = crate::R<bool, I2C_RST_N_A>;
impl I2C_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_RST_N_A {
        match self.bits {
            false => I2C_RST_N_A::RESET_I2C_,
            true => I2C_RST_N_A::DE_ASSET_I2C_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_I2C_`"]
    #[inline(always)]
    pub fn is_reset_i2c_(&self) -> bool {
        *self == I2C_RST_N_A::RESET_I2C_
    }
    #[doc = "Checks if the value of the field is `DE_ASSET_I2C_RESET_`"]
    #[inline(always)]
    pub fn is_de_asset_i2c_reset_(&self) -> bool {
        *self == I2C_RST_N_A::DE_ASSET_I2C_RESET_
    }
}
#[doc = "Write proxy for field `I2C_RST_N`"]
pub struct I2C_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset I2C."]
    #[inline(always)]
    pub fn reset_i2c_(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::RESET_I2C_)
    }
    #[doc = "De-asset I2C reset."]
    #[inline(always)]
    pub fn de_asset_i2c_reset_(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::DE_ASSET_I2C_RESET_)
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
#[doc = "SPISP1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_N_A {
    #[doc = "0: Reset the SPISP1."]
    RESET_THE_SPISP1_ = 0,
    #[doc = "1: De-assert SPISP1 reset."]
    DE_ASSERT_SPISP1_RES = 1,
}
impl From<SSP1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSP1_RST_N`"]
pub type SSP1_RST_N_R = crate::R<bool, SSP1_RST_N_A>;
impl SSP1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_RST_N_A {
        match self.bits {
            false => SSP1_RST_N_A::RESET_THE_SPISP1_,
            true => SSP1_RST_N_A::DE_ASSERT_SPISP1_RES,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_THE_SPISP1_`"]
    #[inline(always)]
    pub fn is_reset_the_spisp1_(&self) -> bool {
        *self == SSP1_RST_N_A::RESET_THE_SPISP1_
    }
    #[doc = "Checks if the value of the field is `DE_ASSERT_SPISP1_RES`"]
    #[inline(always)]
    pub fn is_de_assert_spisp1_res(&self) -> bool {
        *self == SSP1_RST_N_A::DE_ASSERT_SPISP1_RES
    }
}
#[doc = "Write proxy for field `SSP1_RST_N`"]
pub struct SSP1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the SPISP1."]
    #[inline(always)]
    pub fn reset_the_spisp1_(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::RESET_THE_SPISP1_)
    }
    #[doc = "De-assert SPISP1 reset."]
    #[inline(always)]
    pub fn de_assert_spisp1_res(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::DE_ASSERT_SPISP1_RES)
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
impl R {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_N_R {
        SSP0_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&self) -> I2C_RST_N_R {
        I2C_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPISP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_N_R {
        SSP1_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> SSP0_RST_N_W {
        SSP0_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&mut self) -> I2C_RST_N_W {
        I2C_RST_N_W { w: self }
    }
    #[doc = "Bit 2 - SPISP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> SSP1_RST_N_W {
        SSP1_RST_N_W { w: self }
    }
}
