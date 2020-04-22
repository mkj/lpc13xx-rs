#[doc = "Reader of register PIOPORCAP0"]
pub type R = crate::R<u32, super::PIOPORCAP0>;
#[doc = "Reader of field `CAPPIO0_0`"]
pub type CAPPIO0_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_1`"]
pub type CAPPIO0_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_2`"]
pub type CAPPIO0_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_3`"]
pub type CAPPIO0_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_4`"]
pub type CAPPIO0_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_5`"]
pub type CAPPIO0_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_6`"]
pub type CAPPIO0_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_7`"]
pub type CAPPIO0_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_8`"]
pub type CAPPIO0_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_9`"]
pub type CAPPIO0_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_10`"]
pub type CAPPIO0_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO0_11`"]
pub type CAPPIO0_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_0`"]
pub type CAPPIO1_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_1`"]
pub type CAPPIO1_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_2`"]
pub type CAPPIO1_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_3`"]
pub type CAPPIO1_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_4`"]
pub type CAPPIO1_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_5`"]
pub type CAPPIO1_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_6`"]
pub type CAPPIO1_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_7`"]
pub type CAPPIO1_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_8`"]
pub type CAPPIO1_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_9`"]
pub type CAPPIO1_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_10`"]
pub type CAPPIO1_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO1_11`"]
pub type CAPPIO1_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_0`"]
pub type CAPPIO2_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_1`"]
pub type CAPPIO2_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_2`"]
pub type CAPPIO2_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_3`"]
pub type CAPPIO2_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_4`"]
pub type CAPPIO2_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_5`"]
pub type CAPPIO2_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_6`"]
pub type CAPPIO2_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAPPIO2_7`"]
pub type CAPPIO2_7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_0(&self) -> CAPPIO0_0_R {
        CAPPIO0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_1(&self) -> CAPPIO0_1_R {
        CAPPIO0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_2(&self) -> CAPPIO0_2_R {
        CAPPIO0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_3(&self) -> CAPPIO0_3_R {
        CAPPIO0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_4(&self) -> CAPPIO0_4_R {
        CAPPIO0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_5(&self) -> CAPPIO0_5_R {
        CAPPIO0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_6(&self) -> CAPPIO0_6_R {
        CAPPIO0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_7(&self) -> CAPPIO0_7_R {
        CAPPIO0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_8(&self) -> CAPPIO0_8_R {
        CAPPIO0_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_9(&self) -> CAPPIO0_9_R {
        CAPPIO0_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_10(&self) -> CAPPIO0_10_R {
        CAPPIO0_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Raw reset status input PIO0_11 to PIO0_0"]
    #[inline(always)]
    pub fn cappio0_11(&self) -> CAPPIO0_11_R {
        CAPPIO0_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_0(&self) -> CAPPIO1_0_R {
        CAPPIO1_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_1(&self) -> CAPPIO1_1_R {
        CAPPIO1_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_2(&self) -> CAPPIO1_2_R {
        CAPPIO1_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_3(&self) -> CAPPIO1_3_R {
        CAPPIO1_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_4(&self) -> CAPPIO1_4_R {
        CAPPIO1_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_5(&self) -> CAPPIO1_5_R {
        CAPPIO1_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_6(&self) -> CAPPIO1_6_R {
        CAPPIO1_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_7(&self) -> CAPPIO1_7_R {
        CAPPIO1_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_8(&self) -> CAPPIO1_8_R {
        CAPPIO1_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_9(&self) -> CAPPIO1_9_R {
        CAPPIO1_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_10(&self) -> CAPPIO1_10_R {
        CAPPIO1_10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Raw reset status input PIO1_11 to PIO1_0"]
    #[inline(always)]
    pub fn cappio1_11(&self) -> CAPPIO1_11_R {
        CAPPIO1_11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_0(&self) -> CAPPIO2_0_R {
        CAPPIO2_0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_1(&self) -> CAPPIO2_1_R {
        CAPPIO2_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_2(&self) -> CAPPIO2_2_R {
        CAPPIO2_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_3(&self) -> CAPPIO2_3_R {
        CAPPIO2_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_4(&self) -> CAPPIO2_4_R {
        CAPPIO2_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_5(&self) -> CAPPIO2_5_R {
        CAPPIO2_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_6(&self) -> CAPPIO2_6_R {
        CAPPIO2_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Raw reset status input PIO2_7 to PIO2_0"]
    #[inline(always)]
    pub fn cappio2_7(&self) -> CAPPIO2_7_R {
        CAPPIO2_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
