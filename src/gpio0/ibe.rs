#[doc = "Reader of register IBE"]
pub type R = crate::R<u32, super::IBE>;
#[doc = "Writer for register IBE"]
pub type W = crate::W<u32, super::IBE>;
#[doc = "Register IBE `reset()`'s with value 0"]
impl crate::ResetValue for super::IBE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IBE0`"]
pub type IBE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE0`"]
pub struct IBE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE0_W<'a> {
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
#[doc = "Reader of field `IBE1`"]
pub type IBE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE1`"]
pub struct IBE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE1_W<'a> {
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
#[doc = "Reader of field `IBE2`"]
pub type IBE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE2`"]
pub struct IBE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE2_W<'a> {
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
#[doc = "Reader of field `IBE3`"]
pub type IBE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE3`"]
pub struct IBE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE3_W<'a> {
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
#[doc = "Reader of field `IBE4`"]
pub type IBE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE4`"]
pub struct IBE4_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE4_W<'a> {
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
#[doc = "Reader of field `IBE5`"]
pub type IBE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE5`"]
pub struct IBE5_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE5_W<'a> {
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
#[doc = "Reader of field `IBE6`"]
pub type IBE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE6`"]
pub struct IBE6_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE6_W<'a> {
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
#[doc = "Reader of field `IBE7`"]
pub type IBE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE7`"]
pub struct IBE7_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE7_W<'a> {
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
#[doc = "Reader of field `IBE8`"]
pub type IBE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE8`"]
pub struct IBE8_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE8_W<'a> {
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
#[doc = "Reader of field `IBE9`"]
pub type IBE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE9`"]
pub struct IBE9_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE9_W<'a> {
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
#[doc = "Reader of field `IBE10`"]
pub type IBE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE10`"]
pub struct IBE10_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE10_W<'a> {
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
#[doc = "Reader of field `IBE11`"]
pub type IBE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBE11`"]
pub struct IBE11_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE11_W<'a> {
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
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe0(&self) -> IBE0_R {
        IBE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe1(&self) -> IBE1_R {
        IBE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe2(&self) -> IBE2_R {
        IBE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe3(&self) -> IBE3_R {
        IBE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe4(&self) -> IBE4_R {
        IBE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe5(&self) -> IBE5_R {
        IBE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe6(&self) -> IBE6_R {
        IBE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe7(&self) -> IBE7_R {
        IBE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe8(&self) -> IBE8_R {
        IBE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe9(&self) -> IBE9_R {
        IBE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe10(&self) -> IBE10_R {
        IBE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe11(&self) -> IBE11_R {
        IBE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe0(&mut self) -> IBE0_W {
        IBE0_W { w: self }
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe1(&mut self) -> IBE1_W {
        IBE1_W { w: self }
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe2(&mut self) -> IBE2_W {
        IBE2_W { w: self }
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe3(&mut self) -> IBE3_W {
        IBE3_W { w: self }
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe4(&mut self) -> IBE4_W {
        IBE4_W { w: self }
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe5(&mut self) -> IBE5_W {
        IBE5_W { w: self }
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe6(&mut self) -> IBE6_W {
        IBE6_W { w: self }
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe7(&mut self) -> IBE7_W {
        IBE7_W { w: self }
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe8(&mut self) -> IBE8_W {
        IBE8_W { w: self }
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe9(&mut self) -> IBE9_W {
        IBE9_W { w: self }
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe10(&mut self) -> IBE10_W {
        IBE10_W { w: self }
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered on both edges (x = 0 to 11). 0 = Interrupt on pin PIOn_x is controlled through register GPIOIEV. 1 = Both edges on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn ibe11(&mut self) -> IBE11_W {
        IBE11_W { w: self }
    }
}
