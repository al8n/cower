use crate::{CowRepr, CowStorage, DefaultPolicy, OwnedStorage, ToOwned};
use core::{
  borrow::Borrow,
  cmp::Ordering,
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
};

/// A transparent wrapper around the storage selected by a borrowed type's policy.
///
/// With `std` or `alloc`, [`DefaultPolicy`] stores a standard `Cow` directly.
/// Deref coercion therefore accepts `&cower::Cow` where `&std::borrow::Cow` is
/// required. Custom policies normally store [`CowRepr`] instead.
///
/// This names a struct, not an enum. Use [`Cow::borrowed`] / [`Cow::owned`] to
/// construct any policy, and [`Cow::into_repr`] to match the two variants.
/// The default policy also supplies const `Borrowed` and `Owned` constructors.
/// Const accessors are selected by the actual storage, including for custom policies.
pub type Cow<'a, T, P = DefaultPolicy> = CowWithStorage<'a, T, P, <T as ToOwned<P>>::Storage<'a>>;

/// The storage-explicit representation of [`Cow`].
///
/// Prefer [`Cow<'a, T, P>`](Cow), which selects `S` from [`ToOwned::Storage`].
/// Keeping storage explicit lets inherent const implementations distinguish
/// [`CowRepr`] from standard `Cow` without overlapping associated-type bounds.
/// This type is documented so methods shared with the `Cow` alias have public
/// documentation and working links.
#[repr(transparent)]
#[cfg_attr(
  feature = "zerocopy_0_8",
  derive(zerocopy_0_8::KnownLayout, zerocopy_0_8::Immutable)
)]
#[cfg_attr(feature = "zerocopy_0_8", zerocopy(crate = "zerocopy_0_8"))]
pub struct CowWithStorage<'a, T: ?Sized + ToOwned<P>, P: ?Sized, S> {
  inner: S,
  // This zero-sized, alignment-one marker does not impose Send/Sync bounds on
  // T or P beyond those of the actual storage, and adds no validity restrictions.
  marker: PhantomData<fn() -> (&'a T, *const P)>,
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> Cow<'a, T, P> {
  /// Wraps the policy's actual storage, including in const contexts.
  #[inline]
  pub const fn from_storage(inner: T::Storage<'a>) -> Self {
    Self {
      inner,
      marker: PhantomData,
    }
  }

  /// Moves out the actual storage without cloning or allocating.
  #[inline]
  pub const fn into_storage(self) -> T::Storage<'a> {
    // Rust 1.85 rejects moving generic drop types out of structs in const fns.
    let source = ManuallyDrop::new(self);
    // SAFETY: ManuallyDrop<Self> has Self's layout and validity. The pointer is
    // aligned and refers to a live initialized wrapper. Read its storage field
    // exactly once, then never inspect or drop the source. No enum is cast to a
    // different type, and the returned storage retains its original type.
    unsafe {
      let source = &*(&source as *const ManuallyDrop<Self> as *const Self);
      core::ptr::read(&source.inner)
    }
  }

  /// Borrows the actual backing storage.
  #[inline]
  pub const fn as_storage(&self) -> &T::Storage<'a> {
    &self.inner
  }

  /// Mutably borrows the actual backing storage.
  #[inline]
  pub const fn as_storage_mut(&mut self) -> &mut T::Storage<'a> {
    &mut self.inner
  }

  /// Creates a borrowed value for any policy.
  #[inline]
  pub fn borrowed(value: &'a T) -> Self {
    Self::from_storage(T::Storage::borrowed(value))
  }

  /// Creates an owned value for any policy.
  #[inline]
  pub fn owned(value: T::Owned) -> Self {
    Self::from_storage(T::Storage::owned(value))
  }

  /// Returns whether this value is borrowed.
  #[inline]
  pub fn is_borrowed(&self) -> bool {
    self.inner.as_borrowed().is_some()
  }

  /// Returns whether this value is owned.
  #[inline]
  pub fn is_owned(&self) -> bool {
    !self.is_borrowed()
  }

  /// Moves the value into [`CowRepr`] for explicit variant matching.
  #[inline]
  pub fn into_repr(self) -> CowRepr<'a, T, P> {
    self.inner.into_repr()
  }

  /// Borrows the owned field if this value is owned.
  #[inline]
  pub fn as_owned(&self) -> Option<&T::Owned> {
    self.inner.as_owned()
  }

  /// Returns the original borrow if this value is borrowed.
  #[inline]
  pub fn as_borrowed(&self) -> Option<&'a T> {
    self.inner.as_borrowed()
  }

  /// Converts a borrow to owned storage or moves out existing owned storage.
  ///
  /// Standard backing uses standard `ToOwned`; custom backing uses its own
  /// conversion capability. Static storage requires an already-static borrow.
  #[inline]
  pub fn into_owned(self) -> T::Owned
  where
    T::Storage<'a>: OwnedStorage<'a, T, P>,
  {
    self.inner.into_owned()
  }

  /// Converts once if borrowed and returns mutable access to owned storage.
  #[inline]
  pub fn to_mut(&mut self) -> &mut T::Owned
  where
    T::Storage<'a>: OwnedStorage<'a, T, P>,
  {
    self.inner.to_mut()
  }

  /// Tries to convert a borrow into owned storage, or moves out existing storage.
  #[inline]
  pub fn try_into_owned(self) -> Result<T::Owned, <T::Owned as TryFrom<&'a T>>::Error>
  where
    T::Owned: TryFrom<&'a T>,
  {
    match self.into_repr() {
      CowRepr::Borrowed(value) => T::Owned::try_from(value),
      CowRepr::Owned(value) => Ok(value),
    }
  }

  /// Tries to convert once, leaving the original borrow unchanged on failure.
  #[inline]
  pub fn try_to_mut(&mut self) -> Result<&mut T::Owned, <T::Owned as TryFrom<&'a T>>::Error>
  where
    T::Owned: TryFrom<&'a T>,
  {
    if let Some(value) = self.inner.as_borrowed() {
      self.inner = T::Storage::owned(T::Owned::try_from(value)?);
    }
    Ok(
      self
        .inner
        .as_owned_mut()
        .expect("CowStorage must preserve owned variants"),
    )
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T: ?Sized + std::borrow::ToOwned> Cow<'a, T> {
  /// Const constructor matching the standard `Cow::Borrowed` spelling.
  #[allow(non_snake_case)]
  #[inline]
  pub const fn Borrowed(value: &'a T) -> Self {
    Self::from_storage(std::borrow::Cow::Borrowed(value))
  }
  /// Const constructor matching the standard `Cow::Owned` spelling.
  #[allow(non_snake_case)]
  #[inline]
  pub const fn Owned(value: <T as std::borrow::ToOwned>::Owned) -> Self {
    Self::from_storage(std::borrow::Cow::Owned(value))
  }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'a, T: ?Sized + ToOwned<Storage<'a> = CowRepr<'a, T>>> Cow<'a, T> {
  /// Const constructor for the default policy's borrowed representation.
  #[allow(non_snake_case)]
  #[inline]
  pub const fn Borrowed(value: &'a T) -> Self {
    Self::from_storage(CowRepr::Borrowed(value))
  }
  /// Const constructor for the default policy's owned representation.
  #[allow(non_snake_case)]
  #[inline]
  pub const fn Owned(value: T::Owned) -> Self {
    Self::from_storage(CowRepr::Owned(value))
  }
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> Deref for Cow<'a, T, P> {
  type Target = T::Storage<'a>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
impl<T: ?Sized + ToOwned<P>, P: ?Sized> DerefMut for Cow<'_, T, P> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
impl<T: ?Sized + ToOwned<P>, P: ?Sized> AsRef<T> for Cow<'_, T, P> {
  #[inline]
  fn as_ref(&self) -> &T {
    &self.inner
  }
}
impl<T: ?Sized + ToOwned<P>, P: ?Sized> Borrow<T> for Cow<'_, T, P> {
  #[inline]
  fn borrow(&self) -> &T {
    self.as_ref()
  }
}
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> From<&'a T> for Cow<'a, T, P> {
  #[inline]
  fn from(value: &'a T) -> Self {
    Self::borrowed(value)
  }
}
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> Clone for Cow<'a, T, P>
where
  T::Storage<'a>: Clone,
{
  fn clone(&self) -> Self {
    Self::from_storage(self.inner.clone())
  }
  fn clone_from(&mut self, source: &Self) {
    self.inner.clone_from(&source.inner);
  }
}
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> Copy for Cow<'a, T, P> where T::Storage<'a>: Copy {}
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> fmt::Debug for Cow<'a, T, P>
where
  T::Storage<'a>: fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.inner, f)
  }
}
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> fmt::Display for Cow<'a, T, P>
where
  T::Storage<'a>: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.inner, f)
  }
}
impl<T, U, P, Q> PartialEq<Cow<'_, U, Q>> for Cow<'_, T, P>
where
  T: ?Sized + ToOwned<P> + PartialEq<U>,
  U: ?Sized + ToOwned<Q>,
  P: ?Sized,
  Q: ?Sized,
{
  fn eq(&self, other: &Cow<'_, U, Q>) -> bool {
    self.as_ref().eq(other.as_ref())
  }
}
impl<T: ?Sized + ToOwned<P> + Eq, P: ?Sized> Eq for Cow<'_, T, P> {}
impl<T, U, P, Q> PartialOrd<Cow<'_, U, Q>> for Cow<'_, T, P>
where
  T: ?Sized + ToOwned<P> + PartialOrd<U>,
  U: ?Sized + ToOwned<Q>,
  P: ?Sized,
  Q: ?Sized,
{
  fn partial_cmp(&self, other: &Cow<'_, U, Q>) -> Option<Ordering> {
    self.as_ref().partial_cmp(other.as_ref())
  }
}
impl<T: ?Sized + ToOwned<P> + Ord, P: ?Sized> Ord for Cow<'_, T, P> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.as_ref().cmp(other.as_ref())
  }
}
impl<T: ?Sized + ToOwned<P> + Hash, P: ?Sized> Hash for Cow<'_, T, P> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state);
  }
}
