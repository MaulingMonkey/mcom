//! [MethodHResult]

use winapi::shared::winerror::{HRESULT, SUCCEEDED};

use winresult_types::HResult;

use std::fmt::{self, Debug, Display, Formatter};



/// An error about some specific method returning an [HRESULT](https://www.hresult.info/)
#[derive(Clone)]
pub struct MethodHResult {
    pub(crate) method:  &'static str,
    pub(crate) hr:      HResult,
}

impl MethodHResult {
    /// Returns an `Err(MethodHResult(...))` if `!SUCCEEDED(hr)`
    pub fn check(method: &'static str, hr: HRESULT) -> Result<(), Self> {
        if SUCCEEDED(hr) {
            Ok(())
        } else {
            Err(Self::unchecked(method, hr))
        }
    }

    /// Returns the [HRESULT] of the error
    pub fn hresult(&self) -> HRESULT { self.hr.to_u32() as _ }

    /// Returns a link in the format of e.g. "<https://www.hresult.info/Search?q=0x80000005>"
    pub fn hresult_info_search_link(&self) -> String { format!("https://www.hresult.info/Search?q=0x{:08x}", self.hr.to_u32()) }
}

impl MethodHResult {
    pub(crate) fn unchecked(method: &'static str, hr: HRESULT) -> Self { Self { method, hr: HResult::from(hr) } }
    pub(crate) fn to_u32(&self) -> u32 { self.hr.to_u32() }
}

impl Debug   for MethodHResult { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "MethodHResult({:?}, 0x{:08x})", self.method, self.to_u32()) } }
impl Display for MethodHResult { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed with HRESULT == 0x{:08x}", self.method, self.to_u32()) } }

impl std::error::Error for MethodHResult {}

impl From<MethodHResult> for HRESULT { fn from(value: MethodHResult) -> Self { value.hresult() } }
impl From<MethodHResult> for HResult { fn from(value: MethodHResult) -> Self { value.hr } }
