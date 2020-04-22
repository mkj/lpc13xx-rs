#[doc = "Reader of register STARTAPRP0"]
pub type R = crate::R<u32, super::STARTAPRP0>;
#[doc = "Writer for register STARTAPRP0"]
pub type W = crate::W<u32, super::STARTAPRP0>;
#[doc = "Register STARTAPRP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTAPRP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APRPIO0_0`"]
pub type APRPIO0_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_0`"]
pub struct APRPIO0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_0_W<'a> {
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
#[doc = "Reader of field `APRPIO0_1`"]
pub type APRPIO0_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_1`"]
pub struct APRPIO0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_1_W<'a> {
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
#[doc = "Reader of field `APRPIO0_2`"]
pub type APRPIO0_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_2`"]
pub struct APRPIO0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_2_W<'a> {
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
#[doc = "Reader of field `APRPIO0_3`"]
pub type APRPIO0_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_3`"]
pub struct APRPIO0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_3_W<'a> {
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
#[doc = "Reader of field `APRPIO0_4`"]
pub type APRPIO0_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_4`"]
pub struct APRPIO0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_4_W<'a> {
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
#[doc = "Reader of field `APRPIO0_5`"]
pub type APRPIO0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_5`"]
pub struct APRPIO0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_5_W<'a> {
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
#[doc = "Reader of field `APRPIO0_6`"]
pub type APRPIO0_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_6`"]
pub struct APRPIO0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_6_W<'a> {
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
#[doc = "Reader of field `APRPIO0_7`"]
pub type APRPIO0_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_7`"]
pub struct APRPIO0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_7_W<'a> {
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
#[doc = "Reader of field `APRPIO0_8`"]
pub type APRPIO0_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_8`"]
pub struct APRPIO0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_8_W<'a> {
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
#[doc = "Reader of field `APRPIO0_9`"]
pub type APRPIO0_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_9`"]
pub struct APRPIO0_9_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_9_W<'a> {
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
#[doc = "Reader of field `APRPIO0_10`"]
pub type APRPIO0_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_10`"]
pub struct APRPIO0_10_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_10_W<'a> {
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
#[doc = "Reader of field `APRPIO0_11`"]
pub type APRPIO0_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO0_11`"]
pub struct APRPIO0_11_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO0_11_W<'a> {
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
#[doc = "Reader of field `APRPIO1_0`"]
pub type APRPIO1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_0`"]
pub struct APRPIO1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_0_W<'a> {
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
#[doc = "Reader of field `APRPIO1_1`"]
pub type APRPIO1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_1`"]
pub struct APRPIO1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_1_W<'a> {
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
#[doc = "Reader of field `APRPIO1_2`"]
pub type APRPIO1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_2`"]
pub struct APRPIO1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_2_W<'a> {
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
#[doc = "Reader of field `APRPIO1_3`"]
pub type APRPIO1_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_3`"]
pub struct APRPIO1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_3_W<'a> {
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
#[doc = "Reader of field `APRPIO1_4`"]
pub type APRPIO1_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_4`"]
pub struct APRPIO1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_4_W<'a> {
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
#[doc = "Reader of field `APRPIO1_5`"]
pub type APRPIO1_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_5`"]
pub struct APRPIO1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_5_W<'a> {
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
#[doc = "Reader of field `APRPIO1_6`"]
pub type APRPIO1_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_6`"]
pub struct APRPIO1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_6_W<'a> {
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
#[doc = "Reader of field `APRPIO1_7`"]
pub type APRPIO1_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_7`"]
pub struct APRPIO1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_7_W<'a> {
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
#[doc = "Reader of field `APRPIO1_8`"]
pub type APRPIO1_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_8`"]
pub struct APRPIO1_8_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_8_W<'a> {
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
#[doc = "Reader of field `APRPIO1_9`"]
pub type APRPIO1_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_9`"]
pub struct APRPIO1_9_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_9_W<'a> {
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
#[doc = "Reader of field `APRPIO1_10`"]
pub type APRPIO1_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_10`"]
pub struct APRPIO1_10_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_10_W<'a> {
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
#[doc = "Reader of field `APRPIO1_11`"]
pub type APRPIO1_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO1_11`"]
pub struct APRPIO1_11_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO1_11_W<'a> {
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
#[doc = "Reader of field `APRPIO2_0`"]
pub type APRPIO2_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_0`"]
pub struct APRPIO2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_0_W<'a> {
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
#[doc = "Reader of field `APRPIO2_1`"]
pub type APRPIO2_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_1`"]
pub struct APRPIO2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_1_W<'a> {
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
#[doc = "Reader of field `APRPIO2_2`"]
pub type APRPIO2_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_2`"]
pub struct APRPIO2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_2_W<'a> {
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
#[doc = "Reader of field `APRPIO2_3`"]
pub type APRPIO2_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_3`"]
pub struct APRPIO2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_3_W<'a> {
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
#[doc = "Reader of field `APRPIO2_4`"]
pub type APRPIO2_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_4`"]
pub struct APRPIO2_4_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_4_W<'a> {
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
#[doc = "Reader of field `APRPIO2_5`"]
pub type APRPIO2_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_5`"]
pub struct APRPIO2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_5_W<'a> {
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
#[doc = "Reader of field `APRPIO2_6`"]
pub type APRPIO2_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_6`"]
pub struct APRPIO2_6_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_6_W<'a> {
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
#[doc = "Reader of field `APRPIO2_7`"]
pub type APRPIO2_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRPIO2_7`"]
pub struct APRPIO2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> APRPIO2_7_W<'a> {
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
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&self) -> APRPIO0_0_R {
        APRPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&self) -> APRPIO0_1_R {
        APRPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&self) -> APRPIO0_2_R {
        APRPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&self) -> APRPIO0_3_R {
        APRPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&self) -> APRPIO0_4_R {
        APRPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&self) -> APRPIO0_5_R {
        APRPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&self) -> APRPIO0_6_R {
        APRPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&self) -> APRPIO0_7_R {
        APRPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&self) -> APRPIO0_8_R {
        APRPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&self) -> APRPIO0_9_R {
        APRPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&self) -> APRPIO0_10_R {
        APRPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&self) -> APRPIO0_11_R {
        APRPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&self) -> APRPIO1_0_R {
        APRPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_1(&self) -> APRPIO1_1_R {
        APRPIO1_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_2(&self) -> APRPIO1_2_R {
        APRPIO1_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_3(&self) -> APRPIO1_3_R {
        APRPIO1_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_4(&self) -> APRPIO1_4_R {
        APRPIO1_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_5(&self) -> APRPIO1_5_R {
        APRPIO1_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_6(&self) -> APRPIO1_6_R {
        APRPIO1_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_7(&self) -> APRPIO1_7_R {
        APRPIO1_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_8(&self) -> APRPIO1_8_R {
        APRPIO1_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_9(&self) -> APRPIO1_9_R {
        APRPIO1_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_10(&self) -> APRPIO1_10_R {
        APRPIO1_10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_11(&self) -> APRPIO1_11_R {
        APRPIO1_11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_0(&self) -> APRPIO2_0_R {
        APRPIO2_0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_1(&self) -> APRPIO2_1_R {
        APRPIO2_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_2(&self) -> APRPIO2_2_R {
        APRPIO2_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_3(&self) -> APRPIO2_3_R {
        APRPIO2_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_4(&self) -> APRPIO2_4_R {
        APRPIO2_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_5(&self) -> APRPIO2_5_R {
        APRPIO2_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_6(&self) -> APRPIO2_6_R {
        APRPIO2_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_7(&self) -> APRPIO2_7_R {
        APRPIO2_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_0(&mut self) -> APRPIO0_0_W {
        APRPIO0_0_W { w: self }
    }
    #[doc = "Bit 1 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_1(&mut self) -> APRPIO0_1_W {
        APRPIO0_1_W { w: self }
    }
    #[doc = "Bit 2 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_2(&mut self) -> APRPIO0_2_W {
        APRPIO0_2_W { w: self }
    }
    #[doc = "Bit 3 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_3(&mut self) -> APRPIO0_3_W {
        APRPIO0_3_W { w: self }
    }
    #[doc = "Bit 4 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_4(&mut self) -> APRPIO0_4_W {
        APRPIO0_4_W { w: self }
    }
    #[doc = "Bit 5 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_5(&mut self) -> APRPIO0_5_W {
        APRPIO0_5_W { w: self }
    }
    #[doc = "Bit 6 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_6(&mut self) -> APRPIO0_6_W {
        APRPIO0_6_W { w: self }
    }
    #[doc = "Bit 7 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_7(&mut self) -> APRPIO0_7_W {
        APRPIO0_7_W { w: self }
    }
    #[doc = "Bit 8 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_8(&mut self) -> APRPIO0_8_W {
        APRPIO0_8_W { w: self }
    }
    #[doc = "Bit 9 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_9(&mut self) -> APRPIO0_9_W {
        APRPIO0_9_W { w: self }
    }
    #[doc = "Bit 10 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_10(&mut self) -> APRPIO0_10_W {
        APRPIO0_10_W { w: self }
    }
    #[doc = "Bit 11 - Edge select for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio0_11(&mut self) -> APRPIO0_11_W {
        APRPIO0_11_W { w: self }
    }
    #[doc = "Bit 12 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_0(&mut self) -> APRPIO1_0_W {
        APRPIO1_0_W { w: self }
    }
    #[doc = "Bit 13 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_1(&mut self) -> APRPIO1_1_W {
        APRPIO1_1_W { w: self }
    }
    #[doc = "Bit 14 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_2(&mut self) -> APRPIO1_2_W {
        APRPIO1_2_W { w: self }
    }
    #[doc = "Bit 15 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_3(&mut self) -> APRPIO1_3_W {
        APRPIO1_3_W { w: self }
    }
    #[doc = "Bit 16 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_4(&mut self) -> APRPIO1_4_W {
        APRPIO1_4_W { w: self }
    }
    #[doc = "Bit 17 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_5(&mut self) -> APRPIO1_5_W {
        APRPIO1_5_W { w: self }
    }
    #[doc = "Bit 18 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_6(&mut self) -> APRPIO1_6_W {
        APRPIO1_6_W { w: self }
    }
    #[doc = "Bit 19 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_7(&mut self) -> APRPIO1_7_W {
        APRPIO1_7_W { w: self }
    }
    #[doc = "Bit 20 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_8(&mut self) -> APRPIO1_8_W {
        APRPIO1_8_W { w: self }
    }
    #[doc = "Bit 21 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_9(&mut self) -> APRPIO1_9_W {
        APRPIO1_9_W { w: self }
    }
    #[doc = "Bit 22 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_10(&mut self) -> APRPIO1_10_W {
        APRPIO1_10_W { w: self }
    }
    #[doc = "Bit 23 - Edge select for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio1_11(&mut self) -> APRPIO1_11_W {
        APRPIO1_11_W { w: self }
    }
    #[doc = "Bit 24 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_0(&mut self) -> APRPIO2_0_W {
        APRPIO2_0_W { w: self }
    }
    #[doc = "Bit 25 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_1(&mut self) -> APRPIO2_1_W {
        APRPIO2_1_W { w: self }
    }
    #[doc = "Bit 26 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_2(&mut self) -> APRPIO2_2_W {
        APRPIO2_2_W { w: self }
    }
    #[doc = "Bit 27 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_3(&mut self) -> APRPIO2_3_W {
        APRPIO2_3_W { w: self }
    }
    #[doc = "Bit 28 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_4(&mut self) -> APRPIO2_4_W {
        APRPIO2_4_W { w: self }
    }
    #[doc = "Bit 29 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_5(&mut self) -> APRPIO2_5_W {
        APRPIO2_5_W { w: self }
    }
    #[doc = "Bit 30 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_6(&mut self) -> APRPIO2_6_W {
        APRPIO2_6_W { w: self }
    }
    #[doc = "Bit 31 - Edge select for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Falling edge. 1 = Rising edge."]
    #[inline(always)]
    pub fn aprpio2_7(&mut self) -> APRPIO2_7_W {
        APRPIO2_7_W { w: self }
    }
}
