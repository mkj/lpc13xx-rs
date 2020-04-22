#[doc = "Writer for register STARTRSRP0CLR"]
pub type W = crate::W<u32, super::STARTRSRP0CLR>;
#[doc = "Register STARTRSRP0CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTRSRP0CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RSRPIO0_0`"]
pub struct RSRPIO0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_0_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_1`"]
pub struct RSRPIO0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_1_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_2`"]
pub struct RSRPIO0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_2_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_3`"]
pub struct RSRPIO0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_3_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_4`"]
pub struct RSRPIO0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_4_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_5`"]
pub struct RSRPIO0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_5_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_6`"]
pub struct RSRPIO0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_6_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_7`"]
pub struct RSRPIO0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_7_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_8`"]
pub struct RSRPIO0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_8_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_9`"]
pub struct RSRPIO0_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_9_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_10`"]
pub struct RSRPIO0_10_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_10_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO0_11`"]
pub struct RSRPIO0_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO0_11_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_0`"]
pub struct RSRPIO1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_0_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_1`"]
pub struct RSRPIO1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_1_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_2`"]
pub struct RSRPIO1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_2_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_3`"]
pub struct RSRPIO1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_3_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_4`"]
pub struct RSRPIO1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_4_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_5`"]
pub struct RSRPIO1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_5_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_6`"]
pub struct RSRPIO1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_6_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_7`"]
pub struct RSRPIO1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_7_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_8`"]
pub struct RSRPIO1_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_8_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_9`"]
pub struct RSRPIO1_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_9_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_10`"]
pub struct RSRPIO1_10_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_10_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO1_11`"]
pub struct RSRPIO1_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO1_11_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_0`"]
pub struct RSRPIO2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_0_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_1`"]
pub struct RSRPIO2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_1_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_2`"]
pub struct RSRPIO2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_2_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_3`"]
pub struct RSRPIO2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_3_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_4`"]
pub struct RSRPIO2_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_4_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_5`"]
pub struct RSRPIO2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_5_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_6`"]
pub struct RSRPIO2_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_6_W<'a> {
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
#[doc = "Write proxy for field `RSRPIO2_7`"]
pub struct RSRPIO2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRPIO2_7_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_0(&mut self) -> RSRPIO0_0_W {
        RSRPIO0_0_W { w: self }
    }
    #[doc = "Bit 1 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_1(&mut self) -> RSRPIO0_1_W {
        RSRPIO0_1_W { w: self }
    }
    #[doc = "Bit 2 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_2(&mut self) -> RSRPIO0_2_W {
        RSRPIO0_2_W { w: self }
    }
    #[doc = "Bit 3 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_3(&mut self) -> RSRPIO0_3_W {
        RSRPIO0_3_W { w: self }
    }
    #[doc = "Bit 4 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_4(&mut self) -> RSRPIO0_4_W {
        RSRPIO0_4_W { w: self }
    }
    #[doc = "Bit 5 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_5(&mut self) -> RSRPIO0_5_W {
        RSRPIO0_5_W { w: self }
    }
    #[doc = "Bit 6 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_6(&mut self) -> RSRPIO0_6_W {
        RSRPIO0_6_W { w: self }
    }
    #[doc = "Bit 7 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_7(&mut self) -> RSRPIO0_7_W {
        RSRPIO0_7_W { w: self }
    }
    #[doc = "Bit 8 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_8(&mut self) -> RSRPIO0_8_W {
        RSRPIO0_8_W { w: self }
    }
    #[doc = "Bit 9 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_9(&mut self) -> RSRPIO0_9_W {
        RSRPIO0_9_W { w: self }
    }
    #[doc = "Bit 10 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_10(&mut self) -> RSRPIO0_10_W {
        RSRPIO0_10_W { w: self }
    }
    #[doc = "Bit 11 - Start signal reset for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio0_11(&mut self) -> RSRPIO0_11_W {
        RSRPIO0_11_W { w: self }
    }
    #[doc = "Bit 12 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_0(&mut self) -> RSRPIO1_0_W {
        RSRPIO1_0_W { w: self }
    }
    #[doc = "Bit 13 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_1(&mut self) -> RSRPIO1_1_W {
        RSRPIO1_1_W { w: self }
    }
    #[doc = "Bit 14 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_2(&mut self) -> RSRPIO1_2_W {
        RSRPIO1_2_W { w: self }
    }
    #[doc = "Bit 15 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_3(&mut self) -> RSRPIO1_3_W {
        RSRPIO1_3_W { w: self }
    }
    #[doc = "Bit 16 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_4(&mut self) -> RSRPIO1_4_W {
        RSRPIO1_4_W { w: self }
    }
    #[doc = "Bit 17 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_5(&mut self) -> RSRPIO1_5_W {
        RSRPIO1_5_W { w: self }
    }
    #[doc = "Bit 18 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_6(&mut self) -> RSRPIO1_6_W {
        RSRPIO1_6_W { w: self }
    }
    #[doc = "Bit 19 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_7(&mut self) -> RSRPIO1_7_W {
        RSRPIO1_7_W { w: self }
    }
    #[doc = "Bit 20 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_8(&mut self) -> RSRPIO1_8_W {
        RSRPIO1_8_W { w: self }
    }
    #[doc = "Bit 21 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_9(&mut self) -> RSRPIO1_9_W {
        RSRPIO1_9_W { w: self }
    }
    #[doc = "Bit 22 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_10(&mut self) -> RSRPIO1_10_W {
        RSRPIO1_10_W { w: self }
    }
    #[doc = "Bit 23 - Start signal reset for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio1_11(&mut self) -> RSRPIO1_11_W {
        RSRPIO1_11_W { w: self }
    }
    #[doc = "Bit 24 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_0(&mut self) -> RSRPIO2_0_W {
        RSRPIO2_0_W { w: self }
    }
    #[doc = "Bit 25 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_1(&mut self) -> RSRPIO2_1_W {
        RSRPIO2_1_W { w: self }
    }
    #[doc = "Bit 26 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_2(&mut self) -> RSRPIO2_2_W {
        RSRPIO2_2_W { w: self }
    }
    #[doc = "Bit 27 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_3(&mut self) -> RSRPIO2_3_W {
        RSRPIO2_3_W { w: self }
    }
    #[doc = "Bit 28 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_4(&mut self) -> RSRPIO2_4_W {
        RSRPIO2_4_W { w: self }
    }
    #[doc = "Bit 29 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_5(&mut self) -> RSRPIO2_5_W {
        RSRPIO2_5_W { w: self }
    }
    #[doc = "Bit 30 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_6(&mut self) -> RSRPIO2_6_W {
        RSRPIO2_6_W { w: self }
    }
    #[doc = "Bit 31 - Start signal reset for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = Do nothing. 1 = Write: reset start signal."]
    #[inline(always)]
    pub fn rsrpio2_7(&mut self) -> RSRPIO2_7_W {
        RSRPIO2_7_W { w: self }
    }
}
