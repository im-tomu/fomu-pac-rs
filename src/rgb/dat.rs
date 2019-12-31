#[doc = "Reader of register DAT"]
pub type R = crate::R<u8, super::DAT>;
#[doc = "Writer for register DAT"]
pub type W = crate::W<u8, super::DAT>;
#[doc = "Register DAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dat`"]
pub type DAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dat`"]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
}
