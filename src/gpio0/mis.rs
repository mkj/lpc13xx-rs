#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `MASK0`"]
pub type MASK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK1`"]
pub type MASK1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK2`"]
pub type MASK2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK3`"]
pub type MASK3_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK4`"]
pub type MASK4_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK5`"]
pub type MASK5_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK6`"]
pub type MASK6_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK7`"]
pub type MASK7_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK8`"]
pub type MASK8_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK9`"]
pub type MASK9_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK10`"]
pub type MASK10_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK11`"]
pub type MASK11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask8(&self) -> MASK8_R {
        MASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask9(&self) -> MASK9_R {
        MASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask10(&self) -> MASK10_R {
        MASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be masked (x = 0 to 11). 0 = No interrupt or interrupt masked on pin PIOn_x. 1 = Interrupt on PIOn_x."]
    #[inline(always)]
    pub fn mask11(&self) -> MASK11_R {
        MASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
