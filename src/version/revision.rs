#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Writer for register REVISION"]
pub type W = crate::W<u32, super::REVISION>;
#[doc = "Register REVISION `reset()`'s with value 0x01"]
impl crate::ResetValue for super::REVISION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
impl R {}
impl W {}
