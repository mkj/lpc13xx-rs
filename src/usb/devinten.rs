#[doc = "Reader of register DEVINTEN"]
pub type R = crate::R<u32, super::DEVINTEN>;
#[doc = "Writer for register DEVINTEN"]
pub type W = crate::W<u32, super::DEVINTEN>;
#[doc = "Register DEVINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAME_EN`"]
pub type FRAME_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_EN`"]
pub struct FRAME_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_EN_W<'a> {
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
#[doc = "Reader of field `EP0_EN`"]
pub type EP0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_EN`"]
pub struct EP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_EN_W<'a> {
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
#[doc = "Reader of field `EP1_EN`"]
pub type EP1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_EN`"]
pub struct EP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_EN_W<'a> {
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
#[doc = "Reader of field `EP2_EN`"]
pub type EP2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_EN`"]
pub struct EP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EP3_EN`"]
pub type EP3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_EN`"]
pub struct EP3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_EN_W<'a> {
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
#[doc = "Reader of field `EP4_EN`"]
pub type EP4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_EN`"]
pub struct EP4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EP5_EN`"]
pub type EP5_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_EN`"]
pub struct EP5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EP6_EN`"]
pub type EP6_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_EN`"]
pub struct EP6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EP7_EN`"]
pub type EP7_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_EN`"]
pub struct EP7_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_EN_W<'a> {
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
#[doc = "Reader of field `DEV_STAT_EN`"]
pub type DEV_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_STAT_EN`"]
pub struct DEV_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_STAT_EN_W<'a> {
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
#[doc = "Reader of field `CC_EMPTY_EN`"]
pub type CC_EMPTY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC_EMPTY_EN`"]
pub struct CC_EMPTY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_EMPTY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CD_FULL_EN`"]
pub type CD_FULL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CD_FULL_EN`"]
pub struct CD_FULL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_FULL_EN_W<'a> {
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
#[doc = "Reader of field `RXENDPKT_EN`"]
pub type RXENDPKT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENDPKT_EN`"]
pub struct RXENDPKT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDPKT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXENDPKT_EN`"]
pub type TXENDPKT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENDPKT_EN`"]
pub struct TXENDPKT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDPKT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn frame_en(&self) -> FRAME_EN_R {
        FRAME_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep0_en(&self) -> EP0_EN_R {
        EP0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep1_en(&self) -> EP1_EN_R {
        EP1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep2_en(&self) -> EP2_EN_R {
        EP2_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep3_en(&self) -> EP3_EN_R {
        EP3_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep4_en(&self) -> EP4_EN_R {
        EP4_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5_en(&self) -> EP5_EN_R {
        EP5_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep6_en(&self) -> EP6_EN_R {
        EP6_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep7_en(&self) -> EP7_EN_R {
        EP7_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn dev_stat_en(&self) -> DEV_STAT_EN_R {
        DEV_STAT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cc_empty_en(&self) -> CC_EMPTY_EN_R {
        CC_EMPTY_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cd_full_en(&self) -> CD_FULL_EN_R {
        CD_FULL_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn rxendpkt_en(&self) -> RXENDPKT_EN_R {
        RXENDPKT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn txendpkt_en(&self) -> TXENDPKT_EN_R {
        TXENDPKT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn frame_en(&mut self) -> FRAME_EN_W {
        FRAME_EN_W { w: self }
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep0_en(&mut self) -> EP0_EN_W {
        EP0_EN_W { w: self }
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep1_en(&mut self) -> EP1_EN_W {
        EP1_EN_W { w: self }
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep2_en(&mut self) -> EP2_EN_W {
        EP2_EN_W { w: self }
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep3_en(&mut self) -> EP3_EN_W {
        EP3_EN_W { w: self }
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep4_en(&mut self) -> EP4_EN_W {
        EP4_EN_W { w: self }
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5_en(&mut self) -> EP5_EN_W {
        EP5_EN_W { w: self }
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep6_en(&mut self) -> EP6_EN_W {
        EP6_EN_W { w: self }
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep7_en(&mut self) -> EP7_EN_W {
        EP7_EN_W { w: self }
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn dev_stat_en(&mut self) -> DEV_STAT_EN_W {
        DEV_STAT_EN_W { w: self }
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cc_empty_en(&mut self) -> CC_EMPTY_EN_W {
        CC_EMPTY_EN_W { w: self }
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cd_full_en(&mut self) -> CD_FULL_EN_W {
        CD_FULL_EN_W { w: self }
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn rxendpkt_en(&mut self) -> RXENDPKT_EN_W {
        RXENDPKT_EN_W { w: self }
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn txendpkt_en(&mut self) -> TXENDPKT_EN_W {
        TXENDPKT_EN_W { w: self }
    }
}
