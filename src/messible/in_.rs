#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Writer for register IN"]
pub type W = crate::W<u32, super::IN>;
#[doc = "Register IN `reset()`'s with value 0"]
impl crate::ResetValue for super::IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `in`"]
pub type IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `in`"]
pub struct IN_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn in_(&mut self) -> IN_W {
        IN_W { w: self }
    }
}
