use core_foundation_sys::string::CFStringRef;
use libc::{c_int, c_void};

#[repr(C)]
pub struct __DADissenter(c_void);
pub type DADissenterRef = *const __DADissenter;

pub type DAReturn = c_int;

extern "C" {
    pub fn DADissenterGetStatus(dissenter: DADissenterRef) -> DAReturn;
    pub fn DADissenterGetStatusString(dissenter: DADissenterRef) -> CFStringRef;
}
