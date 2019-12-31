#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `full`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `have`"]
pub type HAVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ``0`` if more data can fit into the IN FIFO."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ``1`` if data can be read from the OUT FIFO."]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
