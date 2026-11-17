use crate::{Cow, ToOwned};
use core::fmt::Debug;
use proptest::{
  arbitrary::Arbitrary,
  strategy::{Map, Strategy},
};

/// Maps the owned type's strategy, parameters, and shrink tree into owned Cows.
#[cfg_attr(docsrs, doc(cfg(feature = "proptest_1")))]
impl<'a, T, P> Arbitrary for Cow<'a, T, P>
where
  T: ?Sized + ToOwned<P>,
  P: ?Sized,
  T::Owned: Arbitrary,
  T::Storage<'a>: Debug,
{
  type Parameters = <T::Owned as Arbitrary>::Parameters;
  type Strategy = Map<<T::Owned as Arbitrary>::Strategy, fn(T::Owned) -> Self>;

  fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
    T::Owned::arbitrary_with(args).prop_map(Self::owned as fn(T::Owned) -> Self)
  }
}
