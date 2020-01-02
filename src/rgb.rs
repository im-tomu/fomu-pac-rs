#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This is the value for the ``SB_LEDDA_IP.DAT`` register. It is directly written into the ``SB_LEDDA_IP`` hardware block, so you should refer to http://www.latticesemi.com/view_document?document_id=50668. The contents of this register are written to the address specified in ``ADDR`` immediately upon writing this register."]
    pub dat: DAT,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - This register is directly connected to ``SB_LEDDA_IP.ADDR``. This register controls the address that is updated whenever ``DAT`` is written. Writing to this register has no immediate effect -- data isn't written until the ``DAT`` register is written."]
    pub addr: ADDR,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Control logic for the RGB LED and LEDDA hardware PWM LED block."]
    pub ctrl: CTRL,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Normally the hardware ``SB_LEDDA_IP`` block controls the brightness of the LED, creating a gentle fading pattern. However, by setting the appropriate bit in ``CTRL``, it is possible to manually control the three individual LEDs."]
    pub raw: RAW,
}
#[doc = "This is the value for the ``SB_LEDDA_IP.DAT`` register. It is directly written into the ``SB_LEDDA_IP`` hardware block, so you should refer to http://www.latticesemi.com/view_document?document_id=50668. The contents of this register are written to the address specified in ``ADDR`` immediately upon writing this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dat](dat) module"]
pub type DAT = crate::Reg<u8, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "`write(|w| ..)` method takes [dat::W](dat::W) writer structure"]
impl crate::Writable for DAT {}
#[doc = "This is the value for the ``SB_LEDDA_IP.DAT`` register. It is directly written into the ``SB_LEDDA_IP`` hardware block, so you should refer to http://www.latticesemi.com/view_document?document_id=50668. The contents of this register are written to the address specified in ``ADDR`` immediately upon writing this register."]
pub mod dat;
#[doc = "This register is directly connected to ``SB_LEDDA_IP.ADDR``. This register controls the address that is updated whenever ``DAT`` is written. Writing to this register has no immediate effect -- data isn't written until the ``DAT`` register is written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u8, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "This register is directly connected to ``SB_LEDDA_IP.ADDR``. This register controls the address that is updated whenever ``DAT`` is written. Writing to this register has no immediate effect -- data isn't written until the ``DAT`` register is written."]
pub mod addr;
#[doc = "Control logic for the RGB LED and LEDDA hardware PWM LED block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u8, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control logic for the RGB LED and LEDDA hardware PWM LED block."]
pub mod ctrl;
#[doc = "Normally the hardware ``SB_LEDDA_IP`` block controls the brightness of the LED, creating a gentle fading pattern. However, by setting the appropriate bit in ``CTRL``, it is possible to manually control the three individual LEDs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [raw](raw) module"]
pub type RAW = crate::Reg<u8, _RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAW;
#[doc = "`read()` method returns [raw::R](raw::R) reader structure"]
impl crate::Readable for RAW {}
#[doc = "`write(|w| ..)` method takes [raw::W](raw::W) writer structure"]
impl crate::Writable for RAW {}
#[doc = "Normally the hardware ``SB_LEDDA_IP`` block controls the brightness of the LED, creating a gentle fading pattern. However, by setting the appropriate bit in ``CTRL``, it is possible to manually control the three individual LEDs."]
pub mod raw;
