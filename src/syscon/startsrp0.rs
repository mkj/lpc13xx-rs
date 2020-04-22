#[doc = "Reader of register STARTSRP0"]
pub type R = crate::R<u32, super::STARTSRP0>;
#[doc = "Reader of field `SRPIO0_0`"]
pub type SRPIO0_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_1`"]
pub type SRPIO0_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_2`"]
pub type SRPIO0_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_3`"]
pub type SRPIO0_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_4`"]
pub type SRPIO0_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_5`"]
pub type SRPIO0_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_6`"]
pub type SRPIO0_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_7`"]
pub type SRPIO0_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_8`"]
pub type SRPIO0_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_9`"]
pub type SRPIO0_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_10`"]
pub type SRPIO0_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO0_11`"]
pub type SRPIO0_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_0`"]
pub type SRPIO1_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_1`"]
pub type SRPIO1_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_2`"]
pub type SRPIO1_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_3`"]
pub type SRPIO1_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_4`"]
pub type SRPIO1_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_5`"]
pub type SRPIO1_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_6`"]
pub type SRPIO1_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_7`"]
pub type SRPIO1_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_8`"]
pub type SRPIO1_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_9`"]
pub type SRPIO1_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_10`"]
pub type SRPIO1_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO1_11`"]
pub type SRPIO1_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_0`"]
pub type SRPIO2_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_1`"]
pub type SRPIO2_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_2`"]
pub type SRPIO2_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_3`"]
pub type SRPIO2_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_4`"]
pub type SRPIO2_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_5`"]
pub type SRPIO2_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_6`"]
pub type SRPIO2_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPIO2_7`"]
pub type SRPIO2_7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_0(&self) -> SRPIO0_0_R {
        SRPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_1(&self) -> SRPIO0_1_R {
        SRPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_2(&self) -> SRPIO0_2_R {
        SRPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_3(&self) -> SRPIO0_3_R {
        SRPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_4(&self) -> SRPIO0_4_R {
        SRPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_5(&self) -> SRPIO0_5_R {
        SRPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_6(&self) -> SRPIO0_6_R {
        SRPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_7(&self) -> SRPIO0_7_R {
        SRPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_8(&self) -> SRPIO0_8_R {
        SRPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_9(&self) -> SRPIO0_9_R {
        SRPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_10(&self) -> SRPIO0_10_R {
        SRPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Start signal status for start logic input PIO0_n (bit 0 = PIO0_1, ..., bit 11 = PIO0_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio0_11(&self) -> SRPIO0_11_R {
        SRPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_0(&self) -> SRPIO1_0_R {
        SRPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_1(&self) -> SRPIO1_1_R {
        SRPIO1_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_2(&self) -> SRPIO1_2_R {
        SRPIO1_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_3(&self) -> SRPIO1_3_R {
        SRPIO1_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_4(&self) -> SRPIO1_4_R {
        SRPIO1_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_5(&self) -> SRPIO1_5_R {
        SRPIO1_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_6(&self) -> SRPIO1_6_R {
        SRPIO1_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_7(&self) -> SRPIO1_7_R {
        SRPIO1_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_8(&self) -> SRPIO1_8_R {
        SRPIO1_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_9(&self) -> SRPIO1_9_R {
        SRPIO1_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_10(&self) -> SRPIO1_10_R {
        SRPIO1_10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Start signal status for start logic input PIO1_n (bit 12 = PIO1_0, ..., bit 23 = PIO1_11). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio1_11(&self) -> SRPIO1_11_R {
        SRPIO1_11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_0(&self) -> SRPIO2_0_R {
        SRPIO2_0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_1(&self) -> SRPIO2_1_R {
        SRPIO2_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_2(&self) -> SRPIO2_2_R {
        SRPIO2_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_3(&self) -> SRPIO2_3_R {
        SRPIO2_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_4(&self) -> SRPIO2_4_R {
        SRPIO2_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_5(&self) -> SRPIO2_5_R {
        SRPIO2_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_6(&self) -> SRPIO2_6_R {
        SRPIO2_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Start signal status for start logic input PIO2_n (bit 24 = PIO2_0, ..., bit 31 = PIO2_7). 0 = No start signal received. 1 = Start signal pending."]
    #[inline(always)]
    pub fn srpio2_7(&self) -> SRPIO2_7_R {
        SRPIO2_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
