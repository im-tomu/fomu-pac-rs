#[doc = "Reader of register O"]
pub type R = crate::R<u8, super::O>;
#[doc = "Writer for register O"]
pub type W = crate::W<u8, super::O>;
#[doc = "Register O `reset()`'s with value 0"]
impl crate::ResetValue for super::O {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `o`"]
pub type O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `o`"]
pub struct O_W<'a> {
    w: &'a mut W,
}
impl<'a> O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn o(&self) -> O_R {
        O_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn o(&mut self) -> O_W {
        O_W { w: self }
    }
}
