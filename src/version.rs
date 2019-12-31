#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Major git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``2``."]
    pub major: MAJOR,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Minor git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``0``."]
    pub minor: MINOR,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Revision git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``3``."]
    pub revision: REVISION,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Bits 24-31 of `VERSION_GITREV`. First 32-bits of the git revision. This documentation was built from git rev ``00000000``, so this value is 0, which should be enough to check out the exact git version used to build this firmware."]
    pub gitrev3: GITREV3,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - Bits 16-23 of `VERSION_GITREV`."]
    pub gitrev2: GITREV2,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - Bits 8-15 of `VERSION_GITREV`."]
    pub gitrev1: GITREV1,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - Bits 0-7 of `VERSION_GITREV`."]
    pub gitrev0: GITREV0,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - Bits 8-9 of `VERSION_GITEXTRA`. The number of additional commits beyond the git tag. For example, if this value is ``1``, then the repository this was built from has one additional commit beyond the tag indicated in `MAJOR`, `MINOR`, and `REVISION`."]
    pub gitextra1: GITEXTRA1,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - Bits 0-7 of `VERSION_GITEXTRA`."]
    pub gitextra0: GITEXTRA0,
    _reserved9: [u8; 3usize],
    #[doc = "0x24 - "]
    pub dirty: DIRTY,
    _reserved10: [u8; 4usize],
    #[doc = "0x28 - "]
    pub model: MODEL,
    _reserved11: [u8; 3usize],
    #[doc = "0x2c - Bits 24-31 of `VERSION_SEED`. 32-bit seed used for the place-and-route."]
    pub seed3: SEED3,
    _reserved12: [u8; 3usize],
    #[doc = "0x30 - Bits 16-23 of `VERSION_SEED`."]
    pub seed2: SEED2,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - Bits 8-15 of `VERSION_SEED`."]
    pub seed1: SEED1,
    _reserved14: [u8; 3usize],
    #[doc = "0x38 - Bits 0-7 of `VERSION_SEED`."]
    pub seed0: SEED0,
}
#[doc = "Major git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``2``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [major](major) module"]
pub type MAJOR = crate::Reg<u8, _MAJOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAJOR;
#[doc = "`read()` method returns [major::R](major::R) reader structure"]
impl crate::Readable for MAJOR {}
#[doc = "Major git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``2``."]
pub mod major;
#[doc = "Minor git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``0``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [minor](minor) module"]
pub type MINOR = crate::Reg<u8, _MINOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MINOR;
#[doc = "`read()` method returns [minor::R](minor::R) reader structure"]
impl crate::Readable for MINOR {}
#[doc = "Minor git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``0``."]
pub mod minor;
#[doc = "Revision git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``3``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u8, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "Revision git tag version. For example, this firmware was built from git tag ``v2.0.3``, so this value is ``3``."]
pub mod revision;
#[doc = "Bits 24-31 of `VERSION_GITREV`. First 32-bits of the git revision. This documentation was built from git rev ``00000000``, so this value is 0, which should be enough to check out the exact git version used to build this firmware.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev3](gitrev3) module"]
pub type GITREV3 = crate::Reg<u8, _GITREV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV3;
#[doc = "`read()` method returns [gitrev3::R](gitrev3::R) reader structure"]
impl crate::Readable for GITREV3 {}
#[doc = "Bits 24-31 of `VERSION_GITREV`. First 32-bits of the git revision. This documentation was built from git rev ``00000000``, so this value is 0, which should be enough to check out the exact git version used to build this firmware."]
pub mod gitrev3;
#[doc = "Bits 16-23 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev2](gitrev2) module"]
pub type GITREV2 = crate::Reg<u8, _GITREV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV2;
#[doc = "`read()` method returns [gitrev2::R](gitrev2::R) reader structure"]
impl crate::Readable for GITREV2 {}
#[doc = "Bits 16-23 of `VERSION_GITREV`."]
pub mod gitrev2;
#[doc = "Bits 8-15 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev1](gitrev1) module"]
pub type GITREV1 = crate::Reg<u8, _GITREV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV1;
#[doc = "`read()` method returns [gitrev1::R](gitrev1::R) reader structure"]
impl crate::Readable for GITREV1 {}
#[doc = "Bits 8-15 of `VERSION_GITREV`."]
pub mod gitrev1;
#[doc = "Bits 0-7 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev0](gitrev0) module"]
pub type GITREV0 = crate::Reg<u8, _GITREV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV0;
#[doc = "`read()` method returns [gitrev0::R](gitrev0::R) reader structure"]
impl crate::Readable for GITREV0 {}
#[doc = "Bits 0-7 of `VERSION_GITREV`."]
pub mod gitrev0;
#[doc = "Bits 8-9 of `VERSION_GITEXTRA`. The number of additional commits beyond the git tag. For example, if this value is ``1``, then the repository this was built from has one additional commit beyond the tag indicated in `MAJOR`, `MINOR`, and `REVISION`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitextra1](gitextra1) module"]
pub type GITEXTRA1 = crate::Reg<u8, _GITEXTRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITEXTRA1;
#[doc = "`read()` method returns [gitextra1::R](gitextra1::R) reader structure"]
impl crate::Readable for GITEXTRA1 {}
#[doc = "Bits 8-9 of `VERSION_GITEXTRA`. The number of additional commits beyond the git tag. For example, if this value is ``1``, then the repository this was built from has one additional commit beyond the tag indicated in `MAJOR`, `MINOR`, and `REVISION`."]
pub mod gitextra1;
#[doc = "Bits 0-7 of `VERSION_GITEXTRA`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitextra0](gitextra0) module"]
pub type GITEXTRA0 = crate::Reg<u8, _GITEXTRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITEXTRA0;
#[doc = "`read()` method returns [gitextra0::R](gitextra0::R) reader structure"]
impl crate::Readable for GITEXTRA0 {}
#[doc = "Bits 0-7 of `VERSION_GITEXTRA`."]
pub mod gitextra0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dirty](dirty) module"]
pub type DIRTY = crate::Reg<u8, _DIRTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRTY;
#[doc = "`read()` method returns [dirty::R](dirty::R) reader structure"]
impl crate::Readable for DIRTY {}
#[doc = ""]
pub mod dirty;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [model](model) module"]
pub type MODEL = crate::Reg<u8, _MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEL;
#[doc = "`read()` method returns [model::R](model::R) reader structure"]
impl crate::Readable for MODEL {}
#[doc = ""]
pub mod model;
#[doc = "Bits 24-31 of `VERSION_SEED`. 32-bit seed used for the place-and-route.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seed3](seed3) module"]
pub type SEED3 = crate::Reg<u8, _SEED3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEED3;
#[doc = "`read()` method returns [seed3::R](seed3::R) reader structure"]
impl crate::Readable for SEED3 {}
#[doc = "Bits 24-31 of `VERSION_SEED`. 32-bit seed used for the place-and-route."]
pub mod seed3;
#[doc = "Bits 16-23 of `VERSION_SEED`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seed2](seed2) module"]
pub type SEED2 = crate::Reg<u8, _SEED2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEED2;
#[doc = "`read()` method returns [seed2::R](seed2::R) reader structure"]
impl crate::Readable for SEED2 {}
#[doc = "Bits 16-23 of `VERSION_SEED`."]
pub mod seed2;
#[doc = "Bits 8-15 of `VERSION_SEED`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seed1](seed1) module"]
pub type SEED1 = crate::Reg<u8, _SEED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEED1;
#[doc = "`read()` method returns [seed1::R](seed1::R) reader structure"]
impl crate::Readable for SEED1 {}
#[doc = "Bits 8-15 of `VERSION_SEED`."]
pub mod seed1;
#[doc = "Bits 0-7 of `VERSION_SEED`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seed0](seed0) module"]
pub type SEED0 = crate::Reg<u8, _SEED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEED0;
#[doc = "`read()` method returns [seed0::R](seed0::R) reader structure"]
impl crate::Readable for SEED0 {}
#[doc = "Bits 0-7 of `VERSION_SEED`."]
pub mod seed0;
