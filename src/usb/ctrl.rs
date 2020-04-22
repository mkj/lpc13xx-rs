#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_EN_A {
    #[doc = "0: Read mode is disabled."]
    READ_MODE_IS_DISABLE = 0,
    #[doc = "1: Read mode is enabled."]
    READ_MODE_IS_ENABLED = 1,
}
impl From<RD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RD_EN`"]
pub type RD_EN_R = crate::R<bool, RD_EN_A>;
impl RD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_EN_A {
        match self.bits {
            false => RD_EN_A::READ_MODE_IS_DISABLE,
            true => RD_EN_A::READ_MODE_IS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `READ_MODE_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_read_mode_is_disable(&self) -> bool {
        *self == RD_EN_A::READ_MODE_IS_DISABLE
    }
    #[doc = "Checks if the value of the field is `READ_MODE_IS_ENABLED`"]
    #[inline(always)]
    pub fn is_read_mode_is_enabled(&self) -> bool {
        *self == RD_EN_A::READ_MODE_IS_ENABLED
    }
}
#[doc = "Write proxy for field `RD_EN`"]
pub struct RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read mode is disabled."]
    #[inline(always)]
    pub fn read_mode_is_disable(self) -> &'a mut W {
        self.variant(RD_EN_A::READ_MODE_IS_DISABLE)
    }
    #[doc = "Read mode is enabled."]
    #[inline(always)]
    pub fn read_mode_is_enabled(self) -> &'a mut W {
        self.variant(RD_EN_A::READ_MODE_IS_ENABLED)
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
#[doc = "Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WR_EN_A {
    #[doc = "0: Write mode is disabled."]
    WRITE_MODE_IS_DISABL = 0,
    #[doc = "1: Write mode is enabled."]
    WRITE_MODE_IS_ENABLE = 1,
}
impl From<WR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WR_EN`"]
pub type WR_EN_R = crate::R<bool, WR_EN_A>;
impl WR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_EN_A {
        match self.bits {
            false => WR_EN_A::WRITE_MODE_IS_DISABL,
            true => WR_EN_A::WRITE_MODE_IS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_MODE_IS_DISABL`"]
    #[inline(always)]
    pub fn is_write_mode_is_disabl(&self) -> bool {
        *self == WR_EN_A::WRITE_MODE_IS_DISABL
    }
    #[doc = "Checks if the value of the field is `WRITE_MODE_IS_ENABLE`"]
    #[inline(always)]
    pub fn is_write_mode_is_enable(&self) -> bool {
        *self == WR_EN_A::WRITE_MODE_IS_ENABLE
    }
}
#[doc = "Write proxy for field `WR_EN`"]
pub struct WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write mode is disabled."]
    #[inline(always)]
    pub fn write_mode_is_disabl(self) -> &'a mut W {
        self.variant(WR_EN_A::WRITE_MODE_IS_DISABL)
    }
    #[doc = "Write mode is enabled."]
    #[inline(always)]
    pub fn write_mode_is_enable(self) -> &'a mut W {
        self.variant(WR_EN_A::WRITE_MODE_IS_ENABLE)
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
#[doc = "Reader of field `LOG_ENDPOINT`"]
pub type LOG_ENDPOINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOG_ENDPOINT`"]
pub struct LOG_ENDPOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOG_ENDPOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&self) -> LOG_ENDPOINT_R {
        LOG_ENDPOINT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&mut self) -> RD_EN_W {
        RD_EN_W { w: self }
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W {
        WR_EN_W { w: self }
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&mut self) -> LOG_ENDPOINT_W {
        LOG_ENDPOINT_W { w: self }
    }
}
