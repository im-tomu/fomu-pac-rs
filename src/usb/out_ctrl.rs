#[doc = "Reader of register OUT_CTRL"]
pub type R = crate::R<u32, super::OUT_CTRL>;
#[doc = "Writer for register OUT_CTRL"]
pub type W = crate::W<u32, super::OUT_CTRL>;
#[doc = "Register OUT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_CTRL {
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
#[doc = "Reader of field `enable`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `reset`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reset`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Reader of field `stall`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `stall`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The endpoint number to update the ``enable`` and ``status`` bits for."]
    #[inline(always)]
    pub fn epno(&self) -> EPNO_R {
        EPNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Write a ``1`` here to enable receiving data"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` here to reset the ``OUT`` handler"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` here to stall an endpoint"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The endpoint number to update the ``enable`` and ``status`` bits for."]
    #[inline(always)]
    pub fn epno(&mut self) -> EPNO_W {
        EPNO_W { w: self }
    }
    #[doc = "Bit 4 - Write a ``1`` here to enable receiving data"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Write a ``1`` here to reset the ``OUT`` handler"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 6 - Write a ``1`` here to stall an endpoint"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
}
