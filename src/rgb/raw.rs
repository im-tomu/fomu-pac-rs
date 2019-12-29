#[doc = "Reader of register RAW"]
pub type R = crate::R<u32, super::RAW>;
#[doc = "Writer for register RAW"]
pub type W = crate::W<u32, super::RAW>;
#[doc = "Register RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `r`"]
pub type R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `r`"]
pub struct R_W<'a> {
    w: &'a mut W,
}
impl<'a> R_W<'a> {
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
#[doc = "Reader of field `g`"]
pub type G_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `g`"]
pub struct G_W<'a> {
    w: &'a mut W,
}
impl<'a> G_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `b`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `b`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Raw value for the red LED when ``CTRL.RRAW`` is ``1``."]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw value for the green LED when ``CTRL.GRAW`` is ``1``."]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw value for the blue LED when ``CTRL.BRAW`` is ``1``."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw value for the red LED when ``CTRL.RRAW`` is ``1``."]
    #[inline(always)]
    pub fn r(&mut self) -> R_W {
        R_W { w: self }
    }
    #[doc = "Bit 1 - Raw value for the green LED when ``CTRL.GRAW`` is ``1``."]
    #[inline(always)]
    pub fn g(&mut self) -> G_W {
        G_W { w: self }
    }
    #[doc = "Bit 2 - Raw value for the blue LED when ``CTRL.BRAW`` is ``1``."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
}
