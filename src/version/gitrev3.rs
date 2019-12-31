#[doc = "Reader of register GITREV3"]
pub type R = crate::R<u32, super::GITREV3>;
#[doc = "Reader of field `gitrev`"]
pub type GITREV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gitrev(&self) -> GITREV_R {
        GITREV_R::new((self.bits & 0xff) as u8)
    }
}
