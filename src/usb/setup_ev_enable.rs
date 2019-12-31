#[doc = "Reader of register SETUP_EV_ENABLE"]
pub type R = crate::R<u8, super::SETUP_EV_ENABLE>;
#[doc = "Writer for register SETUP_EV_ENABLE"]
pub type W = crate::W<u8, super::SETUP_EV_ENABLE>;
#[doc = "Register SETUP_EV_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::SETUP_EV_ENABLE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `setup_ev_enable`"]
pub type SETUP_EV_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `setup_ev_enable`"]
pub struct SETUP_EV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_EV_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn setup_ev_enable(&self) -> SETUP_EV_ENABLE_R {
        SETUP_EV_ENABLE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn setup_ev_enable(&mut self) -> SETUP_EV_ENABLE_W {
        SETUP_EV_ENABLE_W { w: self }
    }
}
