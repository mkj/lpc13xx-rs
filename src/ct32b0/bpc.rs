#[doc = "Reader of register BPC"]
pub type R = crate::R<u32, super::BPC>;
#[doc = "Writer for register BPC"]
pub type W = crate::W<u32, super::BPC>;
#[doc = "Register BPC `reset()`'s with value 0"]
impl crate::ResetValue for super::BPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
}
