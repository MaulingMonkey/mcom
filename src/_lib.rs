//! Mauling**M**onkey's **COM** interfaces/types/wrappers.
//!
//!
//!
//! # Smart Pointers
//!
//! | Type      | [Send]<br>[Sync]  | Lazy           | Eager         | Windows | Partition | Description |
//! | --------- | ----------------- | -------------- | ------------- | ------- | --------- | ----------- |
//! | [`Rc`]    | ❌&nbsp;no       | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">2000+</span> | <span style="opacity: 25%">any</span> | Your basic, super vanilla, apartment &amp; thread-local COM pointer.
//! | [`Git`]   | ✔️&nbsp;yes      | ✔️&nbsp;yes    | ❌&nbsp;no    | <span style="opacity: 25%">2000+</span>    | <span style="opacity: 25%">any</span>    | [IGlobalInterfaceTable](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable)-based COM pointer.
//! | [`Agile`] | ✔️&nbsp;yes      | ✔️&nbsp;yes    | ✔️&nbsp;yes   | ⚠️ **8.1+**                                | ❌ ~~games~~ | [IAgileReference](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iagilereference)-based COM pointer.
//!
//! COM interfaces have complicated thread safety guarantees - when they have thread safety guarantees at all.
//! While some of those guarantees can be expressed through the type system (e.g. many/most WinRT interfaces are guaranteed agile),
//! many of the more subtle ones can only be effectively expressed at runtime.
//!
//! For example, an interface pointer to a given COM object might be usable on other threads of a MTA apartment, but not on STA threads.
//! While we could have some kind of MTA token type only constructable on MTA threads, and require that for accessing the smart pointer,
//! such a token would likely need to be retrieved at runtime anyways - and just add more steps an extra complexity.
//!
//! Since WinRT's free-threaded agile guarantees are the only ones that really line up well with WinRT's type system,
//! and WinRT's rich metadata calls for WinRT-specific crates that can tackle that (e.g. [winrt]), that leaves this
//! crate to tackle the messier runtime-enforced thread safety of more vanilla COM.
//!
//! All these smart pointers assume the COM interface implements [IUnknown].
//! It's worth noting that some "COM" interfaces like [ID3D12FunctionReflection] do not implement [IUnknown], and cannot
//! be held in any of these COM smart pointers as a result.
//!
//! | Legend    | Description |
//! | --------- | ----------- |
//! | Lazy      | Errors out when converting [Agile]/[Git] 🠆 [Rc] if the COM object is being being used in another COM Apartment, unless it is safe to do so.  This is usable even for unmarshalable types (such as Direct3D interfaces) if you stay within the appropriate COM apartment and they implement [IUnknown].
//! | Eager     | Errors out when converting [Rc] 🠆 [Agile]/[Git] if the COM object isn't portable between COM Apartments (e.g. not 100% thread safe) and cannot be wrapped in a marshaling layer that would make it portable.
//! | Windows   | What version of windows is required for this interface.  Most APIs are marked 2000+ because that's what [microsoft.com] says, but likely predate Windows 2000.
//! | Partition | `WINAPI_PARTITION_*` references, as selected by `winapi-family-*` features.  Allows hiding e.g. CoCreateInstance from UWP apps that must use CoCreateInstanceFromApp instead.  See `winapifamily.h` for more details.
//!
//!
//!
//! [microsoft.com]:                https://learn.microsoft.com/
//! [IUnknown]:                     https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
//! [ID3D12FunctionReflection]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12functionreflection
//!
//! [winrt]:                        https://docs.rs/winrt/

#![no_std]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]  // Likely missing "use winapi::shared::winerror::S_OK;" or similar
#![deny(non_snake_case)]        // Likely missing "use winapi::shared::winerror::S_OK;" or similar
#![cfg_attr(not(all(windows = "10", partition = "desktop")), allow(unused_imports))]

extern crate alloc;
extern crate std;

#[cfg(doc)] #[path = "../doc/_doc.rs"] pub mod Documentation;

// smart pointers

#[cfg(all(windows = "8.1", any(partition = "app", partition = "system")))] mod agile;
#[cfg(all(windows = "8.1", any(partition = "app", partition = "system")))] pub use agile::Agile;

#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] mod git;
#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] pub use git::Git;

#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] mod rc;
#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] pub use rc::Rc;

// misc

#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] pub mod errors;
#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] pub mod init;

#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] mod interface;
#[cfg(all(windows = "2000", any(partition = "app", partition = "system", partition = "games")))] pub use interface::*;
