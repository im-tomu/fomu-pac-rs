#[doc = "Reader of register GITEXTRA0"]
pub type R = crate::R<u32, super::GITEXTRA0>;
#[doc = "Reader of field `gitextra`"]
pub type GITEXTRA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gitextra(&self) -> GITEXTRA_R {
        GITEXTRA_R::new((self.bits & 0xff) as u8)
    }
}
