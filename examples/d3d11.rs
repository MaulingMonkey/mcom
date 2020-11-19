use mcom::errors::MethodHResult;

use winapi::shared::winerror::REGDB_E_IIDNOTREG;
use winapi::um::d3dcommon::*;
use winapi::um::d3d11::*;

use std::convert::TryFrom;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};



static QUIT : AtomicBool = AtomicBool::new(false);

fn main() {
    mcom::init::sta().unwrap();

    let mut device = null_mut();
    let hr = unsafe { D3D11CreateDevice(null_mut(), D3D_DRIVER_TYPE_NULL, null_mut(), 0, null(), 0, D3D11_SDK_VERSION, &mut device, null_mut(), null_mut()) };
    MethodHResult::check("D3D11CreateDevice", hr).unwrap();
    let device = unsafe { mcom::Rc::from_raw(device) };

    // Fails - ID3D11Device doesn't implement a proxy type factory
    assert_eq!(REGDB_E_IIDNOTREG, mcom::Agile::try_from(&device).map(|_| ()).unwrap_err().hresult());
    assert_eq!(REGDB_E_IIDNOTREG, mcom::Agile::try_from_eager(&device).map(|_| ()).unwrap_err().hresult());

    // Succeeds - lazy marshaling means this will work as long as we stay in the same COM apartment
    let device = mcom::Agile::try_from_lazy(device).unwrap();
    let device = mcom::Rc::try_from(device).unwrap();
    let device = mcom::Agile::try_from_lazy(device).unwrap();
    let device = mcom::Rc::try_from(device).unwrap();

    // Succeeds - IGlobalInterfaceTable also uses lazy marshaling
    let device = mcom::Git::try_from(device).unwrap();
    let device = mcom::Rc::try_from(device).unwrap();
    let device = mcom::Git::try_from(device).unwrap();
    let device = mcom::Rc::try_from(device).unwrap();

    let device1 = mcom::Git::try_from(&device).unwrap();
    let device2 = mcom::Agile::try_from_lazy(device).unwrap();
    dev::spawn_pump_join(move ||{
        mcom::init::mta().unwrap();
        let device1 = mcom::Rc::try_from(device1);
        let device2 = mcom::Rc::try_from(device2);
        QUIT.store(true, Relaxed);
        let _ = device1.map(|_| ()).unwrap_err(); // Expected to fail - different COM apartment
        let _ = device2.map(|_| ()).unwrap_err(); // Expected to fail - different COM apartment
    });
}
