#[doc = "Reader of register IN_EV_STATUS"]
pub type R = crate::R<u8, super::IN_EV_STATUS>;
#[doc = "Writer for register IN_EV_STATUS"]
pub type W = crate::W<u8, super::IN_EV_STATUS>;
#[doc = "Register IN_EV_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::IN_EV_STATUS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `in_ev_status`"]
pub type IN_EV_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `in_ev_status`"]
pub struct IN_EV_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EV_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn in_ev_status(&self) -> IN_EV_STATUS_R {
        IN_EV_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn in_ev_status(&mut self) -> IN_EV_STATUS_W {
        IN_EV_STATUS_W { w: self }
    }
}
