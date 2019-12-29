#[doc = "Reader of register MODEL"]
pub type R = crate::R<u32, super::MODEL>;
#[doc = "Writer for register MODEL"]
pub type W = crate::W<u32, super::MODEL>;
#[doc = "Register MODEL `reset()`'s with value 0x50"]
impl crate::ResetValue for super::MODEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x50
    }
}
#[doc = "Reader of field `model`"]
pub type MODEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `model`"]
pub struct MODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Contains information on which model device this was built for."]
    #[inline(always)]
    pub fn model(&self) -> MODEL_R {
        MODEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains information on which model device this was built for."]
    #[inline(always)]
    pub fn model(&mut self) -> MODEL_W {
        MODEL_W { w: self }
    }
}
