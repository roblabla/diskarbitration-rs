use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;
use core_foundation_sys::url::CFURLRef;
use libc::c_void;

use crate::disk::DADiskRef;
use crate::dissenter::DADissenterRef;
use crate::session::DASessionRef;

pub type DADiskMountOptions = u32;

pub static kDADiskMountOptionDefault: DADiskMountOptions = 0;
pub static kDADiskMountOptionWhole: DADiskMountOptions = 1;

pub type DADiskMountCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void),
>;

pub type DADiskUnmountOptions = u64;

pub static kDADiskUnmountOptionDefault: DADiskUnmountOptions = 0;
pub static kDADiskUnmountOptionWhole: DADiskUnmountOptions = 1;
pub static kDADiskUnmountOptionForce: DADiskUnmountOptions = 524288;

pub type DADiskUnmountCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, dissenter: DADissenterRef, context: *mut c_void),
>;

type DADiskDisappearedCallback =
    std::option::Option<extern "C" fn(disk: DADiskRef, context: *mut c_void)>;
type DADiskMountApprovalCallback =
    std::option::Option<extern "C" fn(disk: DADiskRef, context: *mut c_void) -> DADissenterRef>;

extern "C" {
    pub fn DADiskMountWithArguments(
        disk: DADiskRef,
        path: CFURLRef,
        options: DADiskMountOptions,
        callback: DADiskMountCallback,
        context: *mut c_void,
        arguments: *mut CFStringRef,
    );
    pub fn DADiskUnmount(
        disk: DADiskRef,
        options: DADiskUnmountOptions,
        callback: DADiskUnmountCallback,
        context: *mut c_void,
    );
    pub fn DARegisterDiskMountApprovalCallback(
        session: DASessionRef,
        r#match: CFDictionaryRef,
        callback: DADiskMountApprovalCallback,
        ctx: *const c_void,
    );
    pub fn DARegisterDiskDisappearedCallback(
        session: DASessionRef,
        r#match: CFDictionaryRef,
        callback: DADiskDisappearedCallback,
        ctx: *const c_void,
    );
    pub fn DAApprovalSessionScheduleWithRunLoop(
        session: DASessionRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );

    pub static kDADiskDescriptionMatchVolumeMountable: CFDictionaryRef;
}
