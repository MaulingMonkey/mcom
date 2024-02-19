//! [Crate Features](features),
//! [Known Soundness Holes &amp; Undefined Behavior Bait](soundness)

use crate::*;
use core::ops::Deref;

macro_rules! docs {
    ( $($ident:ident),+ $(,)? ) => {$(
        #[doc = include_str!(concat!(stringify!($ident), ".md"))] pub mod $ident {}
    )+};
}

docs! {
    features,
    soundness,
}
