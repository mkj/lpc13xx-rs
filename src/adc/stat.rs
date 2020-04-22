#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `DONE0`"]
pub type DONE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE1`"]
pub type DONE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE2`"]
pub type DONE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE3`"]
pub type DONE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE4`"]
pub type DONE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE5`"]
pub type DONE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE6`"]
pub type DONE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE7`"]
pub type DONE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN0`"]
pub type OVERRUN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN1`"]
pub type OVERRUN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN2`"]
pub type OVERRUN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN3`"]
pub type OVERRUN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN4`"]
pub type OVERRUN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN5`"]
pub type OVERRUN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN6`"]
pub type OVERRUN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN7`"]
pub type OVERRUN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADINT`"]
pub type ADINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done2(&self) -> DONE2_R {
        DONE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done3(&self) -> DONE3_R {
        DONE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done4(&self) -> DONE4_R {
        DONE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done5(&self) -> DONE5_R {
        DONE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done6(&self) -> DONE6_R {
        DONE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - These bits mirror the DONE status flags that appear in the result register for each A/D channel."]
    #[inline(always)]
    pub fn done7(&self) -> DONE7_R {
        DONE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> ADINT_R {
        ADINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
