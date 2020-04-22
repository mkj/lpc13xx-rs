#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Interrupt Status"]
    pub devintst: DEVINTST,
    #[doc = "0x04 - USB Device Interrupt Enable"]
    pub devinten: DEVINTEN,
    #[doc = "0x08 - USB Device Interrupt Clear"]
    pub devintctrl: DEVINTCTRL,
    #[doc = "0x0c - USB Device Interrupt Set"]
    pub devintset: DEVINTSET,
    #[doc = "0x10 - USB Command Code"]
    pub cmdcode: CMDCODE,
    #[doc = "0x14 - USB Command Data"]
    pub cmddata: CMDDATA,
    #[doc = "0x18 - USB Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x1c - USB Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x20 - USB Receive Packet Length"]
    pub rxplen: RXPLEN,
    #[doc = "0x24 - USB Transmit Packet Length"]
    pub txplenn: TXPLENN,
    #[doc = "0x28 - USB Control"]
    pub ctrl: CTRL,
    #[doc = "0x2c - USB Device FIQ select"]
    pub devfiqsel: DEVFIQSEL,
}
#[doc = "USB Device Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintst](devintst) module"]
pub type DEVINTST = crate::Reg<u32, _DEVINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTST;
#[doc = "`read()` method returns [devintst::R](devintst::R) reader structure"]
impl crate::Readable for DEVINTST {}
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "USB Device Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinten](devinten) module"]
pub type DEVINTEN = crate::Reg<u32, _DEVINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTEN;
#[doc = "`read()` method returns [devinten::R](devinten::R) reader structure"]
impl crate::Readable for DEVINTEN {}
#[doc = "`write(|w| ..)` method takes [devinten::W](devinten::W) writer structure"]
impl crate::Writable for DEVINTEN {}
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "USB Device Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintctrl](devintctrl) module"]
pub type DEVINTCTRL = crate::Reg<u32, _DEVINTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTCTRL;
#[doc = "`write(|w| ..)` method takes [devintctrl::W](devintctrl::W) writer structure"]
impl crate::Writable for DEVINTCTRL {}
#[doc = "USB Device Interrupt Clear"]
pub mod devintctrl;
#[doc = "USB Device Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintset](devintset) module"]
pub type DEVINTSET = crate::Reg<u32, _DEVINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTSET;
#[doc = "`write(|w| ..)` method takes [devintset::W](devintset::W) writer structure"]
impl crate::Writable for DEVINTSET {}
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "USB Command Code\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdcode](cmdcode) module"]
pub type CMDCODE = crate::Reg<u32, _CMDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDCODE;
#[doc = "`write(|w| ..)` method takes [cmdcode::W](cmdcode::W) writer structure"]
impl crate::Writable for CMDCODE {}
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "USB Command Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmddata](cmddata) module"]
pub type CMDDATA = crate::Reg<u32, _CMDDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDDATA;
#[doc = "`read()` method returns [cmddata::R](cmddata::R) reader structure"]
impl crate::Readable for CMDDATA {}
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "USB Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](rxdata) module"]
pub type RXDATA = crate::Reg<u32, _RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATA;
#[doc = "`read()` method returns [rxdata::R](rxdata::R) reader structure"]
impl crate::Readable for RXDATA {}
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "USB Transmit Data\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](txdata) module"]
pub type TXDATA = crate::Reg<u32, _TXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATA;
#[doc = "`write(|w| ..)` method takes [txdata::W](txdata::W) writer structure"]
impl crate::Writable for TXDATA {}
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "USB Receive Packet Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxplen](rxplen) module"]
pub type RXPLEN = crate::Reg<u32, _RXPLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPLEN;
#[doc = "`read()` method returns [rxplen::R](rxplen::R) reader structure"]
impl crate::Readable for RXPLEN {}
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "USB Transmit Packet Length\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txplenn](txplenn) module"]
pub type TXPLENN = crate::Reg<u32, _TXPLENN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPLENN;
#[doc = "`write(|w| ..)` method takes [txplenn::W](txplenn::W) writer structure"]
impl crate::Writable for TXPLENN {}
#[doc = "USB Transmit Packet Length"]
pub mod txplenn;
#[doc = "USB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "USB Device FIQ select\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devfiqsel](devfiqsel) module"]
pub type DEVFIQSEL = crate::Reg<u32, _DEVFIQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVFIQSEL;
#[doc = "`write(|w| ..)` method takes [devfiqsel::W](devfiqsel::W) writer structure"]
impl crate::Writable for DEVFIQSEL {}
#[doc = "USB Device FIQ select"]
pub mod devfiqsel;
