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
    /// Take ownership of a raw COM pointer.  [AddRef] will **not** be called.  [Release] **will* be called when this [Rc] is dropped.
    ///
    /// ### Safety
    ///
    /// * `ptr` may be null, in which case `None` will be returned.  Otherwise:
    /// * `ptr` must be a "valid" [IUnknown]-derived COM interface pointer, accessible from the current COM apartment.
    /// * `ptr` must remain valid until this [Rc] is dropped
    /// * `ptr.Release()` must be safe+sound when this [Rc] is dropped
    ///
    /// [AddRef]:           https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    /// [IUnknown]:         https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
    pub unsafe fn from_raw_opt(ptr: *mut I) -> Option<Self> {
        Some(Self(NonNull::new(ptr)?))
    }

    /// Take ownership of a raw COM pointer.  [AddRef] will **not** be called.  [Release] **will* be called when this [Rc] is dropped.
    ///
    /// ### Safety
    ///
    /// * `ptr` may be null, but this will result in a panic.  Otherwise:
    /// * `ptr` must be a "valid" [IUnknown]-derived COM interface pointer, accessible from the current COM apartment.
    /// * `ptr` must remain valid until this [Rc] is dropped
    /// * `ptr.Release()` must be safe+sound when this [Rc] is dropped
    ///
    /// [AddRef]:           https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    /// [IUnknown]:         https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
    pub unsafe fn from_raw(ptr: *mut I) -> Self {
        Self::from_raw_opt(ptr).unwrap()
    }

    /// Take ownership of a raw COM pointer.  [AddRef] will **not** be called.  [Release] **will* be called when this [Rc] is dropped.
    ///
    /// ### Safety
    ///
    /// * `ptr` **must not** be null, on pain of undefined behavior.
    /// * `ptr` must be a "valid" [IUnknown]-derived COM interface pointer, accessible from the current COM apartment.
    /// * `ptr` must remain valid until this [Rc] is dropped
    /// * `ptr.Release()` must be safe+sound when this [Rc] is dropped
    ///
    /// [AddRef]:           https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    /// [IUnknown]:         https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    /// Borrow a raw COM pointer.  [AddRef] will **not** be called.  [Release] will not be called either, as this returns a transmuted reference.
    ///
    /// ### Safety
    ///
    /// * `ptr` may be null, in which case `None` will be returned.  Otherwise:
    /// * `ptr` must be a "valid" [IUnknown]-derived COM interface pointer, accessible from the current COM apartment.
    /// * `ptr` must remain valid until the &[Rc] goes out of scope.
    ///
    /// [AddRef]:           https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    /// [IUnknown]:         https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
    pub unsafe fn borrow(ptr: &*mut I) -> &Option<Self> {
        std::mem::transmute(ptr)
    }

    /// [CoCreateInstance](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)\[[FromApp](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstancefromapp)\]
    ///
    /// ### Safety
    ///
    /// * `I` must have sane implementations of [Interface] and [AsIUnknown] (see [winapi#961] for what can happen with improper implementations!)
    /// * `clsid` and `I` are assumed to be well behaved COM APIs.  This is probably a bad assumption, but a failure to be so is a bug in C++ code, not Rust code.
    ///
    /// [winapi#961]:       https://github.com/retep998/winapi-rs/pull/961
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))\]
    ///
    /// Queries a COM object for a pointer to one of its interface; identifying the interface by a reference to its interface identifier (IID).
    pub fn try_cast<I2: Interface + AsIUnknown>(&self) -> Option<Rc<I2>> {
        let mut ptr = null_mut();
        let hr = unsafe { self.0.as_ref().as_iunknown().QueryInterface(&I2::uuidof(), &mut ptr) };
        // hr should be S_OK or E_NONTERFACE
        if !SUCCEEDED(hr) { return None; }
        unsafe { Rc::from_raw_opt(ptr.cast()) }
    }

    /// Retrieve a raw pointer for passing to COM APIs.  This [Rc] maintains ownership of the pointer.
    pub fn as_ptr(&self) -> *mut I {
        self.0.as_ptr()
    }

    /// Convert this smart pointer into a raw COM API pointer without [Release]ing it.
    /// This is a potential memory leak if the function this pointer was passed to did not assume ownership.
    ///
    /// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    pub fn into_raw(self) -> *mut I {
        let p = self.as_ptr();
        std::mem::forget(self);
        p
    }
}

impl<I: Interface + AsIUnknown + Deref> Rc<I> where I::Target : Interface + AsIUnknown + Sized {
    /// Cast up the COM inheritence tree
    pub fn up(self) -> Rc<I::Target> {
        let raw = self.into_raw();
        let cast : *mut I::Target = raw.cast();
        let deref : *const I::Target = unsafe { &**raw };
        assert_eq!(cast as *const _, deref);
        unsafe { Rc::from_raw(cast) }
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
