use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;
use libc::c_void;

#[repr(C)]
pub struct __DASession(c_void);
pub type DASessionRef = *const __DASession;

extern "C" {
    pub fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
    pub fn DASessionScheduleWithRunLoop(
        session: DASessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
