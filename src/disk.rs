use libc::{c_void, c_char};
use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::url::CFURLRef;

use session::DASessionRef;

#[repr(C)]
pub struct __DADisk(c_void);
pub type DADiskRef = *const __DADisk;

extern {
    pub fn DADiskCreateFromVolumePath(allocator: CFAllocatorRef,
                                      session: DASessionRef, path: CFURLRef)
    -> DADiskRef;
    pub fn DADiskGetBSDName(disk: DADiskRef) -> *const c_char;
}
