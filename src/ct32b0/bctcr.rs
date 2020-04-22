#[doc = "Reader of register BCTCR"]
pub type R = crate::R<u32, super::BCTCR>;
#[doc = "Writer for register BCTCR"]
pub type W = crate::W<u32, super::BCTCR>;
#[doc = "Register BCTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BCTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTM_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLLING = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    BOTHEDGES = 3,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTM`"]
pub type CTM_R = crate::R<u8, CTM_A>;
impl CTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::TIMER_MODE_EVERY_RI,
            1 => CTM_A::RISING,
            2 => CTM_A::FALLLING,
            3 => CTM_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline(always)]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        *self == CTM_A::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CTM_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == CTM_A::FALLLING
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == CTM_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `CTM`"]
pub struct CTM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTM_A::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTM_A::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut W {
        self.variant(CTM_A::FALLLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(CTM_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: CT32Bn_CAP0"]
    CT32BN_CAP0 = 0,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIS`"]
pub type CIS_R = crate::R<u8, CIS_A>;
impl CIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIS_A::CT32BN_CAP0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CT32BN_CAP0`"]
    #[inline(always)]
    pub fn is_ct32bn_cap0(&self) -> bool {
        *self == CIS_A::CT32BN_CAP0
    }
}
#[doc = "Write proxy for field `CIS`"]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CT32Bn_CAP0"]
    #[inline(always)]
    pub fn ct32bn_cap0(self) -> &'a mut W {
        self.variant(CIS_A::CT32BN_CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:"]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn ctm(&mut self) -> CTM_W {
        CTM_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking:"]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
}
