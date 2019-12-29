#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `exe`"]
pub type EXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `exe`"]
pub struct EXE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXE_W<'a> {
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
#[doc = "Reader of field `curren`"]
pub type CURREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `curren`"]
pub struct CURREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CURREN_W<'a> {
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
#[doc = "Reader of field `rgbleden`"]
pub type RGBLEDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rgbleden`"]
pub struct RGBLEDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RGBLEDEN_W<'a> {
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
#[doc = "Reader of field `rraw`"]
pub type RRAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rraw`"]
pub struct RRAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RRAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `graw`"]
pub type GRAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `graw`"]
pub struct GRAW_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `braw`"]
pub type BRAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `braw`"]
pub struct BRAW_W<'a> {
    w: &'a mut W,
}
impl<'a> BRAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Connected to ``SB_LEDDA_IP.LEDDEXE``. Set this to ``1`` to enable the fading pattern."]
    #[inline(always)]
    pub fn exe(&self) -> EXE_R {
        EXE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connected to ``SB_RGBA_DRV.CURREN``. Set this to ``1`` to enable the current source."]
    #[inline(always)]
    pub fn curren(&self) -> CURREN_R {
        CURREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Connected to ``SB_RGBA_DRV.RGBLEDEN``. Set this to ``1`` to enable the RGB PWM control logic."]
    #[inline(always)]
    pub fn rgbleden(&self) -> RGBLEDEN_R {
        RGBLEDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this to ``1`` to enable raw control of the red LED via the ``RAW.R`` register."]
    #[inline(always)]
    pub fn rraw(&self) -> RRAW_R {
        RRAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this to ``1`` to enable raw control of the green LED via the ``RAW.G`` register."]
    #[inline(always)]
    pub fn graw(&self) -> GRAW_R {
        GRAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this to ``1`` to enable raw control of the blue LED via the ``RAW.B`` register."]
    #[inline(always)]
    pub fn braw(&self) -> BRAW_R {
        BRAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Connected to ``SB_LEDDA_IP.LEDDEXE``. Set this to ``1`` to enable the fading pattern."]
    #[inline(always)]
    pub fn exe(&mut self) -> EXE_W {
        EXE_W { w: self }
    }
    #[doc = "Bit 1 - Connected to ``SB_RGBA_DRV.CURREN``. Set this to ``1`` to enable the current source."]
    #[inline(always)]
    pub fn curren(&mut self) -> CURREN_W {
        CURREN_W { w: self }
    }
    #[doc = "Bit 2 - Connected to ``SB_RGBA_DRV.RGBLEDEN``. Set this to ``1`` to enable the RGB PWM control logic."]
    #[inline(always)]
    pub fn rgbleden(&mut self) -> RGBLEDEN_W {
        RGBLEDEN_W { w: self }
    }
    #[doc = "Bit 3 - Set this to ``1`` to enable raw control of the red LED via the ``RAW.R`` register."]
    #[inline(always)]
    pub fn rraw(&mut self) -> RRAW_W {
        RRAW_W { w: self }
    }
    #[doc = "Bit 4 - Set this to ``1`` to enable raw control of the green LED via the ``RAW.G`` register."]
    #[inline(always)]
    pub fn graw(&mut self) -> GRAW_W {
        GRAW_W { w: self }
    }
    #[doc = "Bit 5 - Set this to ``1`` to enable raw control of the blue LED via the ``RAW.B`` register."]
    #[inline(always)]
    pub fn braw(&mut self) -> BRAW_W {
        BRAW_W { w: self }
    }
}
