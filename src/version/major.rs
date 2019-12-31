#[doc = "Reader of register MAJOR"]
pub type R = crate::R<u8, super::MAJOR>;
#[doc = "Reader of field `major`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0xff) as u8)
    }
}
