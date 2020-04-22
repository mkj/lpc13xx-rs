#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin PIO2_6"]
    pub pio2_6: PIO2_6,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - I/O configuration for pin PIO2_0/DTR/SSEL1"]
    pub pio2_0: PIO2_0,
    #[doc = "0x0c - I/O configuration for pin RESET/PIO0_0"]
    pub reset_pio0_0: RESET_PIO0_0,
    #[doc = "0x10 - I/O configuration for pin PIO0_1/CLKOUT/ CT32B0_MAT2/USB_FTOGGLE"]
    pub pio0_1: PIO0_1,
    #[doc = "0x14 - I/O configuration for pin PIO1_8/CT16B1_CAP0"]
    pub pio1_8: PIO1_8,
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - I/O configuration for pin PIO0_2/SSEL0/ CT16B0_CAP0"]
    pub pio0_2: PIO0_2,
    #[doc = "0x20 - I/O configuration for pin PIO2_7"]
    pub pio2_7: PIO2_7,
    #[doc = "0x24 - I/O configuration for pin PIO2_8"]
    pub pio2_8: PIO2_8,
    #[doc = "0x28 - I/O configuration for pin PIO2_1/DSR/SCK1"]
    pub pio2_1: PIO2_1,
    #[doc = "0x2c - I/O configuration for pin PIO0_3/USB_VBUS"]
    pub pio0_3: PIO0_3,
    #[doc = "0x30 - I/O configuration for pin PIO0_4/SCL"]
    pub pio0_4: PIO0_4,
    #[doc = "0x34 - I/O configuration for pin PIO0_5/SDA"]
    pub pio0_5: PIO0_5,
    #[doc = "0x38 - I/O configuration for pin PIO1_9/CT16B1_MAT0"]
    pub pio1_9: PIO1_9,
    #[doc = "0x3c - I/O configuration for pin PIO3_4"]
    pub pio3_4: PIO3_4,
    #[doc = "0x40 - I/O configuration for pin PIO2_4"]
    pub pio2_4: PIO2_4,
    #[doc = "0x44 - I/O configuration for pin PIO2_5"]
    pub pio2_5: PIO2_5,
    #[doc = "0x48 - I/O configuration for pin PIO3_5"]
    pub pio3_5: PIO3_5,
    #[doc = "0x4c - I/O configuration for pin PIO0_6/USB_CONNECT/SCK"]
    pub pio0_6: PIO0_6,
    #[doc = "0x50 - I/O configuration for pin PIO0_7/CTS"]
    pub pio0_7: PIO0_7,
    #[doc = "0x54 - I/O configuration for pin PIO2_9"]
    pub pio2_9: PIO2_9,
    #[doc = "0x58 - I/O configuration for pin PIO2_10"]
    pub pio2_10: PIO2_10,
    #[doc = "0x5c - I/O configuration for pin PIO2_2/DCD/MISO1"]
    pub pio2_2: PIO2_2,
    #[doc = "0x60 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    pub pio0_8: PIO0_8,
    #[doc = "0x64 - I/O configuration for pin PIO0_9/MOSI0/ CT16B0_MAT1/SWO"]
    pub pio0_9: PIO0_9,
    #[doc = "0x68 - I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2"]
    pub swclk_pio0_10: SWCLK_PIO0_10,
    #[doc = "0x6c - I/O configuration for pin PIO1_10/AD6/ CT16B1_MAT1"]
    pub pio1_10: PIO1_10,
    #[doc = "0x70 - I/O configuration for pin PIO2_11/SCK"]
    pub pio2_11: PIO2_11,
    #[doc = "0x74 - I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
    pub r_pio0_11: R_PIO0_11,
    #[doc = "0x78 - I/O configuration for pin R/PIO1_0/AD1/ CT32B1_CAP0"]
    pub r_pio1_0: R_PIO1_0,
    #[doc = "0x7c - I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
    pub r_pio1_1: R_PIO1_1,
    #[doc = "0x80 - I/O configuration for pin R/PIO1_2/AD3/ CT32B1_MAT1"]
    pub r_pio1_2: R_PIO1_2,
    #[doc = "0x84 - I/O configuration for pin PIO3_0/DTR"]
    pub pio3_0: PIO3_0,
    #[doc = "0x88 - I/O configuration for pin PIO3_1/DSR"]
    pub pio3_1: PIO3_1,
    #[doc = "0x8c - I/O configuration for pin PIO2_3/RI/MOSI1"]
    pub pio2_3: PIO2_3,
    #[doc = "0x90 - I/O configuration for pin SWDIO/PIO1_3/AD4/ CT32B1_MAT2"]
    pub swdio_pio1_3: SWDIO_PIO1_3,
    #[doc = "0x94 - I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
    pub pio1_4: PIO1_4,
    #[doc = "0x98 - I/O configuration for pin PIO1_11/AD7"]
    pub pio1_11: PIO1_11,
    #[doc = "0x9c - I/O configuration for pin PIO3_2/DCD"]
    pub pio3_2: PIO3_2,
    #[doc = "0xa0 - I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
    pub pio1_5: PIO1_5,
    #[doc = "0xa4 - I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
    pub pio1_6: PIO1_6,
    #[doc = "0xa8 - I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
    pub pio1_7: PIO1_7,
    #[doc = "0xac - I/O configuration for pin PIO3_3/RI"]
    pub pio3_3: PIO3_3,
    #[doc = "0xb0 - SCK0 pin location register"]
    pub sck0_loc: SCK0_LOC,
    #[doc = "0xb4 - DSR pin location select register"]
    pub dsr_loc: DSR_LOC,
    #[doc = "0xb8 - DCD pin location select register"]
    pub dcd_loc: DCD_LOC,
    #[doc = "0xbc - RI pin location register"]
    pub ri_loc: RI_LOC,
}
#[doc = "I/O configuration for pin PIO2_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_6](pio2_6) module"]
pub type PIO2_6 = crate::Reg<u32, _PIO2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_6;
#[doc = "`read()` method returns [pio2_6::R](pio2_6::R) reader structure"]
impl crate::Readable for PIO2_6 {}
#[doc = "`write(|w| ..)` method takes [pio2_6::W](pio2_6::W) writer structure"]
impl crate::Writable for PIO2_6 {}
#[doc = "I/O configuration for pin PIO2_6"]
pub mod pio2_6;
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_0](pio2_0) module"]
pub type PIO2_0 = crate::Reg<u32, _PIO2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_0;
#[doc = "`read()` method returns [pio2_0::R](pio2_0::R) reader structure"]
impl crate::Readable for PIO2_0 {}
#[doc = "`write(|w| ..)` method takes [pio2_0::W](pio2_0::W) writer structure"]
impl crate::Writable for PIO2_0 {}
#[doc = "I/O configuration for pin PIO2_0/DTR/SSEL1"]
pub mod pio2_0;
#[doc = "I/O configuration for pin RESET/PIO0_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_pio0_0](reset_pio0_0) module"]
pub type RESET_PIO0_0 = crate::Reg<u32, _RESET_PIO0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_PIO0_0;
#[doc = "`read()` method returns [reset_pio0_0::R](reset_pio0_0::R) reader structure"]
impl crate::Readable for RESET_PIO0_0 {}
#[doc = "`write(|w| ..)` method takes [reset_pio0_0::W](reset_pio0_0::W) writer structure"]
impl crate::Writable for RESET_PIO0_0 {}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/ CT32B0_MAT2/USB_FTOGGLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_1](pio0_1) module"]
pub type PIO0_1 = crate::Reg<u32, _PIO0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_1;
#[doc = "`read()` method returns [pio0_1::R](pio0_1::R) reader structure"]
impl crate::Readable for PIO0_1 {}
#[doc = "`write(|w| ..)` method takes [pio0_1::W](pio0_1::W) writer structure"]
impl crate::Writable for PIO0_1 {}
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/ CT32B0_MAT2/USB_FTOGGLE"]
pub mod pio0_1;
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_8](pio1_8) module"]
pub type PIO1_8 = crate::Reg<u32, _PIO1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_8;
#[doc = "`read()` method returns [pio1_8::R](pio1_8::R) reader structure"]
impl crate::Readable for PIO1_8 {}
#[doc = "`write(|w| ..)` method takes [pio1_8::W](pio1_8::W) writer structure"]
impl crate::Writable for PIO1_8 {}
#[doc = "I/O configuration for pin PIO1_8/CT16B1_CAP0"]
pub mod pio1_8;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/ CT16B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_2](pio0_2) module"]
pub type PIO0_2 = crate::Reg<u32, _PIO0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_2;
#[doc = "`read()` method returns [pio0_2::R](pio0_2::R) reader structure"]
impl crate::Readable for PIO0_2 {}
#[doc = "`write(|w| ..)` method takes [pio0_2::W](pio0_2::W) writer structure"]
impl crate::Writable for PIO0_2 {}
#[doc = "I/O configuration for pin PIO0_2/SSEL0/ CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "I/O configuration for pin PIO2_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_7](pio2_7) module"]
pub type PIO2_7 = crate::Reg<u32, _PIO2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_7;
#[doc = "`read()` method returns [pio2_7::R](pio2_7::R) reader structure"]
impl crate::Readable for PIO2_7 {}
#[doc = "`write(|w| ..)` method takes [pio2_7::W](pio2_7::W) writer structure"]
impl crate::Writable for PIO2_7 {}
#[doc = "I/O configuration for pin PIO2_7"]
pub mod pio2_7;
#[doc = "I/O configuration for pin PIO2_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_8](pio2_8) module"]
pub type PIO2_8 = crate::Reg<u32, _PIO2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_8;
#[doc = "`read()` method returns [pio2_8::R](pio2_8::R) reader structure"]
impl crate::Readable for PIO2_8 {}
#[doc = "`write(|w| ..)` method takes [pio2_8::W](pio2_8::W) writer structure"]
impl crate::Writable for PIO2_8 {}
#[doc = "I/O configuration for pin PIO2_8"]
pub mod pio2_8;
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_1](pio2_1) module"]
pub type PIO2_1 = crate::Reg<u32, _PIO2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_1;
#[doc = "`read()` method returns [pio2_1::R](pio2_1::R) reader structure"]
impl crate::Readable for PIO2_1 {}
#[doc = "`write(|w| ..)` method takes [pio2_1::W](pio2_1::W) writer structure"]
impl crate::Writable for PIO2_1 {}
#[doc = "I/O configuration for pin PIO2_1/DSR/SCK1"]
pub mod pio2_1;
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_3](pio0_3) module"]
pub type PIO0_3 = crate::Reg<u32, _PIO0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_3;
#[doc = "`read()` method returns [pio0_3::R](pio0_3::R) reader structure"]
impl crate::Readable for PIO0_3 {}
#[doc = "`write(|w| ..)` method takes [pio0_3::W](pio0_3::W) writer structure"]
impl crate::Writable for PIO0_3 {}
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub mod pio0_3;
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_4](pio0_4) module"]
pub type PIO0_4 = crate::Reg<u32, _PIO0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_4;
#[doc = "`read()` method returns [pio0_4::R](pio0_4::R) reader structure"]
impl crate::Readable for PIO0_4 {}
#[doc = "`write(|w| ..)` method takes [pio0_4::W](pio0_4::W) writer structure"]
impl crate::Writable for PIO0_4 {}
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "I/O configuration for pin PIO0_5/SDA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_5](pio0_5) module"]
pub type PIO0_5 = crate::Reg<u32, _PIO0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_5;
#[doc = "`read()` method returns [pio0_5::R](pio0_5::R) reader structure"]
impl crate::Readable for PIO0_5 {}
#[doc = "`write(|w| ..)` method takes [pio0_5::W](pio0_5::W) writer structure"]
impl crate::Writable for PIO0_5 {}
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_9](pio1_9) module"]
pub type PIO1_9 = crate::Reg<u32, _PIO1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_9;
#[doc = "`read()` method returns [pio1_9::R](pio1_9::R) reader structure"]
impl crate::Readable for PIO1_9 {}
#[doc = "`write(|w| ..)` method takes [pio1_9::W](pio1_9::W) writer structure"]
impl crate::Writable for PIO1_9 {}
#[doc = "I/O configuration for pin PIO1_9/CT16B1_MAT0"]
pub mod pio1_9;
#[doc = "I/O configuration for pin PIO3_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_4](pio3_4) module"]
pub type PIO3_4 = crate::Reg<u32, _PIO3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_4;
#[doc = "`read()` method returns [pio3_4::R](pio3_4::R) reader structure"]
impl crate::Readable for PIO3_4 {}
#[doc = "`write(|w| ..)` method takes [pio3_4::W](pio3_4::W) writer structure"]
impl crate::Writable for PIO3_4 {}
#[doc = "I/O configuration for pin PIO3_4"]
pub mod pio3_4;
#[doc = "I/O configuration for pin PIO2_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_4](pio2_4) module"]
pub type PIO2_4 = crate::Reg<u32, _PIO2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_4;
#[doc = "`read()` method returns [pio2_4::R](pio2_4::R) reader structure"]
impl crate::Readable for PIO2_4 {}
#[doc = "`write(|w| ..)` method takes [pio2_4::W](pio2_4::W) writer structure"]
impl crate::Writable for PIO2_4 {}
#[doc = "I/O configuration for pin PIO2_4"]
pub mod pio2_4;
#[doc = "I/O configuration for pin PIO2_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_5](pio2_5) module"]
pub type PIO2_5 = crate::Reg<u32, _PIO2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_5;
#[doc = "`read()` method returns [pio2_5::R](pio2_5::R) reader structure"]
impl crate::Readable for PIO2_5 {}
#[doc = "`write(|w| ..)` method takes [pio2_5::W](pio2_5::W) writer structure"]
impl crate::Writable for PIO2_5 {}
#[doc = "I/O configuration for pin PIO2_5"]
pub mod pio2_5;
#[doc = "I/O configuration for pin PIO3_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_5](pio3_5) module"]
pub type PIO3_5 = crate::Reg<u32, _PIO3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_5;
#[doc = "`read()` method returns [pio3_5::R](pio3_5::R) reader structure"]
impl crate::Readable for PIO3_5 {}
#[doc = "`write(|w| ..)` method takes [pio3_5::W](pio3_5::W) writer structure"]
impl crate::Writable for PIO3_5 {}
#[doc = "I/O configuration for pin PIO3_5"]
pub mod pio3_5;
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_6](pio0_6) module"]
pub type PIO0_6 = crate::Reg<u32, _PIO0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_6;
#[doc = "`read()` method returns [pio0_6::R](pio0_6::R) reader structure"]
impl crate::Readable for PIO0_6 {}
#[doc = "`write(|w| ..)` method takes [pio0_6::W](pio0_6::W) writer structure"]
impl crate::Writable for PIO0_6 {}
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK"]
pub mod pio0_6;
#[doc = "I/O configuration for pin PIO0_7/CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_7](pio0_7) module"]
pub type PIO0_7 = crate::Reg<u32, _PIO0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_7;
#[doc = "`read()` method returns [pio0_7::R](pio0_7::R) reader structure"]
impl crate::Readable for PIO0_7 {}
#[doc = "`write(|w| ..)` method takes [pio0_7::W](pio0_7::W) writer structure"]
impl crate::Writable for PIO0_7 {}
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "I/O configuration for pin PIO2_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_9](pio2_9) module"]
pub type PIO2_9 = crate::Reg<u32, _PIO2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_9;
#[doc = "`read()` method returns [pio2_9::R](pio2_9::R) reader structure"]
impl crate::Readable for PIO2_9 {}
#[doc = "`write(|w| ..)` method takes [pio2_9::W](pio2_9::W) writer structure"]
impl crate::Writable for PIO2_9 {}
#[doc = "I/O configuration for pin PIO2_9"]
pub mod pio2_9;
#[doc = "I/O configuration for pin PIO2_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_10](pio2_10) module"]
pub type PIO2_10 = crate::Reg<u32, _PIO2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_10;
#[doc = "`read()` method returns [pio2_10::R](pio2_10::R) reader structure"]
impl crate::Readable for PIO2_10 {}
#[doc = "`write(|w| ..)` method takes [pio2_10::W](pio2_10::W) writer structure"]
impl crate::Writable for PIO2_10 {}
#[doc = "I/O configuration for pin PIO2_10"]
pub mod pio2_10;
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_2](pio2_2) module"]
pub type PIO2_2 = crate::Reg<u32, _PIO2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_2;
#[doc = "`read()` method returns [pio2_2::R](pio2_2::R) reader structure"]
impl crate::Readable for PIO2_2 {}
#[doc = "`write(|w| ..)` method takes [pio2_2::W](pio2_2::W) writer structure"]
impl crate::Writable for PIO2_2 {}
#[doc = "I/O configuration for pin PIO2_2/DCD/MISO1"]
pub mod pio2_2;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_8](pio0_8) module"]
pub type PIO0_8 = crate::Reg<u32, _PIO0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_8;
#[doc = "`read()` method returns [pio0_8::R](pio0_8::R) reader structure"]
impl crate::Readable for PIO0_8 {}
#[doc = "`write(|w| ..)` method takes [pio0_8::W](pio0_8::W) writer structure"]
impl crate::Writable for PIO0_8 {}
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/ CT16B0_MAT1/SWO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_9](pio0_9) module"]
pub type PIO0_9 = crate::Reg<u32, _PIO0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_9;
#[doc = "`read()` method returns [pio0_9::R](pio0_9::R) reader structure"]
impl crate::Readable for PIO0_9 {}
#[doc = "`write(|w| ..)` method takes [pio0_9::W](pio0_9::W) writer structure"]
impl crate::Writable for PIO0_9 {}
#[doc = "I/O configuration for pin PIO0_9/MOSI0/ CT16B0_MAT1/SWO"]
pub mod pio0_9;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swclk_pio0_10](swclk_pio0_10) module"]
pub type SWCLK_PIO0_10 = crate::Reg<u32, _SWCLK_PIO0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWCLK_PIO0_10;
#[doc = "`read()` method returns [swclk_pio0_10::R](swclk_pio0_10::R) reader structure"]
impl crate::Readable for SWCLK_PIO0_10 {}
#[doc = "`write(|w| ..)` method takes [swclk_pio0_10::W](swclk_pio0_10::W) writer structure"]
impl crate::Writable for SWCLK_PIO0_10 {}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "I/O configuration for pin PIO1_10/AD6/ CT16B1_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_10](pio1_10) module"]
pub type PIO1_10 = crate::Reg<u32, _PIO1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_10;
#[doc = "`read()` method returns [pio1_10::R](pio1_10::R) reader structure"]
impl crate::Readable for PIO1_10 {}
#[doc = "`write(|w| ..)` method takes [pio1_10::W](pio1_10::W) writer structure"]
impl crate::Writable for PIO1_10 {}
#[doc = "I/O configuration for pin PIO1_10/AD6/ CT16B1_MAT1"]
pub mod pio1_10;
#[doc = "I/O configuration for pin PIO2_11/SCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_11](pio2_11) module"]
pub type PIO2_11 = crate::Reg<u32, _PIO2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_11;
#[doc = "`read()` method returns [pio2_11::R](pio2_11::R) reader structure"]
impl crate::Readable for PIO2_11 {}
#[doc = "`write(|w| ..)` method takes [pio2_11::W](pio2_11::W) writer structure"]
impl crate::Writable for PIO2_11 {}
#[doc = "I/O configuration for pin PIO2_11/SCK"]
pub mod pio2_11;
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r_pio0_11](r_pio0_11) module"]
pub type R_PIO0_11 = crate::Reg<u32, _R_PIO0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R_PIO0_11;
#[doc = "`read()` method returns [r_pio0_11::R](r_pio0_11::R) reader structure"]
impl crate::Readable for R_PIO0_11 {}
#[doc = "`write(|w| ..)` method takes [r_pio0_11::W](r_pio0_11::W) writer structure"]
impl crate::Writable for R_PIO0_11 {}
#[doc = "I/O configuration for pin R/PIO0_11/AD0/CT32B0_MAT3"]
pub mod r_pio0_11;
#[doc = "I/O configuration for pin R/PIO1_0/AD1/ CT32B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r_pio1_0](r_pio1_0) module"]
pub type R_PIO1_0 = crate::Reg<u32, _R_PIO1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R_PIO1_0;
#[doc = "`read()` method returns [r_pio1_0::R](r_pio1_0::R) reader structure"]
impl crate::Readable for R_PIO1_0 {}
#[doc = "`write(|w| ..)` method takes [r_pio1_0::W](r_pio1_0::W) writer structure"]
impl crate::Writable for R_PIO1_0 {}
#[doc = "I/O configuration for pin R/PIO1_0/AD1/ CT32B1_CAP0"]
pub mod r_pio1_0;
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r_pio1_1](r_pio1_1) module"]
pub type R_PIO1_1 = crate::Reg<u32, _R_PIO1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R_PIO1_1;
#[doc = "`read()` method returns [r_pio1_1::R](r_pio1_1::R) reader structure"]
impl crate::Readable for R_PIO1_1 {}
#[doc = "`write(|w| ..)` method takes [r_pio1_1::W](r_pio1_1::W) writer structure"]
impl crate::Writable for R_PIO1_1 {}
#[doc = "I/O configuration for pin R/PIO1_1/AD2/CT32B1_MAT0"]
pub mod r_pio1_1;
#[doc = "I/O configuration for pin R/PIO1_2/AD3/ CT32B1_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r_pio1_2](r_pio1_2) module"]
pub type R_PIO1_2 = crate::Reg<u32, _R_PIO1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R_PIO1_2;
#[doc = "`read()` method returns [r_pio1_2::R](r_pio1_2::R) reader structure"]
impl crate::Readable for R_PIO1_2 {}
#[doc = "`write(|w| ..)` method takes [r_pio1_2::W](r_pio1_2::W) writer structure"]
impl crate::Writable for R_PIO1_2 {}
#[doc = "I/O configuration for pin R/PIO1_2/AD3/ CT32B1_MAT1"]
pub mod r_pio1_2;
#[doc = "I/O configuration for pin PIO3_0/DTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_0](pio3_0) module"]
pub type PIO3_0 = crate::Reg<u32, _PIO3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_0;
#[doc = "`read()` method returns [pio3_0::R](pio3_0::R) reader structure"]
impl crate::Readable for PIO3_0 {}
#[doc = "`write(|w| ..)` method takes [pio3_0::W](pio3_0::W) writer structure"]
impl crate::Writable for PIO3_0 {}
#[doc = "I/O configuration for pin PIO3_0/DTR"]
pub mod pio3_0;
#[doc = "I/O configuration for pin PIO3_1/DSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_1](pio3_1) module"]
pub type PIO3_1 = crate::Reg<u32, _PIO3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_1;
#[doc = "`read()` method returns [pio3_1::R](pio3_1::R) reader structure"]
impl crate::Readable for PIO3_1 {}
#[doc = "`write(|w| ..)` method takes [pio3_1::W](pio3_1::W) writer structure"]
impl crate::Writable for PIO3_1 {}
#[doc = "I/O configuration for pin PIO3_1/DSR"]
pub mod pio3_1;
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio2_3](pio2_3) module"]
pub type PIO2_3 = crate::Reg<u32, _PIO2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO2_3;
#[doc = "`read()` method returns [pio2_3::R](pio2_3::R) reader structure"]
impl crate::Readable for PIO2_3 {}
#[doc = "`write(|w| ..)` method takes [pio2_3::W](pio2_3::W) writer structure"]
impl crate::Writable for PIO2_3 {}
#[doc = "I/O configuration for pin PIO2_3/RI/MOSI1"]
pub mod pio2_3;
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/ CT32B1_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdio_pio1_3](swdio_pio1_3) module"]
pub type SWDIO_PIO1_3 = crate::Reg<u32, _SWDIO_PIO1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWDIO_PIO1_3;
#[doc = "`read()` method returns [swdio_pio1_3::R](swdio_pio1_3::R) reader structure"]
impl crate::Readable for SWDIO_PIO1_3 {}
#[doc = "`write(|w| ..)` method takes [swdio_pio1_3::W](swdio_pio1_3::W) writer structure"]
impl crate::Writable for SWDIO_PIO1_3 {}
#[doc = "I/O configuration for pin SWDIO/PIO1_3/AD4/ CT32B1_MAT2"]
pub mod swdio_pio1_3;
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_4](pio1_4) module"]
pub type PIO1_4 = crate::Reg<u32, _PIO1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_4;
#[doc = "`read()` method returns [pio1_4::R](pio1_4::R) reader structure"]
impl crate::Readable for PIO1_4 {}
#[doc = "`write(|w| ..)` method takes [pio1_4::W](pio1_4::W) writer structure"]
impl crate::Writable for PIO1_4 {}
#[doc = "I/O configuration for pin PIO1_4/AD5/CT32B1_MAT3"]
pub mod pio1_4;
#[doc = "I/O configuration for pin PIO1_11/AD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_11](pio1_11) module"]
pub type PIO1_11 = crate::Reg<u32, _PIO1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_11;
#[doc = "`read()` method returns [pio1_11::R](pio1_11::R) reader structure"]
impl crate::Readable for PIO1_11 {}
#[doc = "`write(|w| ..)` method takes [pio1_11::W](pio1_11::W) writer structure"]
impl crate::Writable for PIO1_11 {}
#[doc = "I/O configuration for pin PIO1_11/AD7"]
pub mod pio1_11;
#[doc = "I/O configuration for pin PIO3_2/DCD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_2](pio3_2) module"]
pub type PIO3_2 = crate::Reg<u32, _PIO3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_2;
#[doc = "`read()` method returns [pio3_2::R](pio3_2::R) reader structure"]
impl crate::Readable for PIO3_2 {}
#[doc = "`write(|w| ..)` method takes [pio3_2::W](pio3_2::W) writer structure"]
impl crate::Writable for PIO3_2 {}
#[doc = "I/O configuration for pin PIO3_2/DCD"]
pub mod pio3_2;
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_5](pio1_5) module"]
pub type PIO1_5 = crate::Reg<u32, _PIO1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_5;
#[doc = "`read()` method returns [pio1_5::R](pio1_5::R) reader structure"]
impl crate::Readable for PIO1_5 {}
#[doc = "`write(|w| ..)` method takes [pio1_5::W](pio1_5::W) writer structure"]
impl crate::Writable for PIO1_5 {}
#[doc = "I/O configuration for pin PIO1_5/RTS/CT32B0_CAP0"]
pub mod pio1_5;
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_6](pio1_6) module"]
pub type PIO1_6 = crate::Reg<u32, _PIO1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_6;
#[doc = "`read()` method returns [pio1_6::R](pio1_6::R) reader structure"]
impl crate::Readable for PIO1_6 {}
#[doc = "`write(|w| ..)` method takes [pio1_6::W](pio1_6::W) writer structure"]
impl crate::Writable for PIO1_6 {}
#[doc = "I/O configuration for pin PIO1_6/RXD/CT32B0_MAT0"]
pub mod pio1_6;
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_7](pio1_7) module"]
pub type PIO1_7 = crate::Reg<u32, _PIO1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_7;
#[doc = "`read()` method returns [pio1_7::R](pio1_7::R) reader structure"]
impl crate::Readable for PIO1_7 {}
#[doc = "`write(|w| ..)` method takes [pio1_7::W](pio1_7::W) writer structure"]
impl crate::Writable for PIO1_7 {}
#[doc = "I/O configuration for pin PIO1_7/TXD/CT32B0_MAT1"]
pub mod pio1_7;
#[doc = "I/O configuration for pin PIO3_3/RI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio3_3](pio3_3) module"]
pub type PIO3_3 = crate::Reg<u32, _PIO3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO3_3;
#[doc = "`read()` method returns [pio3_3::R](pio3_3::R) reader structure"]
impl crate::Readable for PIO3_3 {}
#[doc = "`write(|w| ..)` method takes [pio3_3::W](pio3_3::W) writer structure"]
impl crate::Writable for PIO3_3 {}
#[doc = "I/O configuration for pin PIO3_3/RI"]
pub mod pio3_3;
#[doc = "SCK0 pin location register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sck0_loc](sck0_loc) module"]
pub type SCK0_LOC = crate::Reg<u32, _SCK0_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCK0_LOC;
#[doc = "`read()` method returns [sck0_loc::R](sck0_loc::R) reader structure"]
impl crate::Readable for SCK0_LOC {}
#[doc = "`write(|w| ..)` method takes [sck0_loc::W](sck0_loc::W) writer structure"]
impl crate::Writable for SCK0_LOC {}
#[doc = "SCK0 pin location register"]
pub mod sck0_loc;
#[doc = "DSR pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsr_loc](dsr_loc) module"]
pub type DSR_LOC = crate::Reg<u32, _DSR_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSR_LOC;
#[doc = "`read()` method returns [dsr_loc::R](dsr_loc::R) reader structure"]
impl crate::Readable for DSR_LOC {}
#[doc = "`write(|w| ..)` method takes [dsr_loc::W](dsr_loc::W) writer structure"]
impl crate::Writable for DSR_LOC {}
#[doc = "DSR pin location select register"]
pub mod dsr_loc;
#[doc = "DCD pin location select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcd_loc](dcd_loc) module"]
pub type DCD_LOC = crate::Reg<u32, _DCD_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCD_LOC;
#[doc = "`read()` method returns [dcd_loc::R](dcd_loc::R) reader structure"]
impl crate::Readable for DCD_LOC {}
#[doc = "`write(|w| ..)` method takes [dcd_loc::W](dcd_loc::W) writer structure"]
impl crate::Writable for DCD_LOC {}
#[doc = "DCD pin location select register"]
pub mod dcd_loc;
#[doc = "RI pin location register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ri_loc](ri_loc) module"]
pub type RI_LOC = crate::Reg<u32, _RI_LOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RI_LOC;
#[doc = "`read()` method returns [ri_loc::R](ri_loc::R) reader structure"]
impl crate::Readable for RI_LOC {}
#[doc = "`write(|w| ..)` method takes [ri_loc::W](ri_loc::W) writer structure"]
impl crate::Writable for RI_LOC {}
#[doc = "RI pin location register"]
pub mod ri_loc;
