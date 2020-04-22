#[doc = "Reader of register DIR"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO0`"]
pub type IO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO0`"]
pub struct IO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_W<'a> {
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
#[doc = "Reader of field `IO1`"]
pub type IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO1`"]
pub struct IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_W<'a> {
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
#[doc = "Reader of field `IO2`"]
pub type IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO2`"]
pub struct IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_W<'a> {
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
#[doc = "Reader of field `IO3`"]
pub type IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO3`"]
pub struct IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_W<'a> {
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
#[doc = "Reader of field `IO4`"]
pub type IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO4`"]
pub struct IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_W<'a> {
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
#[doc = "Reader of field `IO5`"]
pub type IO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO5`"]
pub struct IO5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_W<'a> {
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
#[doc = "Reader of field `IO6`"]
pub type IO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO6`"]
pub struct IO6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_W<'a> {
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
#[doc = "Reader of field `IO7`"]
pub type IO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO7`"]
pub struct IO7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_W<'a> {
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
#[doc = "Reader of field `IO8`"]
pub type IO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO8`"]
pub struct IO8_W<'a> {
    w: &'a mut W,
}
impl<'a> IO8_W<'a> {
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
#[doc = "Reader of field `IO9`"]
pub type IO9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO9`"]
pub struct IO9_W<'a> {
    w: &'a mut W,
}
impl<'a> IO9_W<'a> {
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
#[doc = "Reader of field `IO10`"]
pub type IO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO10`"]
pub struct IO10_W<'a> {
    w: &'a mut W,
}
impl<'a> IO10_W<'a> {
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
#[doc = "Reader of field `IO11`"]
pub type IO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO11`"]
pub struct IO11_W<'a> {
    w: &'a mut W,
}
impl<'a> IO11_W<'a> {
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
    #[doc = "Bit 0 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io8(&self) -> IO8_R {
        IO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io9(&self) -> IO9_R {
        IO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io10(&self) -> IO10_R {
        IO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io11(&self) -> IO11_R {
        IO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W {
        IO0_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W {
        IO1_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W {
        IO2_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W {
        IO3_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W {
        IO4_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W {
        IO5_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W {
        IO6_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W {
        IO7_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io8(&mut self) -> IO8_W {
        IO8_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io9(&mut self) -> IO9_W {
        IO9_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io10(&mut self) -> IO10_W {
        IO10_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin x as input or output (x = 0 to 11). 0 = Pin PIOn_x is configured as input. 1 = Pin PIOn_x is configured as output."]
    #[inline(always)]
    pub fn io11(&mut self) -> IO11_W {
        IO11_W { w: self }
    }
}
