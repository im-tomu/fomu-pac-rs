#[doc = "Reader of register PULLUP_OUT"]
pub type R = crate::R<u8, super::PULLUP_OUT>;
#[doc = "Writer for register PULLUP_OUT"]
pub type W = crate::W<u8, super::PULLUP_OUT>;
#[doc = "Register PULLUP_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PULLUP_OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pullup_out`"]
pub type PULLUP_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pullup_out`"]
pub struct PULLUP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_OUT_W<'a> {
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
    pub fn pullup_out(&self) -> PULLUP_OUT_R {
        PULLUP_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pullup_out(&mut self) -> PULLUP_OUT_W {
        PULLUP_OUT_W { w: self }
    }
}
