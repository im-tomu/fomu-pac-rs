#[doc = "Reader of register OUT_EV_PENDING"]
pub type R = crate::R<u32, super::OUT_EV_PENDING>;
#[doc = "Writer for register OUT_EV_PENDING"]
pub type W = crate::W<u32, super::OUT_EV_PENDING>;
#[doc = "Register OUT_EV_PENDING `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_EV_PENDING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `out_ev_pending`"]
pub type OUT_EV_PENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `out_ev_pending`"]
pub struct OUT_EV_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EV_PENDING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn out_ev_pending(&self) -> OUT_EV_PENDING_R {
        OUT_EV_PENDING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn out_ev_pending(&mut self) -> OUT_EV_PENDING_W {
        OUT_EV_PENDING_W { w: self }
    }
}
