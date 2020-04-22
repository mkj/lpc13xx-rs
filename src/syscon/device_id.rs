#[doc = "Reader of register DEVICE_ID"]
pub type R = crate::R<u32, super::DEVICE_ID>;
#[doc = "Reader of field `DEVICEID`"]
pub type DEVICEID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device ID for LPC13xx parts: 0x2C42 502B = LPC1311FHN33 0x2C40 102B = LPC1313FHN33 0x2C40 102B = LPC1313FBD48 0x3D01 402B = LPC1342FHN33 0x3D00 002B = LPC1343FHN33 0x3D00 002B = LPC1343FBD48 0x1816 902B = LPC1311FHN33/01 0x1830 102B = LPC1313FHN33/01 0x1830 102B = LPC1313FBD48/01"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
