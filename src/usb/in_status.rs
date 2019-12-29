#[doc = "Reader of register IN_STATUS"]
pub type R = crate::R<u32, super::IN_STATUS>;
#[doc = "Writer for register IN_STATUS"]
pub type W = crate::W<u32, super::IN_STATUS>;
#[doc = "Register IN_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::IN_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `idle`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `idle`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
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
#[doc = "Reader of field `have`"]
pub type HAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `have`"]
pub struct HAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HAVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `pend`"]
pub type PEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pend`"]
pub struct PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> PEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
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
impl W {
    #[doc = "Bit 0 - This value is ``1`` if the packet has finished transmitting."]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 4 - This value is ``0`` if the FIFO is empty."]
    #[inline(always)]
    pub fn have(&mut self) -> HAVE_W {
        HAVE_W { w: self }
    }
    #[doc = "Bit 5 - ``1`` if there is an IRQ pending."]
    #[inline(always)]
    pub fn pend(&mut self) -> PEND_W {
        PEND_W { w: self }
    }
}
