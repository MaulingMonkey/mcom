use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;



#[doc(hidden)]
// XXX: Ehh... this type may not remain exposed by mcom / might be subsumed into winapi.
// See https://github.com/retep998/winapi-rs/pull/961 for some of the details.
pub unsafe trait AsIUnknown {
    fn as_iunknown(&self) -> &IUnknown;

    fn as_iunknown_ptr(&self) -> *mut IUnknown {
        self.as_iunknown() as *const IUnknown as *mut IUnknown
    }
}

unsafe impl<I: Interface> AsIUnknown for I {
    fn as_iunknown(&self) -> &IUnknown {
        unsafe { &*(self as *const Self as *const IUnknown) }
    }
}
