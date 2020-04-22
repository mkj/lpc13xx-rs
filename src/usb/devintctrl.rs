#[doc = "Writer for register DEVINTCTRL"]
pub type W = crate::W<u32, super::DEVINTCTRL>;
#[doc = "Register DEVINTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVINTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FRAME_CLR`"]
pub struct FRAME_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP0_CLR`"]
pub struct EP0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP1_CLR`"]
pub struct EP1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP2_CLR`"]
pub struct EP2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP3_CLR`"]
pub struct EP3_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP4_CLR`"]
pub struct EP4_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP5_CLR`"]
pub struct EP5_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP6_CLR`"]
pub struct EP6_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_CLR_W<'a> {
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
#[doc = "Write proxy for field `EP7_CLR`"]
pub struct EP7_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_CLR_W<'a> {
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
#[doc = "Write proxy for field `DEV_STAT_CLR`"]
pub struct DEV_STAT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_STAT_CLR_W<'a> {
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
#[doc = "Write proxy for field `CC_EMPTY_CLR`"]
pub struct CC_EMPTY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_EMPTY_CLR_W<'a> {
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
#[doc = "Write proxy for field `CD_FULL_CLR`"]
pub struct CD_FULL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_FULL_CLR_W<'a> {
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
#[doc = "Write proxy for field `RXENDPKT_CLR`"]
pub struct RXENDPKT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDPKT_CLR_W<'a> {
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
#[doc = "Write proxy for field `TXENDPKT_CLR`"]
pub struct TXENDPKT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDPKT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn frame_clr(&mut self) -> FRAME_CLR_W {
        FRAME_CLR_W { w: self }
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep0_clr(&mut self) -> EP0_CLR_W {
        EP0_CLR_W { w: self }
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep1_clr(&mut self) -> EP1_CLR_W {
        EP1_CLR_W { w: self }
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep2_clr(&mut self) -> EP2_CLR_W {
        EP2_CLR_W { w: self }
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep3_clr(&mut self) -> EP3_CLR_W {
        EP3_CLR_W { w: self }
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep4_clr(&mut self) -> EP4_CLR_W {
        EP4_CLR_W { w: self }
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep5_clr(&mut self) -> EP5_CLR_W {
        EP5_CLR_W { w: self }
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep6_clr(&mut self) -> EP6_CLR_W {
        EP6_CLR_W { w: self }
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep7_clr(&mut self) -> EP7_CLR_W {
        EP7_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn dev_stat_clr(&mut self) -> DEV_STAT_CLR_W {
        DEV_STAT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn cc_empty_clr(&mut self) -> CC_EMPTY_CLR_W {
        CC_EMPTY_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn cd_full_clr(&mut self) -> CD_FULL_CLR_W {
        CD_FULL_CLR_W { w: self }
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn rxendpkt_clr(&mut self) -> RXENDPKT_CLR_W {
        RXENDPKT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn txendpkt_clr(&mut self) -> TXENDPKT_CLR_W {
        TXENDPKT_CLR_W { w: self }
    }
}
