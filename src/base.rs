use libc::c_void;
use core_foundation_sys::url::CFURLRef;
use core_foundation_sys::string::CFStringRef;

use disk::DADiskRef;
use dissenter::DADissenterRef;

pub type DADiskMountOptions = u32;

pub static kDADiskMountOptionDefault: DADiskMountOptions = 0;
pub static kDADiskMountOptionWhole: DADiskMountOptions = 1;

pub type DADiskMountCallback =
    ::std::option::Option<unsafe extern "C" fn(disk: DADiskRef,
                                               dissenter: DADissenterRef,
                                               context: *mut c_void)>;

pub type DADiskUnmountOptions = u64;

pub static kDADiskUnmountOptionDefault: DADiskUnmountOptions = 0;
pub static kDADiskUnmountOptionWhole: DADiskUnmountOptions = 1;
pub static kDADiskUnmountOptionForce: DADiskUnmountOptions = 524288;

pub type DADiskUnmountCallback =
    ::std::option::Option<unsafe extern "C" fn(disk: DADiskRef,
                                               dissenter: DADissenterRef,
                                               context: *mut c_void)>;

extern {
    pub fn DADiskMountWithArguments(disk: DADiskRef, path: CFURLRef,
                                    options: DADiskMountOptions,
                                    callback: DADiskMountCallback,
                                    context: *mut c_void,
                                    arguments: *mut CFStringRef);
    pub fn DADiskUnmount(disk: DADiskRef, options: DADiskUnmountOptions,
                         callback: DADiskUnmountCallback,
                         context: *mut c_void);
}
