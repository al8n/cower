#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

mod const_api;
mod cow;
mod ops;
mod policy;
mod storage;

// Keep implementation paths readable while dependencies have versioned Cargo aliases.
#[cfg(feature = "arbitrary_1")]
extern crate arbitrary_1 as arbitrary;
#[cfg(feature = "heapless_0_9")]
extern crate heapless_0_9 as heapless;
#[cfg(feature = "proptest_1")]
extern crate proptest_1 as proptest;
#[cfg(feature = "quickcheck_1")]
extern crate quickcheck_1 as quickcheck;
#[cfg(feature = "rkyv_0_8")]
extern crate rkyv_0_8 as rkyv;
#[cfg(feature = "serde_1")]
extern crate serde_core_1 as serde_core;

#[cfg(feature = "arcstr_1")]
mod arcstr_policy;
#[cfg(feature = "arrayvec_0_7")]
mod arrayvec_policy;
#[cfg(feature = "borsh_1")]
mod borsh_impl;
#[cfg(feature = "bytemuck_1")]
mod bytemuck_impl;
#[cfg(feature = "bytes_1")]
mod bytes_policy;
#[cfg(feature = "compact_str_0_9")]
mod compact_str_policy;
#[cfg(feature = "ecow_0_3")]
mod ecow_policy;
#[cfg(feature = "schemars_1")]
mod schemars_impl;
#[cfg(feature = "smallvec_1")]
mod smallvec_policy;
#[cfg(feature = "smol-bytes_0_2")]
mod smol_bytes_policy;
#[cfg(feature = "zerocopy_0_8")]
mod zerocopy_impl;

#[cfg(feature = "arbitrary_1")]
mod arbitrary_impl;
#[cfg(feature = "proptest_1")]
mod proptest_impl;
#[cfg(feature = "quickcheck_1")]
mod quickcheck_impl;
#[cfg(feature = "rkyv_0_8")]
mod rkyv_impl;
#[cfg(feature = "serde_1")]
mod serde_impl;

#[cfg(feature = "heapless_0_9")]
mod heapless_policy;

#[cfg(any(feature = "std", feature = "alloc"))]
mod convert;

#[cfg(feature = "std")]
mod std_support;

pub use cow::{Cow, CowWithStorage};
pub use policy::{DefaultPolicy, StaticPolicy, ToOwned};
pub use storage::{CowRepr, CowStorage, OwnedStorage};

#[cfg(feature = "arcstr_1")]
pub use arcstr_policy::{ArcStrPolicy, ArcSubstrPolicy};
#[cfg(feature = "ecow_0_3")]
pub use ecow_policy::{EcoBytesPolicy, EcoPolicy};

#[cfg(feature = "arrayvec_0_7")]
pub use arrayvec_policy::ArrayVecPolicy;
#[cfg(feature = "bytes_1")]
pub use bytes_policy::BytesPolicy;
#[cfg(feature = "compact_str_0_9")]
pub use compact_str_policy::CompactStrPolicy;
#[cfg(feature = "smallvec_1")]
pub use smallvec_policy::SmallVecPolicy;
#[cfg(feature = "smol-bytes_0_2")]
pub use smol_bytes_policy::SmolBytesBufferPolicy;
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
pub use smol_bytes_policy::{SmolBytesCompactPolicy, SmolBytesMutPolicy, SmolBytesPolicy};

#[cfg(feature = "rkyv_0_8")]
pub use rkyv_impl::ArchivedCow;

#[cfg(feature = "heapless_0_9")]
pub use heapless_policy::Heapless;

#[cfg(test)]
mod tests;
