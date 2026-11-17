#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

mod cow;
mod ops;
mod policy;
mod storage;

#[cfg(feature = "heapless")]
mod heapless_policy;

#[cfg(any(feature = "std", feature = "alloc"))]
mod convert;

#[cfg(feature = "std")]
mod std_support;

pub use cow::Cow;
pub use policy::{DefaultPolicy, StaticPolicy, ToOwned};
pub use storage::{CowRepr, CowStorage, OwnedStorage};

#[cfg(feature = "heapless")]
pub use heapless_policy::Heapless;

#[cfg(test)]
mod tests;
