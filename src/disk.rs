use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::url::CFURLRef;
use libc::{c_char, c_void};

use crate::session::DASessionRef;

#[repr(C)]
pub struct __DADisk(c_void);
pub type DADiskRef = *const __DADisk;

extern "C" {
    pub fn DADiskCreateFromVolumePath(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        path: CFURLRef,
    ) -> DADiskRef;
    pub fn DADiskGetBSDName(disk: DADiskRef) -> *const c_char;
}
