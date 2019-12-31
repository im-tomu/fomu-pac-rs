#[doc = "Reader of register I"]
pub type R = crate::R<u32, super::I>;
#[doc = "Reader of field `i`"]
pub type I_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new((self.bits & 0x0f) as u8)
    }
}
