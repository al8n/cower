use crate::{Cow, CowRepr, ToOwned};
use arcstr_1::{ArcStr, Substr};

/// Maps `str` to [`ArcStr`], preserving cheap shared clones and static literals.
///
/// Local borrows promote infallibly. Owned strings are immutable; `to_mut`
/// permits replacing the handle, not editing its bytes. Requires an allocator
/// and pointer atomics, even when cower's `std` / `alloc` tiers are disabled.
#[cfg_attr(docsrs, doc(cfg(feature = "arcstr_1")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArcStrPolicy {}

/// Maps `str` to [`Substr`], preserving a shared parent and substring range.
///
/// Moving an existing substring retains its view without copying the text.
/// Promoting a local `&str` creates a new parent string. As with [`ArcStrPolicy`],
/// the bytes are immutable and the dependency needs an allocator and atomics.
#[cfg_attr(docsrs, doc(cfg(feature = "arcstr_1")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArcSubstrPolicy {}

macro_rules! storage {
  ($policy:ty, $owned:ty) => {
    impl ToOwned<$policy> for str {
      type Owned = $owned;
      type Storage<'a> = CowRepr<'a, Self, $policy>;
    }

    impl From<$owned> for Cow<'_, str, $policy> {
      fn from(value: $owned) -> Self {
        Self::owned(value)
      }
    }

    impl<'a, P: ?Sized> From<&'a $owned> for Cow<'a, str, P>
    where
      str: ToOwned<P>,
    {
      fn from(value: &'a $owned) -> Self {
        Self::borrowed(value.as_str())
      }
    }
  };
}

storage!(ArcStrPolicy, ArcStr);
storage!(ArcSubstrPolicy, Substr);
