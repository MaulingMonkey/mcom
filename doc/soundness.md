# Known Soundness Holes &amp; Undefined Behavior Bait

[winapi#961](https://github.com/retep998/winapi-rs/pull/961) details some of these:
*   [Rc]&lt;[ID3D12FunctionReflection]&gt; and other smart pointer combinations compile,
    despite [ID3D12FunctionReflection] not implementing [IUnknown], resulting in UB if used.
*   Bogus [winapi::Interface] implementations can allow casting COM interfaces to unrelated rust structs
*   Bogus [Deref]&lt;Target=[IUnknown]&gt; implementations can return dangling vtables, fn ptrs, or the wrong object

This crate is *technically* globally sound (you cannot construct the types that allow UB without `unsafe`),
but bogus/evil-but-safe implementations of winapi traits can make safe-looking code locally unsound (e.g. a "safe"
cast could invoke UB.)  While this does offend my sensibilities a bit, I will fix it if/when able (e.g. if winapi
can provide me with the necessary tools), and any *reasonable* implementor of such traits will be filling the file
with the unsafe keyword anyways.



<!-- References -->

[winapi::Interface]:            https://docs.rs/winapi/0.3/winapi/trait.Interface.html
[IUnknown]:                     https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown
[ID3D12FunctionReflection]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12functionreflection
