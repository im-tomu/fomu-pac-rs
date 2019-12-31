#[doc = "Reader of register IN_STATUS"]
pub type R = crate::R<u8, super::IN_STATUS>;
#[doc = "Reader of field `idle`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `have`"]
pub type HAVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `pend`"]
pub type PEND_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This value is ``1`` if the packet has finished transmitting."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - This value is ``0`` if the FIFO is empty."]
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
