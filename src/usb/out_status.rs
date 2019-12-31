#[doc = "Reader of register OUT_STATUS"]
pub type R = crate::R<u8, super::OUT_STATUS>;
#[doc = "Reader of field `epno`"]
pub type EPNO_R = crate::R<u8, u8>;
#[doc = "Reader of field `have`"]
pub type HAVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `pend`"]
pub type PEND_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - The destination endpoint for the most recent ``OUT`` packet."]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ``1`` if there is data in the FIFO."]
    #[inline(always)]
    pub fn have(&self) -> HAVE_R {
        HAVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ``1`` if there is an IRQ pending."]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
