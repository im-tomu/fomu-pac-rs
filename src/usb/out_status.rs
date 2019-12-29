#[doc = "Reader of register OUT_STATUS"]
pub type R = crate::R<u32, super::OUT_STATUS>;
#[doc = "Writer for register OUT_STATUS"]
pub type W = crate::W<u32, super::OUT_STATUS>;
#[doc = "Register OUT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `epno`"]
pub type EPNO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `epno`"]
pub struct EPNO_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
impl W {
    #[doc = "Bits 0:3 - The destination endpoint for the most recent ``OUT`` packet."]
    #[inline(always)]
    pub fn epno(&mut self) -> EPNO_W {
        EPNO_W { w: self }
    }
    #[doc = "Bit 4 - ``1`` if there is data in the FIFO."]
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
