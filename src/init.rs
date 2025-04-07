//! Wrappers around [CoInitializeEx] etc. for initializing COM.
//!
//! [CoInitializeEx]:   https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex

use crate::errors::MethodHResult;

use winapi::ctypes::c_void;
use winapi::shared::winerror::{S_OK, S_FALSE};
use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize, CoIncrementMTAUsage, CoDecrementMTAUsage, CO_MTA_USAGE_COOKIE};
use winapi::um::objbase::{COINIT, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED, COINIT_DISABLE_OLE1DDE, COINIT_SPEED_OVER_MEMORY};

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::ops::{BitOr, BitOrAssign};
use core::ptr::null_mut;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)\]
/// Initialize COM for this thread, creating an Apartment-Threaded apartment if necessary.
///
/// This is typically used for Win32 UI threads
///
/// ### Returns
///
/// *   `Ok(true)` - The COM library was initialized successfully on this thread.
/// *   `Ok(false)` - The COM library was already initialized on this thread.
/// *   `Err(e) if e == RPC_E_CHANGED_MODE` - A previous call to [CoInitializeEx] specified this thread belonged to an MTA Apartment.
///
/// [CoInitializeEx]:           https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex
pub fn sta() -> Result<bool, MethodHResult> { co_initialize_ex((), CoInit::STA) }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)\]
/// Initialize COM for this thread, creating a Multi-Threaded apartment if necessary.
///
/// ### Returns
///
/// *   `Ok(true)` - The COM library was initialized successfully on this thread.
/// *   `Ok(false)` - The COM library was already initialized on this thread.
/// *   `Err(e) if e == RPC_E_CHANGED_MODE` - A previous call to [CoInitializeEx] specified this thread belonged to an STA Apartment.
///
/// [CoInitializeEx]:           https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex
pub fn mta() -> Result<bool, MethodHResult> { co_initialize_ex((), CoInit::MTA) }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)\]
/// Initialize COM for this thread, creating an apartment if necessary.
///
/// ### Arguments
///
/// * `reserved` - This parameter is reserved and must be `()`
/// * `coinit` - Initialization flags
///
/// ### Returns
///
/// *   `Ok(true)` - The COM library was initialized successfully on this thread.
/// *   `Ok(false)` - The COM library was already initialized on this thread.
/// *   `Err(e) if e == RPC_E_CHANGED_MODE` - A previous call to [co_initialize_ex] specified the concurrency
///     model for this thread as multithread apartment (MTA). This could also indicate that a change from
///     neutral-threaded apartment to single-threaded apartment has occurred.
pub fn co_initialize_ex<'r>(reserved: impl Into<CoInitializeExReserved<'r>>, coinit: impl Into<CoInit>) -> Result<bool, MethodHResult> {
    let reserved = reserved.into().0;
    let coinit = coinit.into().0;
    let hr = unsafe { CoInitializeEx(reserved, coinit) };
    MethodHResult::check("CoInitializeEx", hr)?;
    match hr {
        S_FALSE     => Ok(false),
        S_OK        => Ok(true),
        _unexpected => {
            // XXX: Log?
            Ok(true)
        },
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)\]
/// Closes the COM library on the current thread, unloads all DLLs loaded by the thread, frees any other resources that
/// the thread maintains, and forces all RPC connections on the thread to close.
///
/// ### Safety
///
/// * Do not call this from within [DllMain]
/// * Various Rust wrappers probably rely on COM remaining initialized on this thread
///
/// [DllMain]:  https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
pub unsafe fn uninitialize() {
    CoUninitialize(); // no hresult to check
}



#[doc(hidden)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)\]
/// Reserved parameter for calling [co_initialize_ex] with.  Pass `()` instead.
pub struct CoInitializeExReserved<'p>(*mut c_void, PhantomData<&'p ()>);

impl<'p> Default for CoInitializeExReserved<'p> {
    fn default() -> Self { Self(null_mut(), PhantomData) }
}

impl<'p> From<()> for CoInitializeExReserved<'p> {
    fn from(_: ()) -> Self { Self(null_mut(), PhantomData) }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit)\]
/// COM apartment type + associated flags for calling [co_initialize_ex] with.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoInit(COINIT);

impl CoInit {
    /// See [co_initialize_ex] (this is just shorthand)
    pub fn init(self) -> Result<bool, MethodHResult> { co_initialize_ex((), self) }

    /// Initializes the thread for apartment-threaded object concurrency.
    pub const APARTMENT_THREADED    : CoInit = CoInit(COINIT_APARTMENTTHREADED);

    /// Initializes the thread for multithreaded object concurrency.
    pub const MULTI_THREADED        : CoInit = CoInit(COINIT_MULTITHREADED);

    /// Disables DDE for OLE1 support.
    pub const DISABLE_OLE1DDE       : CoInitFlag = CoInitFlag::DISABLE_OLE1DDE;

    /// Increase memory usage in an attempt to increase performance.
    pub const SPEED_OVER_MEMORY     : CoInitFlag = CoInitFlag::SPEED_OVER_MEMORY;

    // While I recommend APARTMENT_THREADED over APARTMENTTHREADED for readability,
    // I also want to enable a search-and-replace of "COINIT_" with "CoInit::"

    /// Initializes the thread for apartment-threaded object concurrency.
    #[doc(hidden)] pub const APARTMENTTHREADED  : CoInit = Self::APARTMENT_THREADED;

    /// Initializes the thread for multithreaded object concurrency.
    #[doc(hidden)] pub const MULTITHREADED      : CoInit = Self::MULTI_THREADED;

    /// Initializes the thread for apartment-threaded object concurrency.
    #[doc(hidden)] pub const STA                : CoInit = Self::APARTMENT_THREADED;

    /// Initializes the thread for multithreaded object concurrency.
    #[doc(hidden)] pub const MTA                : CoInit = Self::MULTI_THREADED;
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit)\]
/// Associated flags for calling [co_initialize_ex] with.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoInitFlag(COINIT);

impl CoInitFlag {
    /// Disables DDE for OLE1 support.
    pub const DISABLE_OLE1DDE       : CoInitFlag = CoInitFlag(COINIT_DISABLE_OLE1DDE);

    /// Increase memory usage in an attempt to increase performance.
    pub const SPEED_OVER_MEMORY     : CoInitFlag = CoInitFlag(COINIT_SPEED_OVER_MEMORY);
}

impl BitOrAssign<CoInitFlag> for CoInit     { fn bitor_assign(&mut self, rhs: CoInitFlag) { self.0 |= rhs.0; } }
impl BitOrAssign<CoInitFlag> for CoInitFlag { fn bitor_assign(&mut self, rhs: CoInitFlag) { self.0 |= rhs.0; } }
impl BitOr<CoInitFlag>  for CoInit      { fn bitor(self, rhs: CoInitFlag) -> Self::Output { CoInit(self.0 | rhs.0) } type Output = CoInit; }
impl BitOr<CoInit>      for CoInitFlag  { fn bitor(self, rhs: CoInit    ) -> Self::Output { CoInit(self.0 | rhs.0) } type Output = CoInit; }
impl BitOr<CoInitFlag>  for CoInitFlag  { fn bitor(self, rhs: CoInitFlag) -> Self::Output { CoInitFlag(self.0 | rhs.0) } type Output = CoInitFlag; }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coincrementmtausage)\]
/// Puts the current thread into the MTA, if the current thread is not already in an apartment
///
/// [CoDecrementMTAUsage]:  https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-codecrementmtausage
pub struct MTAUsageScope(CO_MTA_USAGE_COOKIE);

impl MTAUsageScope {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coincrementmtausage)\]
    /// Create and/or keep-alive the MTA, and put the current thread into the MTA if not already in an apartment.
    ///
    /// The CoIncrementMTAUsage function enables clients to create MTA workers and wait on them for completion before exiting the process.
    ///
    /// The CoIncrementMTAUsage function ensures that the system doesn't free resources related to MTA support., even if the MTA thread count goes to 0.
    ///
    /// ### Safety
    ///
    /// * Do not call this from within [DllMain]
    /// * Various Rust wrappers probably rely on COM remaining initialized on this thread, dropping this type might uninitialize.
    ///
    /// [DllMain]:  https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
    pub unsafe fn new() -> Result<Self, MethodHResult> {
        let mut cookie = null_mut();
        let hr = CoIncrementMTAUsage(&mut cookie);
        MethodHResult::check("CoIncrementMTAUsage", hr)?;
        Ok(Self(cookie))
    }
}

impl Debug for MTAUsageScope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "MTAUsageScope") }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-codecrementmtausage)\]
/// CoDecrementMTAUsage
impl Drop for MTAUsageScope {
    fn drop(&mut self) {
        let hr = unsafe { CoDecrementMTAUsage(self.0) };
        MethodHResult::check("CoDecrementMTAUsage", hr).unwrap();
    }
}
