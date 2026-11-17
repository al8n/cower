use crate::{Cow, CowRepr, ToOwned};
use smallvec_1::{Array, SmallVec};

/// Maps `[T]` to [`SmallVec<[T; N]>`], spilling to the heap beyond `N` elements.
///
/// Promotion requires `T: Clone`; borrowing or moving existing storage does not.
/// Requires an allocator. This uninhabited marker is never constructed.
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec_1")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SmallVecPolicy<const N: usize> {}

impl<T, const N: usize> ToOwned<SmallVecPolicy<N>> for [T] {
  type Owned = SmallVec<[T; N]>;
  type Storage<'a>
    = CowRepr<'a, Self, SmallVecPolicy<N>>
  where
    T: 'a;
}

impl<T, const N: usize> From<SmallVec<[T; N]>> for Cow<'_, [T], SmallVecPolicy<N>> {
  fn from(value: SmallVec<[T; N]>) -> Self {
    Self::owned(value)
  }
}

impl<'a, A: Array, P: ?Sized> From<&'a SmallVec<A>> for Cow<'a, [A::Item], P>
where
  [A::Item]: ToOwned<P>,
{
  fn from(value: &'a SmallVec<A>) -> Self {
    Self::borrowed(value.as_slice())
  }
}
