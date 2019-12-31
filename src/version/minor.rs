#[doc = "Reader of register MINOR"]
pub type R = crate::R<u8, super::MINOR>;
#[doc = "Reader of field `minor`"]
pub type MINOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new((self.bits & 0xff) as u8)
    }
}
