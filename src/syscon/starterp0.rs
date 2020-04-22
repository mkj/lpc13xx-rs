#[doc = "Reader of register STARTERP0"]
pub type R = crate::R<u32, super::STARTERP0>;
#[doc = "Writer for register STARTERP0"]
pub type W = crate::W<u32, super::STARTERP0>;
#[doc = "Register STARTERP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERPIO0_0`"]
pub type ERPIO0_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_0`"]
pub struct ERPIO0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_0_W<'a> {
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
#[doc = "Reader of field `ERPIO0_1`"]
pub type ERPIO0_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_1`"]
pub struct ERPIO0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_1_W<'a> {
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
#[doc = "Reader of field `ERPIO0_2`"]
pub type ERPIO0_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_2`"]
pub struct ERPIO0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_2_W<'a> {
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
#[doc = "Reader of field `ERPIO0_3`"]
pub type ERPIO0_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_3`"]
pub struct ERPIO0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_3_W<'a> {
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
#[doc = "Reader of field `ERPIO0_4`"]
pub type ERPIO0_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_4`"]
pub struct ERPIO0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_4_W<'a> {
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
#[doc = "Reader of field `ERPIO0_5`"]
pub type ERPIO0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_5`"]
pub struct ERPIO0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_5_W<'a> {
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
#[doc = "Reader of field `ERPIO0_6`"]
pub type ERPIO0_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_6`"]
pub struct ERPIO0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_6_W<'a> {
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
#[doc = "Reader of field `ERPIO0_7`"]
pub type ERPIO0_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_7`"]
pub struct ERPIO0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_7_W<'a> {
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
#[doc = "Reader of field `ERPIO0_8`"]
pub type ERPIO0_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_8`"]
pub struct ERPIO0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_8_W<'a> {
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
#[doc = "Reader of field `ERPIO0_9`"]
pub type ERPIO0_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_9`"]
pub struct ERPIO0_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_9_W<'a> {
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
#[doc = "Reader of field `ERPIO0_10`"]
pub type ERPIO0_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_10`"]
pub struct ERPIO0_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_10_W<'a> {
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
#[doc = "Reader of field `ERPIO0_11`"]
pub type ERPIO0_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO0_11`"]
pub struct ERPIO0_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO0_11_W<'a> {
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
#[doc = "Reader of field `ERPIO1_0`"]
pub type ERPIO1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_0`"]
pub struct ERPIO1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_0_W<'a> {
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
#[doc = "Reader of field `ERPIO1_1`"]
pub type ERPIO1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_1`"]
pub struct ERPIO1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_1_W<'a> {
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
#[doc = "Reader of field `ERPIO1_2`"]
pub type ERPIO1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_2`"]
pub struct ERPIO1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_3`"]
pub type ERPIO1_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_3`"]
pub struct ERPIO1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_4`"]
pub type ERPIO1_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_4`"]
pub struct ERPIO1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_5`"]
pub type ERPIO1_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_5`"]
pub struct ERPIO1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_6`"]
pub type ERPIO1_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_6`"]
pub struct ERPIO1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_7`"]
pub type ERPIO1_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_7`"]
pub struct ERPIO1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_8`"]
pub type ERPIO1_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_8`"]
pub struct ERPIO1_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_9`"]
pub type ERPIO1_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_9`"]
pub struct ERPIO1_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_10`"]
pub type ERPIO1_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_10`"]
pub struct ERPIO1_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ERPIO1_11`"]
pub type ERPIO1_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO1_11`"]
pub struct ERPIO1_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO1_11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_0`"]
pub type ERPIO2_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_0`"]
pub struct ERPIO2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_1`"]
pub type ERPIO2_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_1`"]
pub struct ERPIO2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_2`"]
pub type ERPIO2_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_2`"]
pub struct ERPIO2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_3`"]
pub type ERPIO2_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_3`"]
pub struct ERPIO2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_4`"]
pub type ERPIO2_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_4`"]
pub struct ERPIO2_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_5`"]
pub type ERPIO2_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_5`"]
pub struct ERPIO2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_6`"]
pub type ERPIO2_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_6`"]
pub struct ERPIO2_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ERPIO2_7`"]
pub type ERPIO2_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERPIO2_7`"]
pub struct ERPIO2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERPIO2_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_0(&self) -> ERPIO0_0_R {
        ERPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_1(&self) -> ERPIO0_1_R {
        ERPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_2(&self) -> ERPIO0_2_R {
        ERPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_3(&self) -> ERPIO0_3_R {
        ERPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_4(&self) -> ERPIO0_4_R {
        ERPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_5(&self) -> ERPIO0_5_R {
        ERPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_6(&self) -> ERPIO0_6_R {
        ERPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_7(&self) -> ERPIO0_7_R {
        ERPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_8(&self) -> ERPIO0_8_R {
        ERPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_9(&self) -> ERPIO0_9_R {
        ERPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_10(&self) -> ERPIO0_10_R {
        ERPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_11(&self) -> ERPIO0_11_R {
        ERPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_0(&self) -> ERPIO1_0_R {
        ERPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_1(&self) -> ERPIO1_1_R {
        ERPIO1_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_2(&self) -> ERPIO1_2_R {
        ERPIO1_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_3(&self) -> ERPIO1_3_R {
        ERPIO1_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_4(&self) -> ERPIO1_4_R {
        ERPIO1_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_5(&self) -> ERPIO1_5_R {
        ERPIO1_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_6(&self) -> ERPIO1_6_R {
        ERPIO1_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_7(&self) -> ERPIO1_7_R {
        ERPIO1_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_8(&self) -> ERPIO1_8_R {
        ERPIO1_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_9(&self) -> ERPIO1_9_R {
        ERPIO1_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_10(&self) -> ERPIO1_10_R {
        ERPIO1_10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_11(&self) -> ERPIO1_11_R {
        ERPIO1_11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_0(&self) -> ERPIO2_0_R {
        ERPIO2_0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_1(&self) -> ERPIO2_1_R {
        ERPIO2_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_2(&self) -> ERPIO2_2_R {
        ERPIO2_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_3(&self) -> ERPIO2_3_R {
        ERPIO2_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_4(&self) -> ERPIO2_4_R {
        ERPIO2_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_5(&self) -> ERPIO2_5_R {
        ERPIO2_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_6(&self) -> ERPIO2_6_R {
        ERPIO2_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_7(&self) -> ERPIO2_7_R {
        ERPIO2_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_0(&mut self) -> ERPIO0_0_W {
        ERPIO0_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_1(&mut self) -> ERPIO0_1_W {
        ERPIO0_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_2(&mut self) -> ERPIO0_2_W {
        ERPIO0_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_3(&mut self) -> ERPIO0_3_W {
        ERPIO0_3_W { w: self }
    }
    #[doc = "Bit 4 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_4(&mut self) -> ERPIO0_4_W {
        ERPIO0_4_W { w: self }
    }
    #[doc = "Bit 5 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_5(&mut self) -> ERPIO0_5_W {
        ERPIO0_5_W { w: self }
    }
    #[doc = "Bit 6 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_6(&mut self) -> ERPIO0_6_W {
        ERPIO0_6_W { w: self }
    }
    #[doc = "Bit 7 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_7(&mut self) -> ERPIO0_7_W {
        ERPIO0_7_W { w: self }
    }
    #[doc = "Bit 8 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_8(&mut self) -> ERPIO0_8_W {
        ERPIO0_8_W { w: self }
    }
    #[doc = "Bit 9 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_9(&mut self) -> ERPIO0_9_W {
        ERPIO0_9_W { w: self }
    }
    #[doc = "Bit 10 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_10(&mut self) -> ERPIO0_10_W {
        ERPIO0_10_W { w: self }
    }
    #[doc = "Bit 11 - Enable start signal for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio0_11(&mut self) -> ERPIO0_11_W {
        ERPIO0_11_W { w: self }
    }
    #[doc = "Bit 12 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_0(&mut self) -> ERPIO1_0_W {
        ERPIO1_0_W { w: self }
    }
    #[doc = "Bit 13 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_1(&mut self) -> ERPIO1_1_W {
        ERPIO1_1_W { w: self }
    }
    #[doc = "Bit 14 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_2(&mut self) -> ERPIO1_2_W {
        ERPIO1_2_W { w: self }
    }
    #[doc = "Bit 15 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_3(&mut self) -> ERPIO1_3_W {
        ERPIO1_3_W { w: self }
    }
    #[doc = "Bit 16 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_4(&mut self) -> ERPIO1_4_W {
        ERPIO1_4_W { w: self }
    }
    #[doc = "Bit 17 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_5(&mut self) -> ERPIO1_5_W {
        ERPIO1_5_W { w: self }
    }
    #[doc = "Bit 18 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_6(&mut self) -> ERPIO1_6_W {
        ERPIO1_6_W { w: self }
    }
    #[doc = "Bit 19 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_7(&mut self) -> ERPIO1_7_W {
        ERPIO1_7_W { w: self }
    }
    #[doc = "Bit 20 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_8(&mut self) -> ERPIO1_8_W {
        ERPIO1_8_W { w: self }
    }
    #[doc = "Bit 21 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_9(&mut self) -> ERPIO1_9_W {
        ERPIO1_9_W { w: self }
    }
    #[doc = "Bit 22 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_10(&mut self) -> ERPIO1_10_W {
        ERPIO1_10_W { w: self }
    }
    #[doc = "Bit 23 - Enable start signal for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio1_11(&mut self) -> ERPIO1_11_W {
        ERPIO1_11_W { w: self }
    }
    #[doc = "Bit 24 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_0(&mut self) -> ERPIO2_0_W {
        ERPIO2_0_W { w: self }
    }
    #[doc = "Bit 25 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_1(&mut self) -> ERPIO2_1_W {
        ERPIO2_1_W { w: self }
    }
    #[doc = "Bit 26 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_2(&mut self) -> ERPIO2_2_W {
        ERPIO2_2_W { w: self }
    }
    #[doc = "Bit 27 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_3(&mut self) -> ERPIO2_3_W {
        ERPIO2_3_W { w: self }
    }
    #[doc = "Bit 28 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_4(&mut self) -> ERPIO2_4_W {
        ERPIO2_4_W { w: self }
    }
    #[doc = "Bit 29 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_5(&mut self) -> ERPIO2_5_W {
        ERPIO2_5_W { w: self }
    }
    #[doc = "Bit 30 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_6(&mut self) -> ERPIO2_6_W {
        ERPIO2_6_W { w: self }
    }
    #[doc = "Bit 31 - Enable start signal for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Disabled. 1 = Enabled."]
    #[inline(always)]
    pub fn erpio2_7(&mut self) -> ERPIO2_7_W {
        ERPIO2_7_W { w: self }
    }
}
