use crate::{Cow, CowRepr, ToOwned};
use ecow_0_3::{EcoBytes, EcoString, EcoVec};

/// Maps `str` to [`EcoString`] and `[T]` to [`EcoVec<T>`].
///
/// Cloning owned storage shares heap allocations; mutation detaches as needed.
/// Small strings are stored inline. Promotion of slices requires `T: Clone`,
/// while borrowing and moving existing vectors do not. Requires an allocator
/// and pointer atomics, independently of cower's `std` / `alloc` tiers.
#[cfg_attr(docsrs, doc(cfg(feature = "ecow_0_3")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EcoPolicy {}

/// Maps `[u8]` to [`EcoBytes`], with inline storage for small byte strings.
///
/// Heap-backed clones share storage and detach on mutation. Requires an
/// allocator and pointer atomics. Use [`EcoPolicy`] for general element types.
#[cfg_attr(docsrs, doc(cfg(feature = "ecow_0_3")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EcoBytesPolicy {}

impl ToOwned<EcoPolicy> for str {
  type Owned = EcoString;
  type Storage<'a> = CowRepr<'a, Self, EcoPolicy>;

  fn clone_into<'a>(&'a self, target: &mut EcoString)
  where
    EcoString: From<&'a Self>,
  {
    target.clear();
    target.push_str(self);
  }
}

impl<T> ToOwned<EcoPolicy> for [T] {
  type Owned = EcoVec<T>;
  type Storage<'a>
    = CowRepr<'a, Self, EcoPolicy>
  where
    T: 'a;
}

impl ToOwned<EcoBytesPolicy> for [u8] {
  type Owned = EcoBytes;
  type Storage<'a> = CowRepr<'a, Self, EcoBytesPolicy>;

  fn clone_into<'a>(&'a self, target: &mut EcoBytes)
  where
    EcoBytes: From<&'a Self>,
  {
    target.clear();
    target.extend_from_slice(self);
  }
}

impl From<EcoString> for Cow<'_, str, EcoPolicy> {
  fn from(value: EcoString) -> Self {
    Self::owned(value)
  }
}

impl<T> From<EcoVec<T>> for Cow<'_, [T], EcoPolicy> {
  fn from(value: EcoVec<T>) -> Self {
    Self::owned(value)
  }
}

impl From<EcoBytes> for Cow<'_, [u8], EcoBytesPolicy> {
  fn from(value: EcoBytes) -> Self {
    Self::owned(value)
  }
}

impl<'a, P: ?Sized> From<&'a EcoString> for Cow<'a, str, P>
where
  str: ToOwned<P>,
{
  fn from(value: &'a EcoString) -> Self {
    Self::borrowed(value.as_str())
  }
}

impl<'a, T, P: ?Sized> From<&'a EcoVec<T>> for Cow<'a, [T], P>
where
  [T]: ToOwned<P>,
{
  fn from(value: &'a EcoVec<T>) -> Self {
    Self::borrowed(value.as_slice())
  }
}

impl<'a, P: ?Sized> From<&'a EcoBytes> for Cow<'a, [u8], P>
where
  [u8]: ToOwned<P>,
{
  fn from(value: &'a EcoBytes) -> Self {
    Self::borrowed(value.as_slice())
  }
}
