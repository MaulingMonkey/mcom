#[allow(unused_imports)] use crate::Git;
use crate::{AsIUnknown, Rc};
use crate::errors::MethodHResult;

use winapi::Interface;
use winapi::um::combaseapi::{AGILEREFERENCE_DEFAULT, AGILEREFERENCE_DELAYEDMARSHAL, AgileReferenceOptions, RoGetAgileReference};
use winapi::um::objidlbase::IAgileReference;

use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::ptr::null_mut;



/// A [Send]+[Sync], [IAgileReference]-held interface pointer.
///
/// [IAgileReference]:          https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iagilereference
/// [IAgileObject]:             https://docs.microsoft.com/en-us/windows/win32/api/objidlbase/nn-objidlbase-iagileobject
pub struct Agile<I: Interface + AsIUnknown> {
    agile:      Rc<IAgileReference>,
    phantom:    PhantomData<*const I>,
}

impl<I: Interface + AsIUnknown> Agile<I> {
    /// Eagerly marshal a COM interface for use in another apartment.  Will fail if this is not possible.
    pub fn try_from_eager(unk: impl AsRef<Rc<I>>) -> Result<Self, MethodHResult> {
        Self::ro_get_agile_reference(ReferenceOptions::DEFAULT, unk)
    }

    /// Lazily marshal a COM interface for use in another thread.  May fail when converted back into an [Rc] if in another COM apartment.
    pub fn try_from_lazy(unk: impl AsRef<Rc<I>>) -> Result<Self, MethodHResult> {
        Self::ro_get_agile_reference(ReferenceOptions::DELAYED_MARSHAL, unk)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-rogetagilereference)\]
    /// Creates an agile reference for an object specified by the given interface.
    ///
    /// ### Arguments
    ///
    /// * `ro` - The registration options
    /// * `unk` - The COM object to wrap
    ///
    /// ### Returns
    ///
    /// * `Ok(Agile(...))` - Success!
    /// * `Err(MethodHResult("RoGetAgileReference", 0x80040155))` - aka `REGDB_E_IIDNOTREG` - The object is [missing a marshaller](https://devblogs.microsoft.com/oldnewthing/20090122-00/?p=19413)
    /// * `Err(MethodHResult("RoGetAgileReference", 0x80004021))` - aka `CO_E_NOT_SUPPORTED` - The object implements the [INoMarshal] interface.
    /// * `Err(...)` - aka `E_INVALIDARG` - The `options` parameter is invalid (impossible?)
    /// * `Err(...)` - aka `E_OUTOFMEMORY` - The agile reference couldn't be constructed due to an out-of-memory condition.
    /// * `Err(...)` - aka `E_NOINTERFACE` - The `unk` parameter doesn't support the interface ID specified by the riid parameter.
    ///
    /// [INoMarshal]:               https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-inomarshal
    fn ro_get_agile_reference(ro: impl Into<ReferenceOptions>, unk: impl AsRef<Rc<I>>) -> Result<Self, MethodHResult> {
        let ro = ro.into().0;
        let unk = unk.as_ref();
        let unk = unk.as_iunknown_ptr();
        let mut agile = null_mut();
        let hr = unsafe { RoGetAgileReference(ro, &I::uuidof(), unk, &mut agile) };
        MethodHResult::check("RoGetAgileReference", hr)?;
        let agile = unsafe { Rc::from_raw_opt(agile) }.ok_or(MethodHResult("RoGetAgileReference", hr))?;
        Ok(Self { agile, phantom: PhantomData })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iagilereference-resolve(refiid_void))\]
    /// Get a COM pointer to `I` that is safe to use from the current thread's COM apartment
    pub fn resolve(&self) -> Result<Rc<I>, MethodHResult> {
        let mut pv = null_mut();
        let hr = unsafe { self.agile.Resolve(&I::uuidof(), &mut pv) };
        MethodHResult::check("IAgileReference::Resolve", hr)?;
        let rc = unsafe { Rc::from_raw(pv.cast()) };
        Ok(rc)
    }
}

unsafe impl<I: Interface + AsIUnknown> Send for Agile<I> {}
/// ### Safety
///
/// The entire point of an [IAgileReference], like [Agile] contains is that it can be safely shared between multiple
/// threads, even if they don't belong to the same COM apartment, **even if the original interface pointer was
/// apartment-specific**.  As such, [Agile] should be safe to mark [Send]+[Sync].
///
/// * Types which implement [IAgileObject] will remain unwrapped
/// * Types which implement [INoMarshal] will fail to convert to [Agile] in the first place
/// * Types which implement neither will be wrapped in lightweight in-process-only marshaling
///
/// [IAgileReference]:          https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iagilereference
/// [IAgileObject]:             https://docs.microsoft.com/en-us/windows/win32/api/objidlbase/nn-objidlbase-iagileobject
/// [INoMarshal]:               https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-inomarshal
unsafe impl<I: Interface + AsIUnknown> Sync for Agile<I> {}

impl<I: Interface + AsIUnknown> Clone for Agile<I> {
    fn clone(&self) -> Self { Self { agile: self.agile.clone(), phantom: PhantomData } }
}

impl<I: Interface + AsIUnknown> TryFrom<Rc<I>> for Agile<I> {
    type Error = MethodHResult;
    fn try_from(value: Rc<I>) -> Result<Self, Self::Error> { Self::try_from_eager(value) }
}

impl<I: Interface + AsIUnknown> TryFrom<&Rc<I>> for Agile<I> {
    type Error = MethodHResult;
    fn try_from(value: &Rc<I>) -> Result<Self, Self::Error> { Self::try_from_eager(value) }
}

impl<I: Interface + AsIUnknown> TryFrom<Agile<I>> for Rc<I> {
    type Error = MethodHResult;
    fn try_from(value: Agile<I>) -> Result<Self, Self::Error> { value.resolve() }
}

impl<I: Interface + AsIUnknown> TryFrom<&Agile<I>> for Rc<I> {
    type Error = MethodHResult;
    fn try_from(value: &Agile<I>) -> Result<Self, Self::Error> { value.resolve() }
}

impl<I: Interface + AsIUnknown> AsRef<Agile<I>> for Agile<I> {
    fn as_ref(&self) -> &Self { self }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/ne-combaseapi-agilereferenceoptions)\]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReferenceOptions(u32);

impl ReferenceOptions {
    pub const DEFAULT           : ReferenceOptions = ReferenceOptions(AGILEREFERENCE_DEFAULT);
    pub const DELAYED_MARSHAL   : ReferenceOptions = ReferenceOptions(AGILEREFERENCE_DELAYEDMARSHAL);
}

impl Debug for ReferenceOptions {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ReferenceOptions::DEFAULT           => write!(f, "ReferenceOptions::DEFAULT"),
            ReferenceOptions::DELAYED_MARSHAL   => write!(f, "ReferenceOptions::DELAYED_MARSHAL"),
            other                               => write!(f, "ReferenceOptions(0x{:08x})", other.0),
        }
    }
}

impl Default for ReferenceOptions {
    fn default() -> Self { ReferenceOptions::DEFAULT }
}

impl From<ReferenceOptions> for AgileReferenceOptions {
    fn from(ro: ReferenceOptions) -> Self { ro.0 }
}

impl From<()> for ReferenceOptions {
    fn from(_: ()) -> Self { ReferenceOptions::DEFAULT }
}
