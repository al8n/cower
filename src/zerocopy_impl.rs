use core::borrow::BorrowMut;

use crate::{Cow, OwnedStorage, ToOwned};
use zerocopy_0_8::{FromBytes, Immutable, IntoBytes, KnownLayout, TryCastError, TryFromBytes};

/// Checked byte views of the borrowed contents, independent of the policy.
#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy_0_8")))]
impl<'a, T: ?Sized + ToOwned<P> + TryFromBytes + KnownLayout + Immutable, P: ?Sized> Cow<'a, T, P> {
  /// Validates `source` as `T` and constructs a borrowed Cow without copying.
  ///
  /// Size, alignment, and validity errors are returned unchanged from zerocopy.
  /// No owned-conversion bound is needed. For slice DSTs with zero-sized
  /// elements, use [`Self::try_from_bytes_with_elems`] to specify their count.
  /// The view cannot outlive its input:
  ///
  /// ```compile_fail
  /// use cower::{Cow, StaticPolicy};
  /// fn extend(input: &[u8]) -> Cow<'static, [u8], StaticPolicy> {
  ///     Cow::try_from_bytes(input).unwrap()
  /// }
  /// ```
  #[inline]
  pub fn try_from_bytes(source: &'a [u8]) -> Result<Self, TryCastError<&'a [u8], T>> {
    T::try_ref_from_bytes(source).map(Self::borrowed)
  }

  /// Validates a `T` at the start of `source`, returning its Cow and the remainder.
  ///
  /// Slice DSTs consume the largest prefix that fits their layout. Use the
  /// `_with_elems` variant to specify a length or handle zero-sized elements.
  #[inline]
  #[allow(clippy::type_complexity)] // Keep zerocopy's tuple order and original error type explicit.
  pub fn try_from_prefix(source: &'a [u8]) -> Result<(Self, &'a [u8]), TryCastError<&'a [u8], T>> {
    T::try_ref_from_prefix(source).map(|(value, rest)| (Self::borrowed(value), rest))
  }

  /// Validates a `T` at the end of `source`, returning the remainder and its Cow.
  ///
  /// Slice DSTs consume the largest suffix that fits their layout. Use the
  /// `_with_elems` variant to specify a length or handle zero-sized elements.
  #[inline]
  #[allow(clippy::type_complexity)] // Keep zerocopy's tuple order and original error type explicit.
  pub fn try_from_suffix(source: &'a [u8]) -> Result<(&'a [u8], Self), TryCastError<&'a [u8], T>> {
    T::try_ref_from_suffix(source).map(|(rest, value)| (rest, Self::borrowed(value)))
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy_0_8")))]
impl<'a, T, P: ?Sized> Cow<'a, T, P>
where
  T: ?Sized + ToOwned<P> + TryFromBytes + KnownLayout<PointerMetadata = usize> + Immutable,
{
  /// Validates an exact slice DST with `count` trailing elements, including ZSTs.
  ///
  /// Returns zerocopy's original size, alignment, or validity error. `count`
  /// describes the borrowed view and is independent of any policy capacity.
  #[inline]
  pub fn try_from_bytes_with_elems(
    source: &'a [u8],
    count: usize,
  ) -> Result<Self, TryCastError<&'a [u8], T>> {
    T::try_ref_from_bytes_with_elems(source, count).map(Self::borrowed)
  }

  /// Validates a slice DST prefix with `count` trailing elements and returns the rest.
  #[inline]
  #[allow(clippy::type_complexity)] // Matches try_from_prefix, with an explicit element count.
  pub fn try_from_prefix_with_elems(
    source: &'a [u8],
    count: usize,
  ) -> Result<(Self, &'a [u8]), TryCastError<&'a [u8], T>> {
    T::try_ref_from_prefix_with_elems(source, count)
      .map(|(value, rest)| (Self::borrowed(value), rest))
  }

  /// Validates a slice DST suffix with `count` trailing elements and returns the rest.
  #[inline]
  #[allow(clippy::type_complexity)] // Matches try_from_suffix, with an explicit element count.
  pub fn try_from_suffix_with_elems(
    source: &'a [u8],
    count: usize,
  ) -> Result<(&'a [u8], Self), TryCastError<&'a [u8], T>> {
    T::try_ref_from_suffix_with_elems(source, count)
      .map(|(rest, value)| (rest, Self::borrowed(value)))
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy_0_8")))]
impl<T: ?Sized + ToOwned<P> + IntoBytes + Immutable, P: ?Sized> Cow<'_, T, P> {
  /// Views the contents as initialized bytes without copying or promoting them.
  ///
  /// The view describes `T`, not the Cow's pointers, variant, or owned handle.
  /// Native layout and byte order apply; this is not a portable serialization format.
  /// The wrapper itself does not implement raw byte-conversion traits:
  ///
  /// ```compile_fail
  /// fn needs_bytes<T: zerocopy_0_8::FromBytes>() {}
  /// needs_bytes::<cower::Cow<'static, [u8]>>();
  /// ```
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.as_ref().as_bytes()
  }
}

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy_0_8")))]
impl<'a, T: ?Sized + ToOwned<P> + IntoBytes + FromBytes, P: ?Sized> Cow<'a, T, P>
where
  T::Owned: BorrowMut<T>,
{
  /// Promotes to owned storage before exposing writable content bytes.
  ///
  /// `T: FromBytes` ensures arbitrary byte edits remain valid. The original
  /// borrowed data is never mutated. Shared owned containers detach through
  /// their `BorrowMut` implementation. Types such as `str` and `bool` cannot
  /// expose unrestricted mutable bytes:
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let mut value: Cow<'_, str> = Cow::borrowed("hello");
  /// value.to_mut_bytes()[0] = 0xff;
  /// ```
  #[inline]
  pub fn to_mut_bytes(&mut self) -> &mut [u8]
  where
    T::Storage<'a>: OwnedStorage<'a, T, P>,
  {
    self.to_mut().borrow_mut().as_mut_bytes()
  }

  /// Fallibly promotes to owned storage before exposing writable content bytes.
  ///
  /// A failed conversion preserves the original borrow and returns the policy's
  /// native error. As with [`Self::to_mut_bytes`], every byte pattern must be valid
  /// for `T`, and mutation goes through the owned type's `BorrowMut` implementation.
  #[inline]
  pub fn try_to_mut_bytes(&mut self) -> Result<&mut [u8], <T::Owned as TryFrom<&'a T>>::Error>
  where
    T::Owned: TryFrom<&'a T>,
  {
    Ok(self.try_to_mut()?.borrow_mut().as_mut_bytes())
  }
}
