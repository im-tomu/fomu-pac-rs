#[doc = "Reader of register SETUP_DATA"]
pub type R = crate::R<u32, super::SETUP_DATA>;
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The next byte of ``SETUP`` data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
