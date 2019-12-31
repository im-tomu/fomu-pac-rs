#[doc = "Reader of register OUT_EV_ENABLE"]
pub type R = crate::R<u32, super::OUT_EV_ENABLE>;
#[doc = "Writer for register OUT_EV_ENABLE"]
pub type W = crate::W<u32, super::OUT_EV_ENABLE>;
#[doc = "Register OUT_EV_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_EV_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `out_ev_enable`"]
pub type OUT_EV_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `out_ev_enable`"]
pub struct OUT_EV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EV_ENABLE_W<'a> {
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
    pub fn out_ev_enable(&self) -> OUT_EV_ENABLE_R {
        OUT_EV_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn out_ev_enable(&mut self) -> OUT_EV_ENABLE_W {
        OUT_EV_ENABLE_W { w: self }
    }
}
