#[doc = "Writer for register STARTRSRP1CLR"]
pub type W = crate::W<u32, super::STARTRSRP1CLR>;
#[doc = "Register STARTRSRP1CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTRSRP1CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RSRPIO2_8`"]
pub struct RSRPIO2_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_8_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_9`"]
pub struct RSRPIO2_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_9_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_10`"]
pub struct RSRPIO2_10_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_10_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_11`"]
pub struct RSRPIO2_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_11_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO3_0`"]
pub struct RSRPIO3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO3_0_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO3_1`"]
pub struct RSRPIO3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO3_1_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO3_2`"]
pub struct RSRPIO3_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO3_2_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO3_3`"]
pub struct RSRPIO3_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO3_3_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_8(&mut self) -> RSRPIO2_8_W {
        RSRPIO2_8_W { w: self }
    }
    #[doc = "Bit 1 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_9(&mut self) -> RSRPIO2_9_W {
        RSRPIO2_9_W { w: self }
    }
    #[doc = "Bit 2 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_10(&mut self) -> RSRPIO2_10_W {
        RSRPIO2_10_W { w: self }
    }
    #[doc = "Bit 3 - Start signal reset for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_11(&mut self) -> RSRPIO2_11_W {
        RSRPIO2_11_W { w: self }
    }
    #[doc = "Bit 4 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_0(&mut self) -> RSRPIO3_0_W {
        RSRPIO3_0_W { w: self }
    }
    #[doc = "Bit 5 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_1(&mut self) -> RSRPIO3_1_W {
        RSRPIO3_1_W { w: self }
    }
    #[doc = "Bit 6 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_2(&mut self) -> RSRPIO3_2_W {
        RSRPIO3_2_W { w: self }
    }
    #[doc = "Bit 7 - Start signal reset for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Do nothing.. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio3_3(&mut self) -> RSRPIO3_3_W {
        RSRPIO3_3_W { w: self }
    }
}
