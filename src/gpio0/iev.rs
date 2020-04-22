#[doc = "Reader of register IEV"]
pub type R = crate::R<u32, super::IEV>;
#[doc = "Writer for register IEV"]
pub type W = crate::W<u32, super::IEV>;
#[doc = "Register IEV `reset()`'s with value 0"]
impl crate::ResetValue for super::IEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEV0`"]
pub type IEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV0`"]
pub struct IEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV0_W<'a> {
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
#[doc = "Reader of field `IEV1`"]
pub type IEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV1`"]
pub struct IEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV1_W<'a> {
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
#[doc = "Reader of field `IEV2`"]
pub type IEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV2`"]
pub struct IEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV2_W<'a> {
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
#[doc = "Reader of field `IEV3`"]
pub type IEV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV3`"]
pub struct IEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV3_W<'a> {
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
#[doc = "Reader of field `IEV4`"]
pub type IEV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV4`"]
pub struct IEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV4_W<'a> {
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
#[doc = "Reader of field `IEV5`"]
pub type IEV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV5`"]
pub struct IEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV5_W<'a> {
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
#[doc = "Reader of field `IEV6`"]
pub type IEV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV6`"]
pub struct IEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV6_W<'a> {
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
#[doc = "Reader of field `IEV7`"]
pub type IEV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV7`"]
pub struct IEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV7_W<'a> {
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
#[doc = "Reader of field `IEV8`"]
pub type IEV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV8`"]
pub struct IEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV8_W<'a> {
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
#[doc = "Reader of field `IEV9`"]
pub type IEV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV9`"]
pub struct IEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV9_W<'a> {
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
#[doc = "Reader of field `IEV10`"]
pub type IEV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV10`"]
pub struct IEV10_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV10_W<'a> {
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
#[doc = "Reader of field `IEV11`"]
pub type IEV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEV11`"]
pub struct IEV11_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV11_W<'a> {
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
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev0(&self) -> IEV0_R {
        IEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev1(&self) -> IEV1_R {
        IEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev2(&self) -> IEV2_R {
        IEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev3(&self) -> IEV3_R {
        IEV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev4(&self) -> IEV4_R {
        IEV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev5(&self) -> IEV5_R {
        IEV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev6(&self) -> IEV6_R {
        IEV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev7(&self) -> IEV7_R {
        IEV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev8(&self) -> IEV8_R {
        IEV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev9(&self) -> IEV9_R {
        IEV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev10(&self) -> IEV10_R {
        IEV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev11(&self) -> IEV11_R {
        IEV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev0(&mut self) -> IEV0_W {
        IEV0_W { w: self }
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev1(&mut self) -> IEV1_W {
        IEV1_W { w: self }
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev2(&mut self) -> IEV2_W {
        IEV2_W { w: self }
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev3(&mut self) -> IEV3_W {
        IEV3_W { w: self }
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev4(&mut self) -> IEV4_W {
        IEV4_W { w: self }
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev5(&mut self) -> IEV5_W {
        IEV5_W { w: self }
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev6(&mut self) -> IEV6_W {
        IEV6_W { w: self }
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev7(&mut self) -> IEV7_W {
        IEV7_W { w: self }
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev8(&mut self) -> IEV8_W {
        IEV8_W { w: self }
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev9(&mut self) -> IEV9_W {
        IEV9_W { w: self }
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev10(&mut self) -> IEV10_W {
        IEV10_W { w: self }
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be triggered rising or falling edges (x = 0 to 11). 0 = Depending on setting in register GPIOIS (see Table 151), falling edges or LOW level on pin PIOn_x trigger an interrupt. 1 = Depending on setting in register GPIOIS (see Table 151), rising edges or HIGH level on pin PIOn_x trigger an interrupt."]
    #[inline(always)]
    pub fn iev11(&mut self) -> IEV11_W {
        IEV11_W { w: self }
    }
}
