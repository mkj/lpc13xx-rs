#[doc = "Reader of register STARTSRP1"]
pub type R = crate::R<u32, super::STARTSRP1>;
#[doc = "Reader of field `SRPIO2_8`"]
pub type SRPIO2_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_9`"]
pub type SRPIO2_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_10`"]
pub type SRPIO2_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_11`"]
pub type SRPIO2_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO3_0`"]
pub type SRPIO3_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO3_1`"]
pub type SRPIO3_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO3_2`"]
pub type SRPIO3_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO3_3`"]
pub type SRPIO3_3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_8(&self) -> SRPIO2_8_R {
        SRPIO2_8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_9(&self) -> SRPIO2_9_R {
        SRPIO2_9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_10(&self) -> SRPIO2_10_R {
        SRPIO2_10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start signal status for start logic input PIO2_n (bit 0 = PIO2_8, ..., bit 3 = PIO2_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_11(&self) -> SRPIO2_11_R {
        SRPIO2_11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_0(&self) -> SRPIO3_0_R {
        SRPIO3_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_1(&self) -> SRPIO3_1_R {
        SRPIO3_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_2(&self) -> SRPIO3_2_R {
        SRPIO3_2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Start signal status for start logic input PIO3_n (bit 4 = PIO3_0, ..., bit 7 = PIO3_3). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio3_3(&self) -> SRPIO3_3_R {
        SRPIO3_3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
