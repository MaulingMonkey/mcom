use crate::AsIUnknown;
use crate::errors::MethodHResult;

use winapi::{Interface};
use winapi::shared::winerror::*;
use winapi::shared::guiddef::GUID;
use winapi::shared::wtypesbase::{CLSCTX, CLSCTX_INPROC_SERVER};
use winapi::um::combaseapi::{CoCreateInstance, CoCreateInstanceFromApp};
use winapi::um::objidlbase::MULTI_QI;
use winapi::um::unknwnbase::IUnknown;

use std::convert::TryInto;
use std::ptr::{NonNull, null_mut};
use std::ops::Deref;



/// A \![Send]+\![Sync] basic reference counting smart pointer residing within the current COM apartment.
#[repr(transparent)] pub struct Rc<I: Interface + AsIUnknown>(NonNull<I>);

impl<I: Interface + AsIUnknown> Rc<I> {
    pub unsafe fn from_raw_opt(ptr: *mut I) -> Option<Self> {
        Some(Self(NonNull::new(ptr)?))
    }

    pub unsafe fn from_raw(ptr: *mut I) -> Self {
        Self::from_raw_opt(ptr).unwrap()
    }

    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    pub unsafe fn borrow(ptr: &*mut I) -> &Option<Self> {
        std::mem::transmute(ptr)
    }

    /// [CoCreateInstance](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)\[[FromApp](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstancefromapp)\]
    pub unsafe fn co_create(clsid: GUID, outer: Option<&Rc<IUnknown>>) -> Result<Self, MethodHResult> {
        Self::co_create_impl(clsid, outer)
    }

    #[cfg(any(partition = "desktop", partition = "system", partition = "games"))]
    unsafe fn co_create_impl(clsid: GUID, outer: Option<&Rc<IUnknown>>) -> Result<Self, MethodHResult> {
        Self::co_create_instance(clsid, outer, CLSCTX_INPROC_SERVER)
    }

    #[cfg(not(any(partition = "desktop", partition = "system", partition = "games")))]
    unsafe fn co_create_impl(clsid: GUID, outer: Option<&Rc<IUnknown>>) -> Result<Self, MethodHResult> {
        Self::co_create_instance_from_app(clsid, outer, CLSCTX_INPROC_SERVER, ())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)\]
    #[cfg(any(partition = "desktop", partition="system", partition="games"))]
    unsafe fn co_create_instance(clsid: GUID, outer: Option<&Rc<IUnknown>>, clsctx: CLSCTX) -> Result<Self, MethodHResult> {
        let mut ptr = null_mut();
        let outer = outer.map_or(null_mut(), |unk| unk.as_iunknown_ptr());
        let hr = CoCreateInstance(&clsid, outer, clsctx, &I::uuidof(), &mut ptr);
        MethodHResult::check("CoCreateInstance", hr)?;
        Ok(Self::from_raw(ptr.cast()))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstancefromapp)\]
    #[cfg(windows = "8.0")]
    #[cfg(any(partition = "app", partition = "system"))]
    #[allow(dead_code)]
    unsafe fn co_create_instance_from_app(clsid: GUID, outer: Option<&Rc<IUnknown>>, clsctx: CLSCTX, reserved: ()) -> Result<Self, MethodHResult> {
        let mut mqi = [MULTI_QI { pIID: &clsid, pItf: null_mut(), hr: 0 }];
        co_create_instance_from_app(clsid, outer, clsctx, reserved, &mut mqi[..])?;
        let [mqi0] = mqi;
        MethodHResult::check("CoCreateInstanceFromApp(..., [0].hr)", mqi0.hr)?;
        Ok(Self::from_raw(mqi0.pItf.cast()))
    }

    pub fn as_ptr(&self) -> *mut I {
        self.0.as_ptr()
    }

    pub fn into_raw(self) -> *mut I {
        let p = self.as_ptr();
        std::mem::forget(self);
        p
    }
}

impl<I: Interface + AsIUnknown> Clone for Rc<I> {
    fn clone(&self) -> Self {
        let _old_rc = unsafe { self.as_iunknown().AddRef() };
        // XXX: Consider asserting if _old_rc > u32::MAX/3 to avoid RC overflows?
        Self(self.0)
    }
}

impl<I: Interface + AsIUnknown> Deref for Rc<I> {
    type Target = I;
    fn deref(&self) -> &Self::Target { unsafe { self.0.as_ref() } }
}

impl<I: Interface + AsIUnknown> Drop for Rc<I> {
    fn drop(&mut self) {
        let (unk, release) = {
            let unk = self.as_iunknown_ptr();
            let release = unsafe { (*(*unk).lpVtbl).Release };
            (unk, release)
        };
        unsafe { release(unk) };
    }
}

impl<I: Interface + AsIUnknown> AsRef<Rc<I>> for Rc<I> {
    fn as_ref(&self) -> &Self { self }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstancefromapp)\]
#[cfg(windows = "8.0")]
#[cfg(any(partition = "app", partition = "system"))]
#[allow(dead_code)]
unsafe fn co_create_instance_from_app(clsid: GUID, outer: Option<&Rc<IUnknown>>, clsctx: CLSCTX, _reserved: (), out: &mut [MULTI_QI]) -> Result<(), MethodHResult> {
    let count : u32 = out.len().try_into().map_err(|_| MethodHResult("co_create_instance_from_app", MAKE_HRESULT(SEVERITY_ERROR, FACILITY_NULL, ERROR_ARITHMETIC_OVERFLOW as _)))?;
    let outer = outer.map_or(null_mut(), |unk| unk.as_iunknown_ptr());
    let hr = CoCreateInstanceFromApp(&clsid, outer, clsctx, null_mut(), count, out.as_mut_ptr());
    MethodHResult::check("CoCreateInstanceFromApp", hr)?;
    Ok(())
}

#[test] fn layout() {
    use std::mem::*;
    use std::ffi::c_void;

    assert_eq!(align_of::<*const c_void>(), align_of::<Rc<IUnknown>>());
    assert_eq!(align_of::<*const c_void>(), align_of::<Option<Rc<IUnknown>>>());
    assert_eq!(align_of::<*const c_void>(), align_of::<&Option<Rc<IUnknown>>>());

    assert_eq!(size_of::<*const c_void>(), size_of::<Rc<IUnknown>>());
    assert_eq!(size_of::<*const c_void>(), size_of::<Option<Rc<IUnknown>>>());
    assert_eq!(size_of::<*const c_void>(), size_of::<&Option<Rc<IUnknown>>>());
}
