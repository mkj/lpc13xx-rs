#[doc = "Writer for register DEVFIQSEL"]
pub type W = crate::W<u32, super::DEVFIQSEL>;
#[doc = "Register DEVFIQSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVFIQSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This interrupt comes from a 1 KHz free running clock resynchronized on the incoming SoF tokens. This is to be used for isochronous packet transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_AW {
    #[doc = "0: FRAME interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: FRAME interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FRAME`"]
pub struct FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRAME interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(FRAME_AW::LOWPRIORITY)
    }
    #[doc = "FRAME interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(FRAME_AW::HIGHPRIORITY)
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
#[doc = "Interrupt routing for bulk out endpoints For logical endpoint 3 (physical endpoints 6 and 7) only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BULKOUT_AW {
    #[doc = "0: BULKOUT interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: BULKOUT interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<BULKOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: BULKOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BULKOUT`"]
pub struct BULKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BULKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BULKOUT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BULKOUT interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(BULKOUT_AW::LOWPRIORITY)
    }
    #[doc = "BULKOUT interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(BULKOUT_AW::HIGHPRIORITY)
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
#[doc = "Interrupt routing for bulk in endpoints For logical endpoint 3 (physical endpoints 6 and 7) only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BULKIN_AW {
    #[doc = "0: BULKIN interrupt will be routed to the low-priority interrupt line IRQ."]
    LOWPRIORITY = 0,
    #[doc = "1: BULKIN interrupt will be routed to the high-priority interrupt line FIQ."]
    HIGHPRIORITY = 1,
}
impl From<BULKIN_AW> for bool {
    #[inline(always)]
    fn from(variant: BULKIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BULKIN`"]
pub struct BULKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BULKIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BULKIN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BULKIN interrupt will be routed to the low-priority interrupt line IRQ."]
    #[inline(always)]
    pub fn lowpriority(self) -> &'a mut W {
        self.variant(BULKIN_AW::LOWPRIORITY)
    }
    #[doc = "BULKIN interrupt will be routed to the high-priority interrupt line FIQ."]
    #[inline(always)]
    pub fn highpriority(self) -> &'a mut W {
        self.variant(BULKIN_AW::HIGHPRIORITY)
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
impl W {
    #[doc = "Bit 0 - This interrupt comes from a 1 KHz free running clock resynchronized on the incoming SoF tokens. This is to be used for isochronous packet transfer."]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W {
        FRAME_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt routing for bulk out endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
    #[inline(always)]
    pub fn bulkout(&mut self) -> BULKOUT_W {
        BULKOUT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt routing for bulk in endpoints For logical endpoint 3 (physical endpoints 6 and 7) only."]
    #[inline(always)]
    pub fn bulkin(&mut self) -> BULKIN_W {
        BULKIN_W { w: self }
    }
}
