#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `RAWST0`"]
pub type RAWST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST1`"]
pub type RAWST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST2`"]
pub type RAWST2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST3`"]
pub type RAWST3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST4`"]
pub type RAWST4_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST5`"]
pub type RAWST5_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST6`"]
pub type RAWST6_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST7`"]
pub type RAWST7_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST8`"]
pub type RAWST8_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST9`"]
pub type RAWST9_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST10`"]
pub type RAWST10_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWST11`"]
pub type RAWST11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst0(&self) -> RAWST0_R {
        RAWST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst1(&self) -> RAWST1_R {
        RAWST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst2(&self) -> RAWST2_R {
        RAWST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst3(&self) -> RAWST3_R {
        RAWST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst4(&self) -> RAWST4_R {
        RAWST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst5(&self) -> RAWST5_R {
        RAWST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst6(&self) -> RAWST6_R {
        RAWST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst7(&self) -> RAWST7_R {
        RAWST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst8(&self) -> RAWST8_R {
        RAWST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst9(&self) -> RAWST9_R {
        RAWST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst10(&self) -> RAWST10_R {
        RAWST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status (x = 0 to 11). 0 = No interrupt on pin PIOn_x. 1 = Interrupt requirements met on PIOn_x."]
    #[inline(always)]
    pub fn rawst11(&self) -> RAWST11_R {
        RAWST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
