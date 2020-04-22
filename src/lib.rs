#![doc = "Peripheral access API for LPC13XX microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn PIO0_0();
    fn PIO0_1();
    fn PIO0_2();
    fn PIO0_3();
    fn PIO0_4();
    fn PIO0_5();
    fn PIO0_6();
    fn PIO0_7();
    fn PIO0_8();
    fn PIO0_9();
    fn PIO0_10();
    fn PIO0_11();
    fn PIO1_0();
    fn PIO1_1();
    fn PIO1_2();
    fn PIO1_3();
    fn PIO1_4();
    fn PIO1_5();
    fn PIO1_6();
    fn PIO1_7();
    fn PIO1_8();
    fn PIO1_9();
    fn PIO1_10();
    fn PIO1_11();
    fn PIO2_0();
    fn PIO2_1();
    fn PIO2_2();
    fn PIO2_3();
    fn PIO2_4();
    fn PIO2_5();
    fn PIO2_6();
    fn PIO2_7();
    fn PIO2_8();
    fn PIO2_9();
    fn PIO2_10();
    fn PIO2_11();
    fn PIO3_0();
    fn PIO3_1();
    fn PIO3_2();
    fn PIO3_3();
    fn I2C0();
    fn CT16B0();
    fn CT16B1();
    fn CT32B0();
    fn CT32B1();
    fn SSP0();
    fn UART();
    fn USBIRQ();
    fn USBFIQ();
    fn ADC();
    fn WDT();
    fn BOD();
    fn PIO_3();
    fn PIO_2();
    fn PIO_1();
    fn PIO_0();
    fn SSP1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 58] = [
    Vector { _handler: PIO0_0 },
    Vector { _handler: PIO0_1 },
    Vector { _handler: PIO0_2 },
    Vector { _handler: PIO0_3 },
    Vector { _handler: PIO0_4 },
    Vector { _handler: PIO0_5 },
    Vector { _handler: PIO0_6 },
    Vector { _handler: PIO0_7 },
    Vector { _handler: PIO0_8 },
    Vector { _handler: PIO0_9 },
    Vector { _handler: PIO0_10 },
    Vector { _handler: PIO0_11 },
    Vector { _handler: PIO1_0 },
    Vector { _handler: PIO1_1 },
    Vector { _handler: PIO1_2 },
    Vector { _handler: PIO1_3 },
    Vector { _handler: PIO1_4 },
    Vector { _handler: PIO1_5 },
    Vector { _handler: PIO1_6 },
    Vector { _handler: PIO1_7 },
    Vector { _handler: PIO1_8 },
    Vector { _handler: PIO1_9 },
    Vector { _handler: PIO1_10 },
    Vector { _handler: PIO1_11 },
    Vector { _handler: PIO2_0 },
    Vector { _handler: PIO2_1 },
    Vector { _handler: PIO2_2 },
    Vector { _handler: PIO2_3 },
    Vector { _handler: PIO2_4 },
    Vector { _handler: PIO2_5 },
    Vector { _handler: PIO2_6 },
    Vector { _handler: PIO2_7 },
    Vector { _handler: PIO2_8 },
    Vector { _handler: PIO2_9 },
    Vector { _handler: PIO2_10 },
    Vector { _handler: PIO2_11 },
    Vector { _handler: PIO3_0 },
    Vector { _handler: PIO3_1 },
    Vector { _handler: PIO3_2 },
    Vector { _handler: PIO3_3 },
    Vector { _handler: I2C0 },
    Vector { _handler: CT16B0 },
    Vector { _handler: CT16B1 },
    Vector { _handler: CT32B0 },
    Vector { _handler: CT32B1 },
    Vector { _handler: SSP0 },
    Vector { _handler: UART },
    Vector { _handler: USBIRQ },
    Vector { _handler: USBFIQ },
    Vector { _handler: ADC },
    Vector { _handler: WDT },
    Vector { _handler: BOD },
    Vector { _reserved: 0 },
    Vector { _handler: PIO_3 },
    Vector { _handler: PIO_2 },
    Vector { _handler: PIO_1 },
    Vector { _handler: PIO_0 },
    Vector { _handler: SSP1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - PIO0_0"]
    PIO0_0 = 0,
    #[doc = "1 - PIO0_1"]
    PIO0_1 = 1,
    #[doc = "2 - PIO0_2"]
    PIO0_2 = 2,
    #[doc = "3 - PIO0_3"]
    PIO0_3 = 3,
    #[doc = "4 - PIO0_4"]
    PIO0_4 = 4,
    #[doc = "5 - PIO0_5"]
    PIO0_5 = 5,
    #[doc = "6 - PIO0_6"]
    PIO0_6 = 6,
    #[doc = "7 - PIO0_7"]
    PIO0_7 = 7,
    #[doc = "8 - PIO0_8"]
    PIO0_8 = 8,
    #[doc = "9 - PIO0_9"]
    PIO0_9 = 9,
    #[doc = "10 - PIO0_10"]
    PIO0_10 = 10,
    #[doc = "11 - PIO0_11"]
    PIO0_11 = 11,
    #[doc = "12 - PIO1_0"]
    PIO1_0 = 12,
    #[doc = "13 - PIO1_1"]
    PIO1_1 = 13,
    #[doc = "14 - PIO1_2"]
    PIO1_2 = 14,
    #[doc = "15 - PIO1_3"]
    PIO1_3 = 15,
    #[doc = "16 - PIO1_4"]
    PIO1_4 = 16,
    #[doc = "17 - PIO1_5"]
    PIO1_5 = 17,
    #[doc = "18 - PIO1_6"]
    PIO1_6 = 18,
    #[doc = "19 - PIO1_7"]
    PIO1_7 = 19,
    #[doc = "20 - PIO1_8"]
    PIO1_8 = 20,
    #[doc = "21 - PIO1_9"]
    PIO1_9 = 21,
    #[doc = "22 - PIO1_10"]
    PIO1_10 = 22,
    #[doc = "23 - PIO1_11"]
    PIO1_11 = 23,
    #[doc = "24 - PIO2_0"]
    PIO2_0 = 24,
    #[doc = "25 - PIO2_1"]
    PIO2_1 = 25,
    #[doc = "26 - PIO2_2"]
    PIO2_2 = 26,
    #[doc = "27 - PIO2_3"]
    PIO2_3 = 27,
    #[doc = "28 - PIO2_4"]
    PIO2_4 = 28,
    #[doc = "29 - PIO2_5"]
    PIO2_5 = 29,
    #[doc = "30 - PIO2_6"]
    PIO2_6 = 30,
    #[doc = "31 - PIO2_7"]
    PIO2_7 = 31,
    #[doc = "32 - PIO2_8"]
    PIO2_8 = 32,
    #[doc = "33 - PIO2_9"]
    PIO2_9 = 33,
    #[doc = "34 - PIO2_10"]
    PIO2_10 = 34,
    #[doc = "35 - PIO2_11"]
    PIO2_11 = 35,
    #[doc = "36 - PIO3_0"]
    PIO3_0 = 36,
    #[doc = "37 - PIO3_1"]
    PIO3_1 = 37,
    #[doc = "38 - PIO3_2"]
    PIO3_2 = 38,
    #[doc = "39 - PIO3_3"]
    PIO3_3 = 39,
    #[doc = "40 - I2C0"]
    I2C0 = 40,
    #[doc = "41 - CT16B0"]
    CT16B0 = 41,
    #[doc = "42 - CT16B1"]
    CT16B1 = 42,
    #[doc = "43 - CT32B0"]
    CT32B0 = 43,
    #[doc = "44 - CT32B1"]
    CT32B1 = 44,
    #[doc = "45 - SSP0"]
    SSP0 = 45,
    #[doc = "46 - UART"]
    UART = 46,
    #[doc = "47 - USBIRQ"]
    USBIRQ = 47,
    #[doc = "48 - USBFIQ"]
    USBFIQ = 48,
    #[doc = "49 - ADC"]
    ADC = 49,
    #[doc = "50 - WDT"]
    WDT = 50,
    #[doc = "51 - BOD"]
    BOD = 51,
    #[doc = "53 - PIO_3"]
    PIO_3 = 53,
    #[doc = "54 - PIO_2"]
    PIO_2 = 54,
    #[doc = "55 - PIO_1"]
    PIO_1 = 55,
    #[doc = "56 - PIO_0"]
    PIO_0 = 56,
    #[doc = "57 - SSP1"]
    SSP1 = 57,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx I2C-bus controller Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx I2C-bus controller Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod i2c;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Windowed WatchDog Timer (WWDT) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Windowed WatchDog Timer (WWDT) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod wwdt;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx UART Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx UART Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod uart;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 16-bit timer/counters (CT16B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct CT16B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B0 {}
impl CT16B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct16b0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for CT16B0 {
    type Target = ct16b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT16B0::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 16-bit timer/counters (CT16B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod ct16b0;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 16-bit timer/counters (CT16B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct CT16B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B1 {}
impl CT16B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct16b0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CT16B1 {
    type Target = ct16b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT16B1::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 32-bit timer/counters (CT32B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct CT32B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B0 {}
impl CT32B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct32b0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for CT32B0 {
    type Target = ct32b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT32B0::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 32-bit timer/counters (CT32B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod ct32b0;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx 32-bit timer/counters (CT32B0/1) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct CT32B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B1 {}
impl CT32B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct32b0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for CT32B1 {
    type Target = ct32b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT32B1::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Analog-to-Digital Converter (ADC) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Analog-to-Digital Converter (ADC) Modification date=4/19/2011 Major revision=2 Minor revision=1"]
pub mod adc;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx USB device controller Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx USB device controller Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod usb;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Power Management Unit (PMU) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Power Management Unit (PMU) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod pmu;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Flash memory programming firmware Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx Flash memory programming firmware Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod fmc;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx SSP0/1 Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx SSP0/1 Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod ssp0;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx I/O configuration Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iocon::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx I/O configuration Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod iocon;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx System configuration Modification date=4/4/2011 Major revision=2 Minor revision=2"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx System configuration Modification date=4/4/2011 Major revision=2 Minor revision=2"]
pub mod syscon;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx SSP0/1 Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp0::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx General Purpose I/O (GPIO) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct GPIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO0 {}
impl GPIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for GPIO0 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO0::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx General Purpose I/O (GPIO) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub mod gpio0;
#[doc = "Product name title=UM10375 Chapter title=LPC13xx General Purpose I/O (GPIO) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct GPIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO1 {}
impl GPIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x5001_0000 as *const _
    }
}
impl Deref for GPIO1 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO1::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx General Purpose I/O (GPIO) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct GPIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO2 {}
impl GPIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for GPIO2 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO2::ptr() }
    }
}
#[doc = "Product name title=UM10375 Chapter title=LPC13xx General Purpose I/O (GPIO) Modification date=4/20/2011 Major revision=2 Minor revision=1"]
pub struct GPIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO3 {}
impl GPIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio0::RegisterBlock {
        0x5003_0000 as *const _
    }
}
impl Deref for GPIO3 {
    type Target = gpio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO3::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "CT16B0"]
    pub CT16B0: CT16B0,
    #[doc = "CT16B1"]
    pub CT16B1: CT16B1,
    #[doc = "CT32B0"]
    pub CT32B0: CT32B0,
    #[doc = "CT32B1"]
    pub CT32B1: CT32B1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "GPIO0"]
    pub GPIO0: GPIO0,
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            I2C: I2C {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            CT16B0: CT16B0 {
                _marker: PhantomData,
            },
            CT16B1: CT16B1 {
                _marker: PhantomData,
            },
            CT32B0: CT32B0 {
                _marker: PhantomData,
            },
            CT32B1: CT32B1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            GPIO0: GPIO0 {
                _marker: PhantomData,
            },
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
        }
    }
}
