#[doc = "Reader of register SETUP_STATUS"]
pub type R = crate::R<u32, super::SETUP_STATUS>;
#[doc = "Reader of field `epno`"]
pub type EPNO_R = crate::R<u8, u8>;
#[doc = "Reader of field `have`"]
pub type HAVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `pend`"]
pub type PEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `is_in`"]
pub type IS_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `data`"]
pub type DATA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - The destination endpoint for the most recent SETUP token."]
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
    #[doc = "Bit 6 - ``1`` if an IN stage was detected."]
    #[inline(always)]
    pub fn is_in(&self) -> IS_IN_R {
        IS_IN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ``1`` if a DATA stage is expected."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
