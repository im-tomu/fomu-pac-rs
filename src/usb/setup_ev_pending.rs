#[doc = "Reader of register SETUP_EV_PENDING"]
pub type R = crate::R<u32, super::SETUP_EV_PENDING>;
#[doc = "Writer for register SETUP_EV_PENDING"]
pub type W = crate::W<u32, super::SETUP_EV_PENDING>;
#[doc = "Register SETUP_EV_PENDING `reset()`'s with value 0"]
impl crate::ResetValue for super::SETUP_EV_PENDING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `setup_ev_pending`"]
pub type SETUP_EV_PENDING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `setup_ev_pending`"]
pub struct SETUP_EV_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_EV_PENDING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn setup_ev_pending(&self) -> SETUP_EV_PENDING_R {
        SETUP_EV_PENDING_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn setup_ev_pending(&mut self) -> SETUP_EV_PENDING_W {
        SETUP_EV_PENDING_W { w: self }
    }
}
