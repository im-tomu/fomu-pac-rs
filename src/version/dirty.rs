#[doc = "Reader of register DIRTY"]
pub type R = crate::R<u8, super::DIRTY>;
#[doc = "Reader of field `dirty`"]
pub type DIRTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Set to ``1`` if this device was built from a git repo with uncommitted modifications."]
    #[inline(always)]
    pub fn dirty(&self) -> DIRTY_R {
        DIRTY_R::new((self.bits & 0x01) != 0)
    }
}
