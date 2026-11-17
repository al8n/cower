use crate::{Cow, CowRepr, ToOwned};
use compact_str_0_9::CompactString;

/// Maps `str` to [`CompactString`], inlining short strings and allocating longer ones.
///
/// Supports infallible promotion of local borrows. Requires an allocator.
#[cfg_attr(docsrs, doc(cfg(feature = "compact_str_0_9")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompactStrPolicy {}

impl ToOwned<CompactStrPolicy> for str {
  type Owned = CompactString;
  type Storage<'a> = CowRepr<'a, Self, CompactStrPolicy>;

  fn clone_into<'a>(&'a self, target: &mut CompactString)
  where
    CompactString: From<&'a Self>,
  {
    target.clear();
    target.push_str(self);
  }
}

impl From<CompactString> for Cow<'_, str, CompactStrPolicy> {
  fn from(value: CompactString) -> Self {
    Self::owned(value)
  }
}

impl<'a, P: ?Sized> From<&'a CompactString> for Cow<'a, str, P>
where
  str: ToOwned<P>,
{
  fn from(value: &'a CompactString) -> Self {
    Self::borrowed(value.as_str())
  }
}
