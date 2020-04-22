#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_AW {
    #[doc = "0: UART FIFOs are disabled. Must not be used in the application."]
    DISABLED = 0,
    #[doc = "1: Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs."]
    ENABLED = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FIFOEN`"]
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::DISABLED)
    }
    #[doc = "Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::ENABLED)
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
#[doc = "RX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOR_AW {
    #[doc = "0: No impact on either of UART FIFOs."]
    NOACTION = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<RXFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXFIFOR`"]
pub struct RXFIFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFOR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact on either of UART FIFOs."]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(RXFIFOR_AW::NOACTION)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFOR_AW::CLEAR)
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
#[doc = "TX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOR_AW {
    #[doc = "0: No impact on either of UART FIFOs."]
    NOACTION = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<TXFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXFIFOR`"]
pub struct TXFIFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFOR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact on either of UART FIFOs."]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(TXFIFOR_AW::NOACTION)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFOR_AW::CLEAR)
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
#[doc = "RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXTLVL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_ = 3,
}
impl From<RXTLVL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTLVL_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `RXTLVL`"]
pub struct RXTLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTLVL_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_3_14_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    pub fn rxfifor(&mut self) -> RXFIFOR_W {
        RXFIFOR_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    pub fn txfifor(&mut self) -> TXFIFOR_W {
        TXFIFOR_W { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtlvl(&mut self) -> RXTLVL_W {
        RXTLVL_W { w: self }
    }
}
