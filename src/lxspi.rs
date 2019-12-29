#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bitbang controls for SPI output. Only standard 1x SPI is supported, meaning the IO2 and IO3 lines will be hardwired to `1` during bitbang mode."]
    pub bitbang: BITBANG,
    #[doc = "0x04 - Incoming value of MISO signal."]
    pub miso: MISO,
    #[doc = "0x08 - Write a `1` here to disable memory-mapped mode and enable bitbang mode."]
    pub bitbang_en: BITBANG_EN,
}
#[doc = "Bitbang controls for SPI output. Only standard 1x SPI is supported, meaning the IO2 and IO3 lines will be hardwired to `1` during bitbang mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bitbang](bitbang) module"]
pub type BITBANG = crate::Reg<u32, _BITBANG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITBANG;
#[doc = "`read()` method returns [bitbang::R](bitbang::R) reader structure"]
impl crate::Readable for BITBANG {}
#[doc = "`write(|w| ..)` method takes [bitbang::W](bitbang::W) writer structure"]
impl crate::Writable for BITBANG {}
#[doc = "Bitbang controls for SPI output. Only standard 1x SPI is supported, meaning the IO2 and IO3 lines will be hardwired to `1` during bitbang mode."]
pub mod bitbang;
#[doc = "Incoming value of MISO signal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [miso](miso) module"]
pub type MISO = crate::Reg<u32, _MISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISO;
#[doc = "`read()` method returns [miso::R](miso::R) reader structure"]
impl crate::Readable for MISO {}
#[doc = "`write(|w| ..)` method takes [miso::W](miso::W) writer structure"]
impl crate::Writable for MISO {}
#[doc = "Incoming value of MISO signal."]
pub mod miso;
#[doc = "Write a `1` here to disable memory-mapped mode and enable bitbang mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bitbang_en](bitbang_en) module"]
pub type BITBANG_EN = crate::Reg<u32, _BITBANG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITBANG_EN;
#[doc = "`read()` method returns [bitbang_en::R](bitbang_en::R) reader structure"]
impl crate::Readable for BITBANG_EN {}
#[doc = "`write(|w| ..)` method takes [bitbang_en::W](bitbang_en::W) writer structure"]
impl crate::Writable for BITBANG_EN {}
#[doc = "Write a `1` here to disable memory-mapped mode and enable bitbang mode."]
pub mod bitbang_en;
