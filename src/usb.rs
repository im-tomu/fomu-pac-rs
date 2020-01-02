#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub pullup_out: PULLUP_OUT,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Sets the USB device address, in order to ignore packets going to other devices on the bus. This value is reset when the host issues a USB Device Reset condition."]
    pub address: ADDRESS,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - In ``eptri``, there are three endpoints. It is possible for an IRQ to fire and have all three bits set. Under these circumstances it can be difficult to know which event to process first. Use this register to determine which event needs to be processed first. Only one bit will ever be set at a time."]
    pub next_ev: NEXT_EV,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Data from the last ``SETUP`` transactions. It will be 10 bytes long, because it will include the CRC16. This is a FIFO, and the queue is advanced automatically."]
    pub setup_data: SETUP_DATA,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - Controls for managing how to handle ``SETUP`` transactions."]
    pub setup_ctrl: SETUP_CTRL,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - Status about the most recent ``SETUP`` transactions, and the state of the FIFO."]
    pub setup_status: SETUP_STATUS,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - "]
    pub setup_ev_status: SETUP_EV_STATUS,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - "]
    pub setup_ev_pending: SETUP_EV_PENDING,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - "]
    pub setup_ev_enable: SETUP_EV_ENABLE,
    _reserved9: [u8; 3usize],
    #[doc = "0x24 - Each byte written into this register gets added to an outgoing FIFO. Any bytes that are written here will be transmitted in the order in which they were added. The FIFO queue is automatically advanced with each write. The FIFO queue is 64 bytes deep. If you exceed this amount, the result is undefined."]
    pub in_data: IN_DATA,
    _reserved10: [u8; 3usize],
    #[doc = "0x28 - Enables transmission of data in response to ``IN`` tokens, or resets the contents of the FIFO."]
    pub in_ctrl: IN_CTRL,
    _reserved11: [u8; 3usize],
    #[doc = "0x2c - Status about the IN handler. As soon as you write to `IN_DATA`, ``IN_STATUS.HAVE`` should go to ``1``."]
    pub in_status: IN_STATUS,
    _reserved12: [u8; 3usize],
    #[doc = "0x30 - "]
    pub in_ev_status: IN_EV_STATUS,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - "]
    pub in_ev_pending: IN_EV_PENDING,
    _reserved14: [u8; 3usize],
    #[doc = "0x38 - "]
    pub in_ev_enable: IN_EV_ENABLE,
    _reserved15: [u8; 3usize],
    #[doc = "0x3c - Data received from the host will go into a FIFO. This register reflects the contents of the top byte in that FIFO. Reading from this register advances the FIFO pointer."]
    pub out_data: OUT_DATA,
    _reserved16: [u8; 3usize],
    #[doc = "0x40 - Controls for receiving packet data. To enable an endpoint, write its value to ``epno``, with the ``enable`` bit set to ``1`` to enable an endpoint, or ``0`` to disable it. Resetting the OutHandler will set all ``enable`` bits to 0. Similarly, you can adjust the ``STALL`` state by setting or clearing the ``stall`` bit."]
    pub out_ctrl: OUT_CTRL,
    _reserved17: [u8; 3usize],
    #[doc = "0x44 - Status about the current state of the `OUT` endpoint."]
    pub out_status: OUT_STATUS,
    _reserved18: [u8; 3usize],
    #[doc = "0x48 - "]
    pub out_ev_status: OUT_EV_STATUS,
    _reserved19: [u8; 3usize],
    #[doc = "0x4c - "]
    pub out_ev_pending: OUT_EV_PENDING,
    _reserved20: [u8; 3usize],
    #[doc = "0x50 - "]
    pub out_ev_enable: OUT_EV_ENABLE,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pullup_out](pullup_out) module"]
pub type PULLUP_OUT = crate::Reg<u8, _PULLUP_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULLUP_OUT;
#[doc = "`read()` method returns [pullup_out::R](pullup_out::R) reader structure"]
impl crate::Readable for PULLUP_OUT {}
#[doc = "`write(|w| ..)` method takes [pullup_out::W](pullup_out::W) writer structure"]
impl crate::Writable for PULLUP_OUT {}
#[doc = ""]
pub mod pullup_out;
#[doc = "Sets the USB device address, in order to ignore packets going to other devices on the bus. This value is reset when the host issues a USB Device Reset condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [address](address) module"]
pub type ADDRESS = crate::Reg<u8, _ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRESS;
#[doc = "`read()` method returns [address::R](address::R) reader structure"]
impl crate::Readable for ADDRESS {}
#[doc = "`write(|w| ..)` method takes [address::W](address::W) writer structure"]
impl crate::Writable for ADDRESS {}
#[doc = "Sets the USB device address, in order to ignore packets going to other devices on the bus. This value is reset when the host issues a USB Device Reset condition."]
pub mod address;
#[doc = "In ``eptri``, there are three endpoints. It is possible for an IRQ to fire and have all three bits set. Under these circumstances it can be difficult to know which event to process first. Use this register to determine which event needs to be processed first. Only one bit will ever be set at a time.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [next_ev](next_ev) module"]
pub type NEXT_EV = crate::Reg<u8, _NEXT_EV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_EV;
#[doc = "`read()` method returns [next_ev::R](next_ev::R) reader structure"]
impl crate::Readable for NEXT_EV {}
#[doc = "In ``eptri``, there are three endpoints. It is possible for an IRQ to fire and have all three bits set. Under these circumstances it can be difficult to know which event to process first. Use this register to determine which event needs to be processed first. Only one bit will ever be set at a time."]
pub mod next_ev;
#[doc = "Data from the last ``SETUP`` transactions. It will be 10 bytes long, because it will include the CRC16. This is a FIFO, and the queue is advanced automatically.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_data](setup_data) module"]
pub type SETUP_DATA = crate::Reg<u8, _SETUP_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_DATA;
#[doc = "`read()` method returns [setup_data::R](setup_data::R) reader structure"]
impl crate::Readable for SETUP_DATA {}
#[doc = "Data from the last ``SETUP`` transactions. It will be 10 bytes long, because it will include the CRC16. This is a FIFO, and the queue is advanced automatically."]
pub mod setup_data;
#[doc = "Controls for managing how to handle ``SETUP`` transactions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_ctrl](setup_ctrl) module"]
pub type SETUP_CTRL = crate::Reg<u8, _SETUP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_CTRL;
#[doc = "`read()` method returns [setup_ctrl::R](setup_ctrl::R) reader structure"]
impl crate::Readable for SETUP_CTRL {}
#[doc = "`write(|w| ..)` method takes [setup_ctrl::W](setup_ctrl::W) writer structure"]
impl crate::Writable for SETUP_CTRL {}
#[doc = "Controls for managing how to handle ``SETUP`` transactions."]
pub mod setup_ctrl;
#[doc = "Status about the most recent ``SETUP`` transactions, and the state of the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_status](setup_status) module"]
pub type SETUP_STATUS = crate::Reg<u8, _SETUP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_STATUS;
#[doc = "`read()` method returns [setup_status::R](setup_status::R) reader structure"]
impl crate::Readable for SETUP_STATUS {}
#[doc = "Status about the most recent ``SETUP`` transactions, and the state of the FIFO."]
pub mod setup_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_ev_status](setup_ev_status) module"]
pub type SETUP_EV_STATUS = crate::Reg<u8, _SETUP_EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_EV_STATUS;
#[doc = "`read()` method returns [setup_ev_status::R](setup_ev_status::R) reader structure"]
impl crate::Readable for SETUP_EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [setup_ev_status::W](setup_ev_status::W) writer structure"]
impl crate::Writable for SETUP_EV_STATUS {}
#[doc = ""]
pub mod setup_ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_ev_pending](setup_ev_pending) module"]
pub type SETUP_EV_PENDING = crate::Reg<u8, _SETUP_EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_EV_PENDING;
#[doc = "`read()` method returns [setup_ev_pending::R](setup_ev_pending::R) reader structure"]
impl crate::Readable for SETUP_EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [setup_ev_pending::W](setup_ev_pending::W) writer structure"]
impl crate::Writable for SETUP_EV_PENDING {}
#[doc = ""]
pub mod setup_ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [setup_ev_enable](setup_ev_enable) module"]
pub type SETUP_EV_ENABLE = crate::Reg<u8, _SETUP_EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETUP_EV_ENABLE;
#[doc = "`read()` method returns [setup_ev_enable::R](setup_ev_enable::R) reader structure"]
impl crate::Readable for SETUP_EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [setup_ev_enable::W](setup_ev_enable::W) writer structure"]
impl crate::Writable for SETUP_EV_ENABLE {}
#[doc = ""]
pub mod setup_ev_enable;
#[doc = "Each byte written into this register gets added to an outgoing FIFO. Any bytes that are written here will be transmitted in the order in which they were added. The FIFO queue is automatically advanced with each write. The FIFO queue is 64 bytes deep. If you exceed this amount, the result is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_data](in_data) module"]
pub type IN_DATA = crate::Reg<u8, _IN_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_DATA;
#[doc = "`read()` method returns [in_data::R](in_data::R) reader structure"]
impl crate::Readable for IN_DATA {}
#[doc = "`write(|w| ..)` method takes [in_data::W](in_data::W) writer structure"]
impl crate::Writable for IN_DATA {}
#[doc = "Each byte written into this register gets added to an outgoing FIFO. Any bytes that are written here will be transmitted in the order in which they were added. The FIFO queue is automatically advanced with each write. The FIFO queue is 64 bytes deep. If you exceed this amount, the result is undefined."]
pub mod in_data;
#[doc = "Enables transmission of data in response to ``IN`` tokens, or resets the contents of the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_ctrl](in_ctrl) module"]
pub type IN_CTRL = crate::Reg<u8, _IN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_CTRL;
#[doc = "`read()` method returns [in_ctrl::R](in_ctrl::R) reader structure"]
impl crate::Readable for IN_CTRL {}
#[doc = "`write(|w| ..)` method takes [in_ctrl::W](in_ctrl::W) writer structure"]
impl crate::Writable for IN_CTRL {}
#[doc = "Enables transmission of data in response to ``IN`` tokens, or resets the contents of the FIFO."]
pub mod in_ctrl;
#[doc = "Status about the IN handler. As soon as you write to `IN_DATA`, ``IN_STATUS.HAVE`` should go to ``1``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_status](in_status) module"]
pub type IN_STATUS = crate::Reg<u8, _IN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_STATUS;
#[doc = "`read()` method returns [in_status::R](in_status::R) reader structure"]
impl crate::Readable for IN_STATUS {}
#[doc = "Status about the IN handler. As soon as you write to `IN_DATA`, ``IN_STATUS.HAVE`` should go to ``1``."]
pub mod in_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_ev_status](in_ev_status) module"]
pub type IN_EV_STATUS = crate::Reg<u8, _IN_EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_EV_STATUS;
#[doc = "`read()` method returns [in_ev_status::R](in_ev_status::R) reader structure"]
impl crate::Readable for IN_EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [in_ev_status::W](in_ev_status::W) writer structure"]
impl crate::Writable for IN_EV_STATUS {}
#[doc = ""]
pub mod in_ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_ev_pending](in_ev_pending) module"]
pub type IN_EV_PENDING = crate::Reg<u8, _IN_EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_EV_PENDING;
#[doc = "`read()` method returns [in_ev_pending::R](in_ev_pending::R) reader structure"]
impl crate::Readable for IN_EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [in_ev_pending::W](in_ev_pending::W) writer structure"]
impl crate::Writable for IN_EV_PENDING {}
#[doc = ""]
pub mod in_ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_ev_enable](in_ev_enable) module"]
pub type IN_EV_ENABLE = crate::Reg<u8, _IN_EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN_EV_ENABLE;
#[doc = "`read()` method returns [in_ev_enable::R](in_ev_enable::R) reader structure"]
impl crate::Readable for IN_EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [in_ev_enable::W](in_ev_enable::W) writer structure"]
impl crate::Writable for IN_EV_ENABLE {}
#[doc = ""]
pub mod in_ev_enable;
#[doc = "Data received from the host will go into a FIFO. This register reflects the contents of the top byte in that FIFO. Reading from this register advances the FIFO pointer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_data](out_data) module"]
pub type OUT_DATA = crate::Reg<u8, _OUT_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_DATA;
#[doc = "`read()` method returns [out_data::R](out_data::R) reader structure"]
impl crate::Readable for OUT_DATA {}
#[doc = "Data received from the host will go into a FIFO. This register reflects the contents of the top byte in that FIFO. Reading from this register advances the FIFO pointer."]
pub mod out_data;
#[doc = "Controls for receiving packet data. To enable an endpoint, write its value to ``epno``, with the ``enable`` bit set to ``1`` to enable an endpoint, or ``0`` to disable it. Resetting the OutHandler will set all ``enable`` bits to 0. Similarly, you can adjust the ``STALL`` state by setting or clearing the ``stall`` bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_ctrl](out_ctrl) module"]
pub type OUT_CTRL = crate::Reg<u8, _OUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CTRL;
#[doc = "`read()` method returns [out_ctrl::R](out_ctrl::R) reader structure"]
impl crate::Readable for OUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [out_ctrl::W](out_ctrl::W) writer structure"]
impl crate::Writable for OUT_CTRL {}
#[doc = "Controls for receiving packet data. To enable an endpoint, write its value to ``epno``, with the ``enable`` bit set to ``1`` to enable an endpoint, or ``0`` to disable it. Resetting the OutHandler will set all ``enable`` bits to 0. Similarly, you can adjust the ``STALL`` state by setting or clearing the ``stall`` bit."]
pub mod out_ctrl;
#[doc = "Status about the current state of the `OUT` endpoint.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_status](out_status) module"]
pub type OUT_STATUS = crate::Reg<u8, _OUT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_STATUS;
#[doc = "`read()` method returns [out_status::R](out_status::R) reader structure"]
impl crate::Readable for OUT_STATUS {}
#[doc = "Status about the current state of the `OUT` endpoint."]
pub mod out_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_ev_status](out_ev_status) module"]
pub type OUT_EV_STATUS = crate::Reg<u8, _OUT_EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EV_STATUS;
#[doc = "`read()` method returns [out_ev_status::R](out_ev_status::R) reader structure"]
impl crate::Readable for OUT_EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [out_ev_status::W](out_ev_status::W) writer structure"]
impl crate::Writable for OUT_EV_STATUS {}
#[doc = ""]
pub mod out_ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_ev_pending](out_ev_pending) module"]
pub type OUT_EV_PENDING = crate::Reg<u8, _OUT_EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EV_PENDING;
#[doc = "`read()` method returns [out_ev_pending::R](out_ev_pending::R) reader structure"]
impl crate::Readable for OUT_EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [out_ev_pending::W](out_ev_pending::W) writer structure"]
impl crate::Writable for OUT_EV_PENDING {}
#[doc = ""]
pub mod out_ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_ev_enable](out_ev_enable) module"]
pub type OUT_EV_ENABLE = crate::Reg<u8, _OUT_EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EV_ENABLE;
#[doc = "`read()` method returns [out_ev_enable::R](out_ev_enable::R) reader structure"]
impl crate::Readable for OUT_EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [out_ev_enable::W](out_ev_enable::W) writer structure"]
impl crate::Writable for OUT_EV_ENABLE {}
#[doc = ""]
pub mod out_ev_enable;
