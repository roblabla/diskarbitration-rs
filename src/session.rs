use libc::c_void;
use core_foundation_sys::base::CFAllocatorRef;

#[repr(C)]
pub struct __DASession(c_void);
pub type DASessionRef = *const __DASession;

extern {
    pub fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
}
