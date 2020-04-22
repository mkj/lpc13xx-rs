#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending."]
    pub bir: BIR,
    #[doc = "0x04 - Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
    pub btcr: BTCR,
    #[doc = "0x08 - Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
    pub btc: BTC,
    #[doc = "0x0c - Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC."]
    pub bpr: BPR,
    #[doc = "0x10 - Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
    pub bpc: BPC,
    #[doc = "0x14 - Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
    pub bmcr: BMCR,
    #[doc = "0x18 - Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC."]
    pub bmr: [BMR; 4],
    #[doc = "0x28 - Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
    pub bccr: BCCR,
    #[doc = "0x2c - Capture Register 0 (CR0). CR0 is loaded with the value of TC when there is an event on the CT32B0_CAP0 input."]
    pub bcr0: BCR0,
    _reserved9: [u8; 12usize],
    #[doc = "0x3c - External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\]."]
    pub bemr: BEMR,
    _reserved10: [u8; 48usize],
    #[doc = "0x70 - Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub bctcr: BCTCR,
    #[doc = "0x74 - PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\]."]
    pub bpwmc: BPWMC,
}
#[doc = "Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bir](bir) module"]
pub type BIR = crate::Reg<u32, _BIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIR;
#[doc = "`read()` method returns [bir::R](bir::R) reader structure"]
impl crate::Readable for BIR {}
#[doc = "`write(|w| ..)` method takes [bir::W](bir::W) writer structure"]
impl crate::Writable for BIR {}
#[doc = "Interrupt Register (IR). The IR can be written to clear interrupts. The IR can be read to identify which of five possible interrupt sources are pending."]
pub mod bir;
#[doc = "Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btcr](btcr) module"]
pub type BTCR = crate::Reg<u32, _BTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTCR;
#[doc = "`read()` method returns [btcr::R](btcr::R) reader structure"]
impl crate::Readable for BTCR {}
#[doc = "`write(|w| ..)` method takes [btcr::W](btcr::W) writer structure"]
impl crate::Writable for BTCR {}
#[doc = "Timer Control Register (TCR). The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
pub mod btcr;
#[doc = "Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btc](btc) module"]
pub type BTC = crate::Reg<u32, _BTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTC;
#[doc = "`read()` method returns [btc::R](btc::R) reader structure"]
impl crate::Readable for BTC {}
#[doc = "`write(|w| ..)` method takes [btc::W](btc::W) writer structure"]
impl crate::Writable for BTC {}
#[doc = "Timer Counter (TC). The 32-bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod btc;
#[doc = "Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpr](bpr) module"]
pub type BPR = crate::Reg<u32, _BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPR;
#[doc = "`read()` method returns [bpr::R](bpr::R) reader structure"]
impl crate::Readable for BPR {}
#[doc = "`write(|w| ..)` method takes [bpr::W](bpr::W) writer structure"]
impl crate::Writable for BPR {}
#[doc = "Prescale Register (PR). When the Prescale Counter (below) is equal to this value, the next clock increments the TC and clears the PC."]
pub mod bpr;
#[doc = "Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpc](bpc) module"]
pub type BPC = crate::Reg<u32, _BPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPC;
#[doc = "`read()` method returns [bpc::R](bpc::R) reader structure"]
impl crate::Readable for BPC {}
#[doc = "`write(|w| ..)` method takes [bpc::W](bpc::W) writer structure"]
impl crate::Writable for BPC {}
#[doc = "Prescale Counter (PC). The 32-bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
pub mod bpc;
#[doc = "Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr](bmcr) module"]
pub type BMCR = crate::Reg<u32, _BMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCR;
#[doc = "`read()` method returns [bmcr::R](bmcr::R) reader structure"]
impl crate::Readable for BMCR {}
#[doc = "`write(|w| ..)` method takes [bmcr::W](bmcr::W) writer structure"]
impl crate::Writable for BMCR {}
#[doc = "Match Control Register (MCR). The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
pub mod bmcr;
#[doc = "Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](bmr) module"]
pub type BMR = crate::Reg<u32, _BMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMR;
#[doc = "`read()` method returns [bmr::R](bmr::R) reader structure"]
impl crate::Readable for BMR {}
#[doc = "`write(|w| ..)` method takes [bmr::W](bmr::W) writer structure"]
impl crate::Writable for BMR {}
#[doc = "Match Register. MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR0 matches the TC."]
pub mod bmr;
#[doc = "Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bccr](bccr) module"]
pub type BCCR = crate::Reg<u32, _BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCCR;
#[doc = "`read()` method returns [bccr::R](bccr::R) reader structure"]
impl crate::Readable for BCCR {}
#[doc = "`write(|w| ..)` method takes [bccr::W](bccr::W) writer structure"]
impl crate::Writable for BCCR {}
#[doc = "Capture Control Register (CCR). The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
pub mod bccr;
#[doc = "Capture Register 0 (CR0). CR0 is loaded with the value of TC when there is an event on the CT32B0_CAP0 input.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr0](bcr0) module"]
pub type BCR0 = crate::Reg<u32, _BCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR0;
#[doc = "`read()` method returns [bcr0::R](bcr0::R) reader structure"]
impl crate::Readable for BCR0 {}
#[doc = "Capture Register 0 (CR0). CR0 is loaded with the value of TC when there is an event on the CT32B0_CAP0 input."]
pub mod bcr0;
#[doc = "External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bemr](bemr) module"]
pub type BEMR = crate::Reg<u32, _BEMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BEMR;
#[doc = "`read()` method returns [bemr::R](bemr::R) reader structure"]
impl crate::Readable for BEMR {}
#[doc = "`write(|w| ..)` method takes [bemr::W](bemr::W) writer structure"]
impl crate::Writable for BEMR {}
#[doc = "External Match Register (EMR). The EMR controls the match function and the external match pins CT32B0_MAT\\[3:0\\]."]
pub mod bemr;
#[doc = "Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bctcr](bctcr) module"]
pub type BCTCR = crate::Reg<u32, _BCTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCTCR;
#[doc = "`read()` method returns [bctcr::R](bctcr::R) reader structure"]
impl crate::Readable for BCTCR {}
#[doc = "`write(|w| ..)` method takes [bctcr::W](bctcr::W) writer structure"]
impl crate::Writable for BCTCR {}
#[doc = "Count Control Register (CTCR). The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod bctcr;
#[doc = "PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpwmc](bpwmc) module"]
pub type BPWMC = crate::Reg<u32, _BPWMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPWMC;
#[doc = "`read()` method returns [bpwmc::R](bpwmc::R) reader structure"]
impl crate::Readable for BPWMC {}
#[doc = "`write(|w| ..)` method takes [bpwmc::W](bpwmc::W) writer structure"]
impl crate::Writable for BPWMC {}
#[doc = "PWM Control Register (PWMCON). The PWMCON enables PWM mode for the external match pins CT32B0_MAT\\[3:0\\]."]
pub mod bpwmc;
