use crate as mcom;
use crate::*;
use crate::errors::MethodHResult;

use winapi::Interface;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::cguid::CLSID_StdGlobalInterfaceTable;
use winapi::um::objidlbase::IGlobalInterfaceTable;

use alloc::sync::Arc;

use core::convert::TryFrom;
use core::num::NonZeroU32;
use core::marker::PhantomData;
use core::ptr::null_mut;



/// A [Send]+[Sync], [IGlobalInterfaceTable]-held interface pointer.
///
/// [IGlobalInterfaceTable]:        https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable
pub struct Git<I: Interface + AsIUnknown>(Arc<Cookie<I>>);

impl<I: Interface + AsIUnknown> Git<I> {
    /// Lazily marshal a COM interface for use in another thread.  May fail when converted back into an [Rc] if in another COM apartment.
    pub fn try_from_lazy(unk: impl AsRef<Rc<I>>) -> Result<Self, MethodHResult> {
        Cookie::new(unk.as_ref()).map(|c| Self(Arc::new(c)))
    }
}

unsafe impl<I: Interface + AsIUnknown> Send for Git<I> {}
/// ### Safety
///
/// The entire point of this [IGlobalInterfaceTable] wrapper, is that the resulting cookie can be safely shared between
/// multiple threads - each resolving their own, COM-apartment-specific interface pointer if necessary.  As such,
/// [Agile] should be safe to mark [Send]+[Sync].
///
/// [IGlobalInterfaceTable]:        https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable
unsafe impl<I: Interface + AsIUnknown> Sync for Git<I> {}

impl<I: Interface + AsIUnknown> Clone for Git<I> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<I: Interface + AsIUnknown> TryFrom<Rc<I>> for Git<I> {
    type Error = MethodHResult;
    fn try_from(src: Rc<I>) -> Result<Self, Self::Error> { Self::try_from_lazy(src) }
}

impl<I: Interface + AsIUnknown> TryFrom<&Rc<I>> for Git<I> {
    type Error = MethodHResult;
    fn try_from(src: &Rc<I>) -> Result<Self, Self::Error> { Self::try_from_lazy(src) }
}

impl<I: Interface + AsIUnknown> TryFrom<Git<I>> for Rc<I> {
    type Error = MethodHResult;
    fn try_from(src: Git<I>) -> Result<Self, Self::Error> { src.0.get() }
}

impl<I: Interface + AsIUnknown> TryFrom<&Git<I>> for Rc<I> {
    type Error = MethodHResult;
    fn try_from(src: &Git<I>) -> Result<Self, Self::Error> { src.0.get() }
}

impl<I: Interface + AsIUnknown> AsRef<Git<I>> for Git<I> {
    fn as_ref(&self) -> &Self { self }
}



#[repr(transparent)] struct Cookie<I: Interface + AsIUnknown> {
    /// "The value of an invalid cookie is 0."
    /// https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iglobalinterfacetable-registerinterfaceinglobal
    cookie:     NonZeroU32,
    phantom:    PhantomData<*const I>,
}

impl<I: Interface + AsIUnknown> Cookie<I> {
    fn new(rc: &Rc<I>) -> Result<Self, MethodHResult> {
        let unk = rc.as_iunknown_ptr();
        let iid = I::uuidof();
        let mut cookie = 0;
        let hr = with_git(|git| unsafe { git.RegisterInterfaceInGlobal(unk, &iid, &mut cookie) });
        MethodHResult::check("IGlobalInterfaceTable::RegisterInterfaceInGlobal", hr)?;
        NonZeroU32::new(cookie).ok_or(MethodHResult::unchecked("IGlobalInterfaceTable::RegisterInterfaceInGlobal", hr)).map(|cookie| Self { cookie, phantom: PhantomData })
    }

    fn get(&self) -> Result<Rc<I>, MethodHResult> {
        let cookie : u32 = self.cookie.into();
        let iid = I::uuidof();
        let mut int = null_mut();
        let hr = with_git(|git| unsafe { git.GetInterfaceFromGlobal(cookie, &iid, &mut int) });
        MethodHResult::check("IGlobalInterfaceTable::GetInterfaceFromGlobal", hr)?;
        unsafe { Rc::from_raw_opt(int.cast()) }.ok_or(MethodHResult::unchecked("IGlobalInterfaceTable::GetInterfaceFromGlobal", hr))
    }
}

impl<I: Interface + AsIUnknown> Drop for Cookie<I> {
    fn drop(&mut self) {
        let cookie : u32 = self.cookie.into();
        let hr = with_git(|git| unsafe { git.RevokeInterfaceFromGlobal(cookie) });
        assert!(SUCCEEDED(hr), "IGlobalInterfaceTable::RevokeInterfaceFromGlobal failed with HRESULT == 0x{:08x}", hr);
    }
}



#[cfg(feature = "std")] fn with_git<R>(f: impl FnOnce(&IGlobalInterfaceTable) -> R) -> R {
    std::thread_local! { static GIT : mcom::Rc<IGlobalInterfaceTable> = create_thread_git(); }
    GIT.with(|git| f(git))
}

// XXX: `Git` was introduced before `feature = "std"`.  Gating behind the feature would be a breaking change.
// As such, roll our own poorly tested thread local storage... but only if we don't have `feature = "std"`.
#[cfg(not(feature = "std"))] fn with_git<R>(f: impl FnOnce(&IGlobalInterfaceTable) -> R) -> R {
    use core::sync::atomic::{AtomicU32, Ordering::*};
    use winapi::um::processthreadsapi::*;

    let tls_slot = {
        static TLS_SLOT : AtomicU32 = AtomicU32::new(TLS_OUT_OF_INDEXES);
        let prev_slot = TLS_SLOT.load(Acquire);
        if prev_slot != TLS_OUT_OF_INDEXES {
            prev_slot
        } else {
            let new_slot = unsafe { TlsAlloc() };
            assert_ne!(new_slot, TLS_OUT_OF_INDEXES, "TlsAlloc failed");
            match TLS_SLOT.compare_exchange(TLS_OUT_OF_INDEXES, new_slot, SeqCst, SeqCst) {
                Ok(_old_slot) => {
                    debug_assert_eq!(_old_slot, TLS_OUT_OF_INDEXES, "TLS_SLOT.compare_exchange(TLS_OUT_OF_INDEXES, ...) should've only returned Ok(TLS_OUT_OF_INDEXES)");
                    new_slot
                },
                Err(new_slot_another_thread) => { // We lost a race with another thread
                    unsafe { TlsFree(new_slot) };
                    new_slot_another_thread
                },
            }
        }
    };

    let git = {
        let git : *mut IGlobalInterfaceTable = unsafe { TlsGetValue(tls_slot) }.cast();
        if git.is_null() {
            let git = create_thread_git();
            assert!(0 != unsafe { TlsSetValue(tls_slot, git.as_ptr().cast()) });
            git.into_raw()
        } else {
            git
        }
    };

    f(unsafe { &*git })
}

fn create_thread_git() -> mcom::Rc<IGlobalInterfaceTable> {
    unsafe { mcom::Rc::co_create(CLSID_StdGlobalInterfaceTable, None) }.unwrap()
}
