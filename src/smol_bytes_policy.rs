use crate::{Cow, CowRepr, ToOwned};
use smol_bytes_0_2 as smol_bytes;

macro_rules! storage {
  ($policy:ty, $borrowed:ty, $owned:ty) => {
    impl ToOwned<$policy> for $borrowed {
      type Owned = $owned;
      type Storage<'a> = CowRepr<'a, Self, $policy>;
    }

    impl From<$owned> for Cow<'_, $borrowed, $policy> {
      fn from(value: $owned) -> Self {
        Self::owned(value)
      }
    }

    impl<'a, P: ?Sized> From<&'a $owned> for Cow<'a, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn from(value: &'a $owned) -> Self {
        Self::borrowed(core::borrow::Borrow::borrow(value))
      }
    }
  };
}

/// Fixed inline storage using `smol-bytes` 0.2's [`smol_bytes::Buffer`] and
/// [`smol_bytes::Utf8Buffer`] for `[u8]` and `str` respectively.
///
/// Works without `std`, `alloc`, or atomics. Capacity is
/// [`smol_bytes::INLINE_CAP`] bytes, including UTF-8 strings. Use
/// [`Cow::try_to_mut`] or [`Cow::try_into_owned`] to report capacity errors.
/// Failed promotion preserves the borrow, which may exceed the inline capacity.
#[cfg_attr(docsrs, doc(cfg(feature = "smol-bytes_0_2")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SmolBytesBufferPolicy {}

storage!(SmolBytesBufferPolicy, [u8], smol_bytes::Buffer);
storage!(SmolBytesBufferPolicy, str, smol_bytes::Utf8Buffer);

#[cfg(any(feature = "std", feature = "alloc"))]
mod growable {
  use super::*;

  /// Shared storage using [`smol_bytes::Bytes`] / [`smol_bytes::Utf8Bytes`].
  ///
  /// Small values are inline; heap-backed clones share their allocation.
  /// Local borrows promote infallibly. The owned types provide immutable data
  /// views; use [`SmolBytesMutPolicy`] when growable mutation is needed.
  #[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc"))))
  )]
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum SmolBytesPolicy {}

  /// Compact storage using `smol-bytes`' compact byte and UTF-8 types.
  ///
  /// Local borrows promote infallibly. Shrinking a heap value can move it back
  /// inline, following the upstream compact strategy.
  #[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc"))))
  )]
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum SmolBytesCompactPolicy {}

  /// Mutable storage using [`smol_bytes::BytesMut`] / [`smol_bytes::Utf8BytesMut`].
  ///
  /// Supports infallible promotion and growable edits, starting inline and
  /// allocating as needed. Moving an existing buffer preserves its storage.
  #[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc"))))
  )]
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum SmolBytesMutPolicy {}

  storage!(SmolBytesPolicy, [u8], smol_bytes::Bytes);
  storage!(SmolBytesPolicy, str, smol_bytes::Utf8Bytes);
  storage!(SmolBytesCompactPolicy, [u8], smol_bytes::compact::Bytes);
  storage!(SmolBytesCompactPolicy, str, smol_bytes::compact::Utf8Bytes);
  storage!(SmolBytesMutPolicy, [u8], smol_bytes::BytesMut);
  storage!(SmolBytesMutPolicy, str, smol_bytes::Utf8BytesMut);
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub use growable::{SmolBytesCompactPolicy, SmolBytesMutPolicy, SmolBytesPolicy};
