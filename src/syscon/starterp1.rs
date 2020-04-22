#[doc = "Reader of register STARTERP1"]
pub type R = crate::R<u32, super::STARTERP1>;
#[doc = "Writer for register STARTERP1"]
pub type W = crate::W<u32, super::STARTERP1>;
#[doc = "Register STARTERP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERPIO2_8`"]
pub type ERPIO2_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_8`"]
pub struct ERPIO2_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_8_W<'a> {
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
#[doc = "Reader of field `ERPIO2_9`"]
pub type ERPIO2_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_9`"]
pub struct ERPIO2_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_9_W<'a> {
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
#[doc = "Reader of field `ERPIO2_10`"]
pub type ERPIO2_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_10`"]
pub struct ERPIO2_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_10_W<'a> {
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
#[doc = "Reader of field `ERPIO2_11`"]
pub type ERPIO2_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_11`"]
pub struct ERPIO2_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_11_W<'a> {
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
#[doc = "Reader of field `ERPIO3_0`"]
pub type ERPIO3_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO3_0`"]
pub struct ERPIO3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO3_0_W<'a> {
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
#[doc = "Reader of field `ERPIO3_1`"]
pub type ERPIO3_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO3_1`"]
pub struct ERPIO3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO3_1_W<'a> {
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
#[doc = "Reader of field `ERPIO3_2`"]
pub type ERPIO3_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO3_2`"]
pub struct ERPIO3_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO3_2_W<'a> {
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
#[doc = "Reader of field `ERPIO3_3`"]
pub type ERPIO3_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO3_3`"]
pub struct ERPIO3_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO3_3_W<'a> {
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
    #[doc = "Bit 0 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_8(&self) -> ERPIO2_8_R {
        ERPIO2_8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_9(&self) -> ERPIO2_9_R {
        ERPIO2_9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_10(&self) -> ERPIO2_10_R {
        ERPIO2_10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_11(&self) -> ERPIO2_11_R {
        ERPIO2_11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_0(&self) -> ERPIO3_0_R {
        ERPIO3_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_1(&self) -> ERPIO3_1_R {
        ERPIO3_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_2(&self) -> ERPIO3_2_R {
        ERPIO3_2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_3(&self) -> ERPIO3_3_R {
        ERPIO3_3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_8(&mut self) -> ERPIO2_8_W {
        ERPIO2_8_W { w: self }
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_9(&mut self) -> ERPIO2_9_W {
        ERPIO2_9_W { w: self }
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_10(&mut self) -> ERPIO2_10_W {
        ERPIO2_10_W { w: self }
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_11(&mut self) -> ERPIO2_11_W {
        ERPIO2_11_W { w: self }
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_0(&mut self) -> ERPIO3_0_W {
        ERPIO3_0_W { w: self }
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_1(&mut self) -> ERPIO3_1_W {
        ERPIO3_1_W { w: self }
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_2(&mut self) -> ERPIO3_2_W {
        ERPIO3_2_W { w: self }
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio3_3(&mut self) -> ERPIO3_3_W {
        ERPIO3_3_W { w: self }
    }
}
