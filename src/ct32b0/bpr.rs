#[doc = "Reader of register BPR"]
pub type R = crate::R<u32, super::BPR>;
#[doc = "Writer for register BPR"]
pub type W = crate::W<u32, super::BPR>;
#[doc = "Register BPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Prescale value."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Prescale value."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
