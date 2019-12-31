#[doc = "Reader of register OE"]
pub type R = crate::R<u32, super::OE>;
#[doc = "Writer for register OE"]
pub type W = crate::W<u32, super::OE>;
#[doc = "Register OE `reset()`'s with value 0"]
impl crate::ResetValue for super::OE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `oe`"]
pub type OE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `oe`"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
}
