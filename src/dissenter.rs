use libc::{c_void, c_int};
use core_foundation_sys::string::CFStringRef;

#[repr(C)]
pub struct __DADissenter(c_void);
pub type DADissenterRef = *const __DADissenter;

pub type DAReturn = c_int;

extern {
    pub fn DADissenterGetStatus(dissenter: DADissenterRef) -> DAReturn;
    pub fn DADissenterGetStatusString(dissenter: DADissenterRef) -> CFStringRef;
}
