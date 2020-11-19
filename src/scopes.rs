use crate::errors::MethodHResult;

use winapi::um::combaseapi::{CoIncrementMTAUsage, CoDecrementMTAUsage, CO_MTA_USAGE_COOKIE};

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



/// [CoIncrementMTAUsage] (+ [CoDecrementMTAUsage])
/// Puts the current thread into the MTA, if the current thread is not already in an apartment
///
/// [CoIncrementMTAUsage]:  https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coincrementmtausage
/// [CoDecrementMTAUsage]:  https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-codecrementmtausage
pub struct MTAUsageScope(CO_MTA_USAGE_COOKIE);

impl MTAUsageScope {
    pub fn new() -> Result<Self, MethodHResult> {
        let mut cookie = null_mut();
        let hr = unsafe { CoIncrementMTAUsage(&mut cookie) };
        MethodHResult::check("CoIncrementMTAUsage", hr)?;
        Ok(Self(cookie))
    }
}

impl Debug for MTAUsageScope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "MTAUsageScope") }
}

impl Drop for MTAUsageScope {
    fn drop(&mut self) {
        let hr = unsafe { CoDecrementMTAUsage(self.0) };
        MethodHResult::check("CoDecrementMTAUsage", hr).unwrap();
    }
}
