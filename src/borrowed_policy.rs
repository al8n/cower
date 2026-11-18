use crate::{Cow, CowRepr, CowStorage, ToOwned};
use core::{borrow::Borrow, convert::Infallible, marker::PhantomData};

/// Stores the original shared reference in every feature configuration.
///
/// `Cow<'a, T, BorrowedPolicy>` has the layout of `&'a T` and is always
/// `Copy`, including when `T` contains local references. Use
/// [`Cow::into_storage`] to recover the reference with its original lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BorrowedPolicy {}

/// The uninhabited owned type associated with [`BorrowedPolicy`].
pub struct NeverOwned<T: ?Sized> {
  never: Infallible,
  marker: PhantomData<fn() -> *const T>,
}

impl<T: ?Sized> Borrow<T> for NeverOwned<T> {
  fn borrow(&self) -> &T {
    match self.never {}
  }
}

impl<T: ?Sized> ToOwned<BorrowedPolicy> for T {
  type Owned = NeverOwned<T>;
  type Storage<'a>
    = &'a T
  where
    T: 'a;
}

impl<'a, T: ?Sized> CowStorage<'a, T, BorrowedPolicy> for &'a T {
  fn borrowed(value: &'a T) -> Self {
    value
  }

  fn owned(value: NeverOwned<T>) -> Self {
    match value.never {}
  }

  fn into_repr(self) -> CowRepr<'a, T, BorrowedPolicy> {
    CowRepr::Borrowed(self)
  }

  fn as_borrowed(&self) -> Option<&'a T> {
    Some(*self)
  }

  fn as_owned(&self) -> Option<&NeverOwned<T>> {
    None
  }

  fn as_owned_mut(&mut self) -> Option<&mut NeverOwned<T>> {
    None
  }
}

impl<'a> Cow<'a, str, BorrowedPolicy> {
  /// Returns the original string with its source lifetime.
  #[inline]
  pub const fn as_str(&self) -> &'a str {
    self.as_storage()
  }

  /// Returns the original string's UTF-8 bytes with its source lifetime.
  #[inline]
  pub const fn as_bytes(&self) -> &'a [u8] {
    self.as_str().as_bytes()
  }
}

impl<'a, T> Cow<'a, [T], BorrowedPolicy> {
  /// Returns the original slice with its source lifetime.
  #[inline]
  pub const fn as_slice(&self) -> &'a [T] {
    self.as_storage()
  }
}

impl<'a> Cow<'a, [u8], BorrowedPolicy> {
  /// Returns the original byte slice with its source lifetime.
  #[inline]
  pub const fn as_bytes(&self) -> &'a [u8] {
    self.as_slice()
  }
}

impl<'a, T> From<Cow<'a, [T], BorrowedPolicy>> for &'a [T] {
  fn from(value: Cow<'a, [T], BorrowedPolicy>) -> Self {
    value.into_storage()
  }
}

impl<'a, T> From<&Cow<'a, [T], BorrowedPolicy>> for &'a [T] {
  fn from(value: &Cow<'a, [T], BorrowedPolicy>) -> Self {
    value.as_slice()
  }
}

impl<'a> From<Cow<'a, str, BorrowedPolicy>> for &'a str {
  fn from(value: Cow<'a, str, BorrowedPolicy>) -> Self {
    value.into_storage()
  }
}

impl<'a> From<&Cow<'a, str, BorrowedPolicy>> for &'a str {
  fn from(value: &Cow<'a, str, BorrowedPolicy>) -> Self {
    value.as_str()
  }
}
