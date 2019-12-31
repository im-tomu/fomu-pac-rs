#[doc = "Reader of register CTRL"]
pub type R = crate::R<u8, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u8, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `image`"]
pub type IMAGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `image`"]
pub struct IMAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `key`"]
pub type KEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `key`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u8) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Which image to reboot to. ``SB_WARMBOOT`` supports four images that are configured at FPGA startup. The bootloader is image 0, so set these bits to 0 to reboot back into the bootloader."]
    #[inline(always)]
    pub fn image(&self) -> IMAGE_R {
        IMAGE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - A reboot key used to prevent accidental reboots when writing to random areas of memory. To initiate a reboot, set this to ``0b101011``."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Which image to reboot to. ``SB_WARMBOOT`` supports four images that are configured at FPGA startup. The bootloader is image 0, so set these bits to 0 to reboot back into the bootloader."]
    #[inline(always)]
    pub fn image(&mut self) -> IMAGE_W {
        IMAGE_W { w: self }
    }
    #[doc = "Bits 2:7 - A reboot key used to prevent accidental reboots when writing to random areas of memory. To initiate a reboot, set this to ``0b101011``."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
