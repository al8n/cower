use core::{
  borrow::Borrow,
  cmp::Ordering,
  fmt,
  hash::{Hash, Hasher},
  ops::Deref,
};

use crate::{DefaultPolicy, ToOwned};

/// A borrowed/owned enum for explicit pattern matching and custom policy storage.
///
/// [`crate::Cow`] is a transparent wrapper. Call its `into_repr` method to match
/// these variants, or select this enum as a custom policy's `Storage` type.
pub enum CowRepr<'a, T: ?Sized + ToOwned<P>, P: ?Sized = DefaultPolicy> {
  /// A borrowed value; it need not be `'static`.
  Borrowed(&'a T),
  /// The representation selected by `T`'s implementation of [`ToOwned<P>`].
  Owned(<T as ToOwned<P>>::Owned),
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> CowRepr<'a, T, P> {
  /// Returns whether this value is borrowed.
  #[inline]
  pub const fn is_borrowed(&self) -> bool {
    matches!(self, Self::Borrowed(_))
  }

  /// Returns whether this value uses its policy's owned representation.
  #[inline]
  pub const fn is_owned(&self) -> bool {
    matches!(self, Self::Owned(_))
  }

  /// Converts a borrow into owned storage, or moves out existing owned storage.
  ///
  /// The conversion must be infallible and valid for the original borrow's
  /// lifetime. With static-reference storage, the borrow must already be
  /// `'static`. Use [`Self::try_into_owned`] for a representation with a fallible
  /// conversion, such as a fixed-capacity buffer.
  #[inline]
  pub fn into_owned(self) -> T::Owned
  where
    T::Owned: From<&'a T>,
  {
    match self {
      Self::Borrowed(value) => <T as ToOwned<P>>::to_owned(value),
      Self::Owned(value) => value,
    }
  }

  /// Tries to convert a borrow into owned storage, or moves out existing storage.
  ///
  /// Returns the representation's conversion error if the borrowed value cannot
  /// be copied. An existing `Owned` value is returned without conversion.
  #[inline]
  pub fn try_into_owned(self) -> Result<T::Owned, <T::Owned as TryFrom<&'a T>>::Error>
  where
    T::Owned: TryFrom<&'a T>,
  {
    match self {
      Self::Borrowed(value) => T::Owned::try_from(value),
      Self::Owned(value) => Ok(value),
    }
  }

  /// Ensures this value is owned and returns mutable access to its storage.
  ///
  /// Converts only once. With static-reference storage the result is a mutable
  /// reference to the **reference**, so it can be replaced but does not allow
  /// mutation of the static data. Use [`Self::try_to_mut`] for fallible storage.
  #[inline]
  pub fn to_mut(&mut self) -> &mut T::Owned
  where
    T::Owned: From<&'a T>,
  {
    if let Self::Borrowed(value) = self {
      *self = Self::Owned(<T as ToOwned<P>>::to_owned(*value));
    }
    match self {
      Self::Owned(value) => value,
      Self::Borrowed(_) => unreachable!(),
    }
  }

  /// Tries to ensure this value is owned and returns mutable access to its storage.
  ///
  /// On conversion failure, the original borrowed value remains unchanged.
  /// Existing owned storage is returned without conversion.
  #[inline]
  pub fn try_to_mut(&mut self) -> Result<&mut T::Owned, <T::Owned as TryFrom<&'a T>>::Error>
  where
    T::Owned: TryFrom<&'a T>,
  {
    if let Self::Borrowed(value) = self {
      *self = Self::Owned(T::Owned::try_from(*value)?);
    }
    match self {
      Self::Owned(value) => Ok(value),
      Self::Borrowed(_) => unreachable!(),
    }
  }
}

impl<T: ?Sized + ToOwned<P>, P: ?Sized> Deref for CowRepr<'_, T, P> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &T {
    match self {
      Self::Borrowed(value) => value,
      Self::Owned(value) => value.borrow(),
    }
  }
}

impl<T: ?Sized + ToOwned<P>, P: ?Sized> AsRef<T> for CowRepr<'_, T, P> {
  #[inline]
  fn as_ref(&self) -> &T {
    self
  }
}

impl<T: ?Sized + ToOwned<P>, P: ?Sized> Borrow<T> for CowRepr<'_, T, P> {
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> From<&'a T> for CowRepr<'a, T, P> {
  #[inline]
  fn from(value: &'a T) -> Self {
    Self::Borrowed(value)
  }
}

impl<T: ?Sized + ToOwned<P>, P: ?Sized> Clone for CowRepr<'_, T, P>
where
  T::Owned: Clone,
{
  #[inline]
  fn clone(&self) -> Self {
    match self {
      Self::Borrowed(value) => Self::Borrowed(value),
      Self::Owned(value) => Self::Owned(value.clone()),
    }
  }

  #[inline]
  fn clone_from(&mut self, source: &Self) {
    match (self, source) {
      (Self::Owned(target), Self::Owned(source)) => target.clone_from(source),
      (target, source) => *target = source.clone(),
    }
  }
}

impl<T: ?Sized + ToOwned<P>, P: ?Sized> Copy for CowRepr<'_, T, P> where T::Owned: Copy {}

impl<T: ?Sized + ToOwned<P> + fmt::Debug, P: ?Sized> fmt::Debug for CowRepr<'_, T, P> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&**self, f)
  }
}

impl<T: ?Sized + ToOwned<P> + fmt::Display, P: ?Sized> fmt::Display for CowRepr<'_, T, P> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&**self, f)
  }
}

impl<T, U, P, Q> PartialEq<CowRepr<'_, U, Q>> for CowRepr<'_, T, P>
where
  T: ?Sized + ToOwned<P> + PartialEq<U>,
  U: ?Sized + ToOwned<Q>,
  P: ?Sized,
  Q: ?Sized,
{
  #[inline]
  fn eq(&self, other: &CowRepr<'_, U, Q>) -> bool {
    (**self).eq(&**other)
  }
}

impl<T: ?Sized + ToOwned<P> + Eq, P: ?Sized> Eq for CowRepr<'_, T, P> {}

impl<T, U, P, Q> PartialOrd<CowRepr<'_, U, Q>> for CowRepr<'_, T, P>
where
  T: ?Sized + ToOwned<P> + PartialOrd<U>,
  U: ?Sized + ToOwned<Q>,
  P: ?Sized,
  Q: ?Sized,
{
  #[inline]
  fn partial_cmp(&self, other: &CowRepr<'_, U, Q>) -> Option<Ordering> {
    (**self).partial_cmp(&**other)
  }
}

impl<T: ?Sized + ToOwned<P> + Ord, P: ?Sized> Ord for CowRepr<'_, T, P> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    (**self).cmp(&**other)
  }
}

impl<T: ?Sized + ToOwned<P> + Hash, P: ?Sized> Hash for CowRepr<'_, T, P> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    (**self).hash(state);
  }
}

/// Backing storage for a policy-based [`crate::Cow`].
///
/// Implementations must preserve the contents and variant of values passed to
/// their constructors. Prefer the supplied implementations for [`CowRepr`] and
/// the standard `Cow`; custom policies usually only select one of those types.
pub trait CowStorage<'a, T: ?Sized + ToOwned<P> + 'a, P: ?Sized>:
  Deref<Target = T> + Sized
{
  /// Stores a borrow without changing its lifetime.
  fn borrowed(value: &'a T) -> Self;
  /// Stores an owned value.
  fn owned(value: T::Owned) -> Self;
  /// Moves the value into an enum that callers can pattern-match.
  fn into_repr(self) -> CowRepr<'a, T, P>;
  /// Returns the original borrow, or `None` for owned storage.
  fn as_borrowed(&self) -> Option<&'a T>;
  /// Returns the owned field, or `None` for borrowed storage.
  fn as_owned(&self) -> Option<&T::Owned>;
  /// Returns mutable access to the owned field, or `None` for borrowed storage.
  fn as_owned_mut(&mut self) -> Option<&mut T::Owned>;
}

/// Storage supporting infallible conversion from its original borrow.
///
/// Standard storage uses standard `ToOwned` without an additional `From` bound.
/// [`CowRepr`] uses the policy's conversion with its lifetime-aware `From` bound.
pub trait OwnedStorage<'a, T: ?Sized + ToOwned<P> + 'a, P: ?Sized>: CowStorage<'a, T, P> {
  /// Moves out owned storage or converts the borrowed value.
  fn into_owned(self) -> T::Owned;
  /// Converts once if borrowed and returns mutable access to the owned field.
  fn to_mut(&mut self) -> &mut T::Owned;
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> CowStorage<'a, T, P> for CowRepr<'a, T, P> {
  fn borrowed(value: &'a T) -> Self {
    Self::Borrowed(value)
  }
  fn owned(value: T::Owned) -> Self {
    Self::Owned(value)
  }
  fn into_repr(self) -> Self {
    self
  }
  fn as_borrowed(&self) -> Option<&'a T> {
    match self {
      Self::Borrowed(value) => Some(value),
      Self::Owned(_) => None,
    }
  }
  fn as_owned(&self) -> Option<&T::Owned> {
    match self {
      Self::Owned(value) => Some(value),
      Self::Borrowed(_) => None,
    }
  }
  fn as_owned_mut(&mut self) -> Option<&mut T::Owned> {
    match self {
      Self::Owned(value) => Some(value),
      Self::Borrowed(_) => None,
    }
  }
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> OwnedStorage<'a, T, P> for CowRepr<'a, T, P>
where
  T::Owned: From<&'a T>,
{
  fn into_owned(self) -> T::Owned {
    CowRepr::into_owned(self)
  }
  fn to_mut(&mut self) -> &mut T::Owned {
    CowRepr::to_mut(self)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod heap {
  use super::*;
  use std::borrow::{Cow as StdCow, ToOwned as StdToOwned};

  impl<'a, T, P: ?Sized> CowStorage<'a, T, P> for StdCow<'a, T>
  where
    T: ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned>,
  {
    fn borrowed(value: &'a T) -> Self {
      Self::Borrowed(value)
    }
    fn owned(value: <T as ToOwned<P>>::Owned) -> Self {
      Self::Owned(value)
    }
    fn into_repr(self) -> CowRepr<'a, T, P> {
      match self {
        Self::Borrowed(value) => CowRepr::Borrowed(value),
        Self::Owned(value) => CowRepr::Owned(value),
      }
    }
    fn as_borrowed(&self) -> Option<&'a T> {
      match self {
        Self::Borrowed(value) => Some(value),
        Self::Owned(_) => None,
      }
    }
    fn as_owned(&self) -> Option<&<T as ToOwned<P>>::Owned> {
      match self {
        Self::Owned(value) => Some(value),
        Self::Borrowed(_) => None,
      }
    }
    fn as_owned_mut(&mut self) -> Option<&mut <T as ToOwned<P>>::Owned> {
      match self {
        Self::Owned(value) => Some(value),
        Self::Borrowed(_) => None,
      }
    }
  }

  impl<'a, T, P: ?Sized> OwnedStorage<'a, T, P> for StdCow<'a, T>
  where
    T: ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned>,
  {
    fn into_owned(self) -> <T as ToOwned<P>>::Owned {
      StdCow::into_owned(self)
    }
    fn to_mut(&mut self) -> &mut <T as ToOwned<P>>::Owned {
      StdCow::to_mut(self)
    }
  }
}
