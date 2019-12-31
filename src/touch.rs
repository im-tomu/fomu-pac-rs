#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output values for pads 1-4"]
    pub o: O,
    #[doc = "0x04 - Output enable control for pads 1-4"]
    pub oe: OE,
    #[doc = "0x08 - Input value for pads 1-4"]
    pub i: I,
}
#[doc = "Output values for pads 1-4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [o](o) module"]
pub type O = crate::Reg<u32, _O>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _O;
#[doc = "`read()` method returns [o::R](o::R) reader structure"]
impl crate::Readable for O {}
#[doc = "`write(|w| ..)` method takes [o::W](o::W) writer structure"]
impl crate::Writable for O {}
#[doc = "Output values for pads 1-4"]
pub mod o;
#[doc = "Output enable control for pads 1-4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oe](oe) module"]
pub type OE = crate::Reg<u32, _OE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OE;
#[doc = "`read()` method returns [oe::R](oe::R) reader structure"]
impl crate::Readable for OE {}
#[doc = "`write(|w| ..)` method takes [oe::W](oe::W) writer structure"]
impl crate::Writable for OE {}
#[doc = "Output enable control for pads 1-4"]
pub mod oe;
#[doc = "Input value for pads 1-4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i](i) module"]
pub type I = crate::Reg<u32, _I>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I;
#[doc = "`read()` method returns [i::R](i::R) reader structure"]
impl crate::Readable for I {}
#[doc = "Input value for pads 1-4"]
pub mod i;
