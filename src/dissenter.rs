use libc::c_void;

#[repr(C)]
pub struct __DADissenter(c_void);
pub type DADissenterRef = *const __DADissenter;
