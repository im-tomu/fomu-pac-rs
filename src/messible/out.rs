#[doc = "Reader of register OUT"]
pub type R = crate::R<u8, super::OUT>;
#[doc = "Reader of field `out`"]
pub type OUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xff) as u8)
    }
}
