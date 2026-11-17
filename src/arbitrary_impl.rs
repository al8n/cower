use crate::{Cow, ToOwned};
use arbitrary::{Arbitrary, MaxRecursionReached, Result, Unstructured, size_hint};

/// Builds owned values from fuzz input using the policy's owned representation.
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary_1")))]
impl<'data, T, P> Arbitrary<'data> for Cow<'_, T, P>
where
  T: ?Sized + ToOwned<P>,
  P: ?Sized,
  T::Owned: Arbitrary<'data>,
{
  fn arbitrary(input: &mut Unstructured<'data>) -> Result<Self> {
    T::Owned::arbitrary(input).map(Self::owned)
  }

  fn arbitrary_take_rest(input: Unstructured<'data>) -> Result<Self> {
    T::Owned::arbitrary_take_rest(input).map(Self::owned)
  }

  fn size_hint(depth: usize) -> (usize, Option<usize>) {
    Self::try_size_hint(depth).unwrap_or_default()
  }

  fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
    size_hint::try_recursion_guard(depth, T::Owned::try_size_hint)
  }
}
