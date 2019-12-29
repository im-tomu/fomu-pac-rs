#![doc = "Peripheral access API for FOMU microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "CTRL"]
pub struct CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRL {}
impl CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrl::RegisterBlock {
        0xe000_0000 as *const _
    }
}
impl Deref for CTRL {
    type Target = ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTRL::ptr() }
    }
}
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "LXSPI"]
pub struct LXSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LXSPI {}
impl LXSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lxspi::RegisterBlock {
        0xe000_7800 as *const _
    }
}
impl Deref for LXSPI {
    type Target = lxspi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LXSPI::ptr() }
    }
}
#[doc = "LXSPI"]
pub mod lxspi;
#[doc = "MESSIBLE"]
pub struct MESSIBLE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MESSIBLE {}
impl MESSIBLE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const messible::RegisterBlock {
        0xe000_8000 as *const _
    }
}
impl Deref for MESSIBLE {
    type Target = messible::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MESSIBLE::ptr() }
    }
}
#[doc = "MESSIBLE"]
pub mod messible;
#[doc = "REBOOT"]
pub struct REBOOT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REBOOT {}
impl REBOOT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const reboot::RegisterBlock {
        0xe000_6000 as *const _
    }
}
impl Deref for REBOOT {
    type Target = reboot::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*REBOOT::ptr() }
    }
}
#[doc = "REBOOT"]
pub mod reboot;
#[doc = "RGB"]
pub struct RGB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RGB {}
impl RGB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rgb::RegisterBlock {
        0xe000_6800 as *const _
    }
}
impl Deref for RGB {
    type Target = rgb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RGB::ptr() }
    }
}
#[doc = "RGB"]
pub mod rgb;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0xe000_2800 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TOUCH"]
pub struct TOUCH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TOUCH {}
impl TOUCH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const touch::RegisterBlock {
        0xe000_5800 as *const _
    }
}
impl Deref for TOUCH {
    type Target = touch::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TOUCH::ptr() }
    }
}
#[doc = "TOUCH"]
pub mod touch;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0xe000_4800 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "VERSION"]
pub struct VERSION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VERSION {}
impl VERSION {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const version::RegisterBlock {
        0xe000_7000 as *const _
    }
}
impl Deref for VERSION {
    type Target = version::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VERSION::ptr() }
    }
}
#[doc = "VERSION"]
pub mod version;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CTRL"]
    pub CTRL: CTRL,
    #[doc = "LXSPI"]
    pub LXSPI: LXSPI,
    #[doc = "MESSIBLE"]
    pub MESSIBLE: MESSIBLE,
    #[doc = "REBOOT"]
    pub REBOOT: REBOOT,
    #[doc = "RGB"]
    pub RGB: RGB,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TOUCH"]
    pub TOUCH: TOUCH,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "VERSION"]
    pub VERSION: VERSION,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CTRL: CTRL {
                _marker: PhantomData,
            },
            LXSPI: LXSPI {
                _marker: PhantomData,
            },
            MESSIBLE: MESSIBLE {
                _marker: PhantomData,
            },
            REBOOT: REBOOT {
                _marker: PhantomData,
            },
            RGB: RGB {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TOUCH: TOUCH {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            VERSION: VERSION {
                _marker: PhantomData,
            },
        }
    }
}
