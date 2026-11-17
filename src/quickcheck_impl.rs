extern crate alloc;

use self::alloc::boxed::Box;
use crate::{Cow, ToOwned};
use quickcheck::{Arbitrary, Gen};

/// Generates owned values and forwards owned shrinking to `T::Owned`.
///
/// Borrowed values have no shrink candidates: arbitrary policies need not
/// support converting a borrow into owned storage.
#[cfg_attr(docsrs, doc(cfg(feature = "quickcheck_1")))]
impl<T, P> Arbitrary for Cow<'static, T, P>
where
  T: ?Sized + ToOwned<P> + 'static,
  P: ?Sized + 'static,
  T::Owned: Arbitrary,
  T::Storage<'static>: Clone,
{
  fn arbitrary(g: &mut Gen) -> Self {
    Self::owned(T::Owned::arbitrary(g))
  }

  fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
    match self.as_owned() {
      Some(value) => Box::new(value.shrink().map(Self::owned)),
      None => quickcheck::empty_shrinker(),
    }
  }
}
