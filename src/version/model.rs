#[doc = "Reader of register MODEL"]
pub type R = crate::R<u32, super::MODEL>;
#[doc = "Reader of field `model`"]
pub type MODEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Contains information on which model device this was built for."]
    #[inline(always)]
    pub fn model(&self) -> MODEL_R {
        MODEL_R::new((self.bits & 0xff) as u8)
    }
}
