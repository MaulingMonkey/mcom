use winapi::shared::d3d9::*;
use winapi::shared::winerror::REGDB_E_IIDNOTREG;

use std::convert::TryFrom;



#[test] fn d3d9() {
    mcom::init::sta().unwrap();
    let d3d9 = unsafe { mcom::Rc::from_raw(Direct3DCreate9(D3D_SDK_VERSION)) };

    // Fails - IDirect3D9 doesn't implement a proxy type factory
    assert_eq!(REGDB_E_IIDNOTREG, mcom::Agile::try_from(&d3d9).map(|_| ()).unwrap_err().hresult());
    assert_eq!(REGDB_E_IIDNOTREG, mcom::Agile::try_from_eager(&d3d9).map(|_| ()).unwrap_err().hresult());

    // Succeeds - lazy marshaling means this will work as long as we stay in the same COM apartment
    let d3d9 = mcom::Agile::try_from_lazy(d3d9).unwrap();
    let d3d9 = mcom::Rc::try_from(d3d9).unwrap();
    let d3d9 = mcom::Agile::try_from_lazy(d3d9).unwrap();
    let d3d9 = mcom::Rc::try_from(d3d9).unwrap();

    // Succeeds - IGlobalInterfaceTable also uses lazy marshaling
    let d3d9 = mcom::Git::try_from(d3d9).unwrap();
    let d3d9 = mcom::Rc::try_from(d3d9).unwrap();
    let d3d9 = mcom::Git::try_from(d3d9).unwrap();
    let d3d9 = mcom::Rc::try_from(d3d9).unwrap();

    let d3d9 = mcom::Git::try_from(d3d9).unwrap();
    dev::spawn_pump_join(move ||{
        mcom::init::mta().unwrap();
        let d3d9 = mcom::Rc::try_from(d3d9);
        let _ = d3d9.map(|_| ()).unwrap_err(); // Expected to fail - different COM apartment
    });
}
