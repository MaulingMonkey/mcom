//! [MethodHResult]

use winapi::shared::winerror::{HRESULT, SUCCEEDED};

use std::fmt::{self, Debug, Display, Formatter};



/// An error about some specific method returning an [HRESULT](https://www.hresult.info/)
#[derive(Clone)]
pub struct MethodHResult(pub(crate) &'static str, pub(crate) HRESULT);

impl MethodHResult {
    /// Returns an `Err(MethodHResult(...))` if `!SUCCEEDED(hr)`
    pub fn check(method: &'static str, hr: HRESULT) -> Result<(), Self> {
        if SUCCEEDED(hr) {
            Ok(())
        } else {
            Err(MethodHResult(method, hr))
        }
    }

    /// Returns the [HRESULT] of the error
    pub fn hresult(&self) -> HRESULT { self.1 }

    /// Returns a link in the format of e.g. "[https://www.hresult.info/Search?q=0x80000005](https://www.hresult.info/Search?q=0x80000005)"
    pub fn hresult_info_search_link(&self) -> String { format!("https://www.hresult.info/Search?q=0x{:08x}", self.1 as u32) }
}

impl Debug   for MethodHResult { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "MethodHResult({:?}, 0x{:08x})", self.0, self.1 as u32) } }
impl Display for MethodHResult { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} failed with HRESULT == 0x{:08x}", self.0, self.1 as u32) } }

impl std::error::Error for MethodHResult {}
