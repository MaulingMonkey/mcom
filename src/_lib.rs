//! Mauling**M**onkey's **COM** interfaces/types/wrappers.
//!
//!
//!
//! # Smart Pointers
//!
//! | Type      | [Send]<br>[Sync]  | Lazy           | Eager         | Windows | Partition | Description |
//! | --------- | ----------------- | -------------- | ------------- | ------- | --------- | ----------- |
//! | [`Rc`]    | ❌&nbsp;no       | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">N/A</span> | <span style="opacity: 25%">2000+</span> | <span style="opacity: 25%">any</span> | Your basic, super vanilla, apartment &amp; thread-local COM pointer.
//! | [`Git`]   | ✔️&nbsp;yes      | ✔️&nbsp;yes    | ❌&nbsp;no    | <span style="opacity: 25%">2000+</span>    | <span style="opacity: 25%">any</span>    | [IGlobalInterfaceTable](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iglobalinterfacetable)-based COM pointer.
//! | [`Agile`] | ✔️&nbsp;yes      | ✔️&nbsp;yes    | ✔️&nbsp;yes   | ⚠️ **8.1+**                                | ❌ ~~games~~ | [IAgileReference](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iagilereference)-based COM pointer.
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
//! | Windows   | What version of windows is required for this interface.  Most APIs are marked 2000+ because that's what [docs.microsoft.com] says, but likely predate Windows 2000.
//! | Partition | `WINAPI_PARTITION_*` references, as selected by `winapi-family-*` features.  Allows hiding e.g. CoCreateInstance from UWP apps that must use CoCreateInstanceFromApp instead.  See `winapifamily.h` for more details.
//!
//!
//!
//! # Known Soundness Holes &amp; Undefined Behavior Bait
//!
//! [winapi#961](https://github.com/retep998/winapi-rs/pull/961) details some of these:
//! *   [Rc]&lt;[ID3D12FunctionReflection]&gt; and other smart pointer combinations compile,
//!     despite [ID3D12FunctionReflection] not implementing [IUnknown], resulting in UB if used.
//! *   Bogus [winapi::Interface] implementations can allow casting COM interfaces to unrelated rust structs
//! *   Bogus Deref&lt;Target=[IUnknown]&gt; implementations can return dangling vtables, fn ptrs, or the wrong object
//!
//! This crate is *technically* globally sound (you cannot construct the types that allow UB without `unsafe`),
//! but bogus/evil-but-safe implementations of winapi traits can make safe-looking code locally unsound (e.g. a "safe"
//! cast could invoke UB.)  While this does offend my sensibilities a bit, I will fix it if/when able (e.g. if winapi
//! can provide me with the necessary tools), and any *reasonable* implementor of such traits will be filling the file
//! with the unsafe keyword anyways.
//!
//!
//!
//! # Crate Features
//!
//! All features are enabled by default.  If you disable the defaults, you can be more specific:
//!
//! | Feature                   | Description   |
//! | ------------------------- | ------------- |
//! | windows-latest            | Enable APIs that require the most recent version of Windows
//! | windows-10                |
//! | windows-8-1               | Enable APIs that require Windows 8.1 or later ([Agile])
//! | windows-8                 |
//! | windows-7                 |
//! | windows-vista             |
//! | windows-xp                |
//! | windows-2000              | Enable APIs that require Windows 2000 or later (most of this crate)
//! | &nbsp; | |
//! | winapi-family-desktop-app | Enable APIs available to Desktop-only non-store apps
//! | winapi-family-pc-app      | Enable APIs available to Desktop-only store apps
//! | winapi-family-phone-app   | Enable APIs available to Phone-only apps
//! | winapi-family-system      | Enable APIs available to Drivers and Tools
//! | winapi-family-server      | Enable APIs available to Windows Server applications
//! | winapi-family-games       | Enable APIs available to Games and Applications
//!
//!
//!
//! <!-- References -->
//!
//! [winrt]:                        https://docs.rs/winrt/
//! [docs.microsoft.com]:           https://docs.microsoft.com/
//! [IUnknown]:                     https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
//! [ID3D12FunctionReflection]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12functionreflection
//! [winapi::Interface]:            https://docs.rs/winapi/0.3/winapi/trait.Interface.html

#![deny(missing_docs)]
#![deny(unreachable_patterns)]  // Likely missing "use winapi::shared::winerror::S_OK;" or similar
#![deny(non_snake_case)]        // Likely missing "use winapi::shared::winerror::S_OK;" or similar
#![cfg_attr(not(all(windows = "10", partition = "desktop")), allow(unused_imports))]

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
