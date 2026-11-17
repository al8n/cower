use crate::{Cow, CowRepr, ToOwned};
use arrayvec_0_7::{ArrayString, ArrayVec};

/// Inline storage mapping `str` to [`ArrayString<N>`] and `[T]` to [`ArrayVec<T, N>`].
///
/// Works without an allocator. `N` counts bytes for strings and elements for
/// slices. Borrows may exceed `N`; use [`Cow::try_to_mut`] or
/// [`Cow::try_into_owned`] to report capacity errors without panicking.
/// A failed promotion leaves the borrow unchanged. Slice promotion requires
/// `T: Clone`, while borrowing and moving owned storage do not.
#[cfg_attr(docsrs, doc(cfg(feature = "arrayvec_0_7")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrayVecPolicy<const N: usize> {}

impl<const N: usize> ToOwned<ArrayVecPolicy<N>> for str {
  type Owned = ArrayString<N>;
  type Storage<'a> = CowRepr<'a, Self, ArrayVecPolicy<N>>;
}

impl<T, const N: usize> ToOwned<ArrayVecPolicy<N>> for [T] {
  type Owned = ArrayVec<T, N>;
  type Storage<'a>
    = CowRepr<'a, Self, ArrayVecPolicy<N>>
  where
    T: 'a;
}

impl<const N: usize> From<ArrayString<N>> for Cow<'_, str, ArrayVecPolicy<N>> {
  fn from(value: ArrayString<N>) -> Self {
    Self::owned(value)
  }
}

impl<T, const N: usize> From<ArrayVec<T, N>> for Cow<'_, [T], ArrayVecPolicy<N>> {
  fn from(value: ArrayVec<T, N>) -> Self {
    Self::owned(value)
  }
}

impl<'a, P: ?Sized, const N: usize> From<&'a ArrayString<N>> for Cow<'a, str, P>
where
  str: ToOwned<P>,
{
  fn from(value: &'a ArrayString<N>) -> Self {
    Self::borrowed(value.as_str())
  }
}

impl<'a, T, P: ?Sized, const N: usize> From<&'a ArrayVec<T, N>> for Cow<'a, [T], P>
where
  [T]: ToOwned<P>,
{
  fn from(value: &'a ArrayVec<T, N>) -> Self {
    Self::borrowed(value.as_slice())
  }
}
