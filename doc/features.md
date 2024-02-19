# Crate Features

✔️ All windows version related features are enabled by default.<br>
❌ All crate interop features are disabled by default.

| Feature                       | Description   |
| ----------------------------- | ------------- |
|                               | **Expose APIs by required windows version.**  Highest version wins.
| ✔️ windows-latest            | Enable APIs that require the most recent version of Windows
| ✔️ windows-10                |
| ✔️ windows-8-1               | Enable APIs that require Windows 8.1 or later ([Agile])
| ✔️ windows-8                 |
| ✔️ windows-7                 |
| ✔️ windows-vista             |
| ✔️ windows-xp                |
| ✔️ windows-2000              | Enable APIs that require Windows 2000 or later (most of this crate)
|                               | **Expose APIs by windows environment.**  Recommend picking one at a time.
| ✔️ winapi-family-all         | Enable APIs available to absolutely everything at once
| ✔️ winapi-family-desktop-app | Enable APIs available to Desktop-only non-store apps
| ✔️ winapi-family-pc-app      | Enable APIs available to Desktop-only store apps
| ✔️ winapi-family-phone-app   | Enable APIs available to Phone-only apps
| ✔️ winapi-family-system      | Enable APIs available to Drivers and Tools
| ✔️ winapi-family-server      | Enable APIs available to Windows Server applications
| ✔️ winapi-family-games       | Enable APIs available to Games and Applications
|                               | **Interop with "peer" crates.**
| ❌ com-0-3                   | `com = "0.3"` interop (convert between [Rc]&lt;[IUnknown]&gt; ⮀ [com::interfaces::IUnknown])
| ❌ wio-0-2                   | `wio = "0.2"` interop (convert between [Rc] ⮀ [wio::com::ComPtr])



<!-- References -->

[IUnknown]:                     https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
[com::interfaces::IUnknown]:    https://docs.rs/com/0.3/com/interfaces/struct.IUnknown.html
[wio::com::ComPtr]:             https://docs.rs/wio/0.2/x86_64-pc-windows-msvc/wio/com/struct.ComPtr.html
