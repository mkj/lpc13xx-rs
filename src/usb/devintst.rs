#[doc = "Reader of register DEVINTST"]
pub type R = crate::R<u32, super::DEVINTST>;
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0`"]
pub type EP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1`"]
pub type EP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2`"]
pub type EP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3`"]
pub type EP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4`"]
pub type EP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5`"]
pub type EP5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6`"]
pub type EP6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7`"]
pub type EP7_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEV_STAT`"]
pub type DEV_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CC_EMPTY`"]
pub type CC_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CD_FULL`"]
pub type CD_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RxENDPKT`"]
pub type RXENDPKT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TxENDPKT`"]
pub type TXENDPKT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. Refer to Section 10.11.7. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DEV_STAT_R {
        DEV_STAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn cc_empty(&self) -> CC_EMPTY_R {
        CC_EMPTY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn cd_full(&self) -> CD_FULL_R {
        CD_FULL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RXENDPKT_R {
        RXENDPKT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TXENDPKT_R {
        TXENDPKT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
