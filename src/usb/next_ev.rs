#[doc = "Reader of register NEXT_EV"]
pub type R = crate::R<u8, super::NEXT_EV>;
#[doc = "Reader of field `in`"]
pub type IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `out`"]
pub type OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `setup`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `reset`"]
pub type RESET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ``1`` if the next event is an ``IN`` event"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ``1`` if the next event is an ``OUT`` event"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ``1`` if the next event is an ``SETUP`` event"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ``1`` if the next event is a ``RESET`` event"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
