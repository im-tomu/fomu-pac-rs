#[doc = "Reader of register ADDRESS"]
pub type R = crate::R<u8, super::ADDRESS>;
#[doc = "Writer for register ADDRESS"]
pub type W = crate::W<u8, super::ADDRESS>;
#[doc = "Register ADDRESS `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDRESS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `addr`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `addr`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u8) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Write the USB address from USB ``SET_ADDRESS`` packets."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Write the USB address from USB ``SET_ADDRESS`` packets."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
