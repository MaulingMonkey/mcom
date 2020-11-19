use crate as mcom;
use crate::*;
use crate::errors::MethodHResult;

use winapi::Interface;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::cguid::CLSID_StdGlobalInterfaceTable;
use winapi::um::objidlbase::IGlobalInterfaceTable;

use std::convert::TryFrom;
use std::num::NonZeroU32;
use std::marker::PhantomData;
use std::ptr::null_mut;
use std::sync::Arc;



/// A [Send]+[Sync], [IGlobalInterfaceTable]-held interface pointer.
///
/// [IGlobalInterfaceTable]:        https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable
pub struct Git<I: Interface + AsIUnknown>(Arc<Cookie<I>>);

impl<I: Interface + AsIUnknown> Git<I> {
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
/// [IGlobalInterfaceTable]:        https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable
unsafe impl<I: Interface + AsIUnknown> Sync for Git<I> {}

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
    /// https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iglobalinterfacetable-registerinterfaceinglobal
    cookie:     NonZeroU32,
    phantom:    PhantomData<I>,
}

impl<I: Interface + AsIUnknown> Cookie<I> {
    fn new(rc: &Rc<I>) -> Result<Self, MethodHResult> {
        let unk = rc.as_iunknown_ptr();
        let iid = I::uuidof();
        let mut cookie = 0;
        let hr = GIT.with(|git| unsafe { git.RegisterInterfaceInGlobal(unk, &iid, &mut cookie) });
        MethodHResult::check("IGlobalInterfaceTable::RegisterInterfaceInGlobal", hr)?;
        NonZeroU32::new(cookie).ok_or(MethodHResult("IGlobalInterfaceTable::RegisterInterfaceInGlobal", hr)).map(|cookie| Self { cookie, phantom: PhantomData })
    }

    fn get(&self) -> Result<Rc<I>, MethodHResult> {
        let cookie : u32 = self.cookie.into();
        let iid = I::uuidof();
        let mut int = null_mut();
        let hr = GIT.with(|git| unsafe { git.GetInterfaceFromGlobal(cookie, &iid, &mut int) });
        MethodHResult::check("IGlobalInterfaceTable::GetInterfaceFromGlobal", hr)?;
        unsafe { Rc::from_raw_opt(int.cast()) }.ok_or(MethodHResult("IGlobalInterfaceTable::GetInterfaceFromGlobal", hr))
    }
}

impl<I: Interface + AsIUnknown> Drop for Cookie<I> {
    fn drop(&mut self) {
        let cookie : u32 = self.cookie.into();
        let hr = GIT.with(|git| unsafe { git.RevokeInterfaceFromGlobal(cookie) });
        assert!(SUCCEEDED(hr), "IGlobalInterfaceTable::RevokeInterfaceFromGlobal failed with HRESULT == 0x{:08x}", hr);
    }
}



thread_local! {
    static GIT : mcom::Rc<IGlobalInterfaceTable> = create_thread_git();
}

fn create_thread_git() -> mcom::Rc<IGlobalInterfaceTable> {
    unsafe { mcom::Rc::co_create(CLSID_StdGlobalInterfaceTable, None) }.unwrap()
}
