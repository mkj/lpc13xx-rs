#[doc = "Reader of register IS"]
pub type R = crate::R<u32, super::IS>;
#[doc = "Writer for register IS"]
pub type W = crate::W<u32, super::IS>;
#[doc = "Register IS `reset()`'s with value 0"]
impl crate::ResetValue for super::IS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENSE0`"]
pub type ISENSE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE0`"]
pub struct ISENSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE0_W<'a> {
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
#[doc = "Reader of field `ISENSE1`"]
pub type ISENSE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE1`"]
pub struct ISENSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE1_W<'a> {
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
#[doc = "Reader of field `ISENSE2`"]
pub type ISENSE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE2`"]
pub struct ISENSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE2_W<'a> {
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
#[doc = "Reader of field `ISENSE3`"]
pub type ISENSE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE3`"]
pub struct ISENSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE3_W<'a> {
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
#[doc = "Reader of field `ISENSE4`"]
pub type ISENSE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE4`"]
pub struct ISENSE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE4_W<'a> {
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
#[doc = "Reader of field `ISENSE5`"]
pub type ISENSE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE5`"]
pub struct ISENSE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE5_W<'a> {
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
#[doc = "Reader of field `ISENSE6`"]
pub type ISENSE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE6`"]
pub struct ISENSE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE6_W<'a> {
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
#[doc = "Reader of field `ISENSE7`"]
pub type ISENSE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE7`"]
pub struct ISENSE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE7_W<'a> {
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
#[doc = "Reader of field `ISENSE8`"]
pub type ISENSE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE8`"]
pub struct ISENSE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE8_W<'a> {
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
#[doc = "Reader of field `ISENSE9`"]
pub type ISENSE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE9`"]
pub struct ISENSE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE9_W<'a> {
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
#[doc = "Reader of field `ISENSE10`"]
pub type ISENSE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE10`"]
pub struct ISENSE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE10_W<'a> {
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
#[doc = "Reader of field `ISENSE11`"]
pub type ISENSE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISENSE11`"]
pub struct ISENSE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENSE11_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense0(&self) -> ISENSE0_R {
        ISENSE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense1(&self) -> ISENSE1_R {
        ISENSE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense2(&self) -> ISENSE2_R {
        ISENSE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense3(&self) -> ISENSE3_R {
        ISENSE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense4(&self) -> ISENSE4_R {
        ISENSE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense5(&self) -> ISENSE5_R {
        ISENSE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense6(&self) -> ISENSE6_R {
        ISENSE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense7(&self) -> ISENSE7_R {
        ISENSE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense8(&self) -> ISENSE8_R {
        ISENSE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense9(&self) -> ISENSE9_R {
        ISENSE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense10(&self) -> ISENSE10_R {
        ISENSE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense11(&self) -> ISENSE11_R {
        ISENSE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense0(&mut self) -> ISENSE0_W {
        ISENSE0_W { w: self }
    }
    #[doc = "Bit 1 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense1(&mut self) -> ISENSE1_W {
        ISENSE1_W { w: self }
    }
    #[doc = "Bit 2 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense2(&mut self) -> ISENSE2_W {
        ISENSE2_W { w: self }
    }
    #[doc = "Bit 3 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense3(&mut self) -> ISENSE3_W {
        ISENSE3_W { w: self }
    }
    #[doc = "Bit 4 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense4(&mut self) -> ISENSE4_W {
        ISENSE4_W { w: self }
    }
    #[doc = "Bit 5 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense5(&mut self) -> ISENSE5_W {
        ISENSE5_W { w: self }
    }
    #[doc = "Bit 6 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense6(&mut self) -> ISENSE6_W {
        ISENSE6_W { w: self }
    }
    #[doc = "Bit 7 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense7(&mut self) -> ISENSE7_W {
        ISENSE7_W { w: self }
    }
    #[doc = "Bit 8 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense8(&mut self) -> ISENSE8_W {
        ISENSE8_W { w: self }
    }
    #[doc = "Bit 9 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense9(&mut self) -> ISENSE9_W {
        ISENSE9_W { w: self }
    }
    #[doc = "Bit 10 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense10(&mut self) -> ISENSE10_W {
        ISENSE10_W { w: self }
    }
    #[doc = "Bit 11 - Selects interrupt on pin x as level or edge sensitive (x = 0 to 11). 0 = Interrupt on pin PIOn_x is configured as edge sensitive. 1 = Interrupt on pin PIOn_x is configured as level sensitive."]
    #[inline(always)]
    pub fn isense11(&mut self) -> ISENSE11_W {
        ISENSE11_W { w: self }
    }
}
