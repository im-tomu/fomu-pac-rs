#[doc = "Reader of register OUT_DATA"]
pub type R = crate::R<u8, super::OUT_DATA>;
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The top byte of the receive FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
