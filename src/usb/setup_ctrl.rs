#[doc = "Reader of register SETUP_CTRL"]
pub type R = crate::R<u32, super::SETUP_CTRL>;
#[doc = "Writer for register SETUP_CTRL"]
pub type W = crate::W<u32, super::SETUP_CTRL>;
#[doc = "Register SETUP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SETUP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
impl R {
    #[doc = "Bit 5 - Write a ``1`` here to reset the `SETUP` handler."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Write a ``1`` here to reset the `SETUP` handler."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
}
