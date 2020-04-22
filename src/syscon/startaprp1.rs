#[doc = "Reader of register STARTAPRP1"]
pub type R = crate::R<u32, super::STARTAPRP1>;
#[doc = "Writer for register STARTAPRP1"]
pub type W = crate::W<u32, super::STARTAPRP1>;
#[doc = "Register STARTAPRP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTAPRP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APRPIO2_8`"]
pub type APRPIO2_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_8`"]
pub struct APRPIO2_8_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_8_W<'a> {
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
#[doc = "Reader of field `APRPIO2_9`"]
pub type APRPIO2_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_9`"]
pub struct APRPIO2_9_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_9_W<'a> {
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
#[doc = "Reader of field `APRPIO2_10`"]
pub type APRPIO2_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_10`"]
pub struct APRPIO2_10_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_10_W<'a> {
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
#[doc = "Reader of field `APRPIO2_11`"]
pub type APRPIO2_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_11`"]
pub struct APRPIO2_11_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_11_W<'a> {
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
#[doc = "Reader of field `APRPIO3_0`"]
pub type APRPIO3_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO3_0`"]
pub struct APRPIO3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO3_0_W<'a> {
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
#[doc = "Reader of field `APRPIO3_1`"]
pub type APRPIO3_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO3_1`"]
pub struct APRPIO3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO3_1_W<'a> {
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
#[doc = "Reader of field `APRPIO3_2`"]
pub type APRPIO3_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO3_2`"]
pub struct APRPIO3_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO3_2_W<'a> {
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
#[doc = "Reader of field `APRPIO3_3`"]
pub type APRPIO3_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO3_3`"]
pub struct APRPIO3_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO3_3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_8(&self) -> APRPIO2_8_R {
        APRPIO2_8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_9(&self) -> APRPIO2_9_R {
        APRPIO2_9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_10(&self) -> APRPIO2_10_R {
        APRPIO2_10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_11(&self) -> APRPIO2_11_R {
        APRPIO2_11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_0(&self) -> APRPIO3_0_R {
        APRPIO3_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_1(&self) -> APRPIO3_1_R {
        APRPIO3_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_2(&self) -> APRPIO3_2_R {
        APRPIO3_2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_3(&self) -> APRPIO3_3_R {
        APRPIO3_3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_8(&mut self) -> APRPIO2_8_W {
        APRPIO2_8_W { w: self }
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_9(&mut self) -> APRPIO2_9_W {
        APRPIO2_9_W { w: self }
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_10(&mut self) -> APRPIO2_10_W {
        APRPIO2_10_W { w: self }
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_11(&mut self) -> APRPIO2_11_W {
        APRPIO2_11_W { w: self }
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_0(&mut self) -> APRPIO3_0_W {
        APRPIO3_0_W { w: self }
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_1(&mut self) -> APRPIO3_1_W {
        APRPIO3_1_W { w: self }
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_2(&mut self) -> APRPIO3_2_W {
        APRPIO3_2_W { w: self }
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio3_3(&mut self) -> APRPIO3_3_W {
        APRPIO3_3_W { w: self }
    }
}
