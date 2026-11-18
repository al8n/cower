//! Interoperation with alloc types through actual storage, never enum layout casts.
use crate::{Cow, CowRepr, ToOwned};
use core::borrow::Borrow;
use std::{borrow::Cow as StdCow, borrow::ToOwned as StdToOwned, boxed::Box, rc::Rc};

impl<'a, T, P: ?Sized> Cow<'a, T, P>
where
  T:
    ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned, Storage<'a> = StdCow<'a, T>>,
{
  /// Moves out the actual standard `Cow`, preserving variant and allocation.
  ///
  /// Available when this policy selects standard `Cow` as its backing storage.
  /// No enum-layout assumption is required; the stored value already has the
  /// exact returned type. Also works in const contexts on Rust 1.85.
  #[inline]
  pub const fn into_std(self) -> StdCow<'a, T> {
    self.into_storage()
  }

  /// Wraps a standard `Cow` without cloning or allocating, including in consts.
  #[inline]
  pub const fn from_std(value: StdCow<'a, T>) -> Self {
    Self::from_storage(value)
  }

  /// Borrows the actual standard `Cow` field, including in const contexts.
  ///
  /// Deref coercion also permits passing `&self` directly to functions accepting
  /// `&std::borrow::Cow`. The reference cannot outlive the borrow of `self`.
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let reference;
  /// {
  ///     let value: Cow<'_, str> = Cow::Owned(String::from("local"));
  ///     reference = value.as_std();
  /// }
  /// assert_eq!(reference.len(), 5);
  /// ```
  ///
  /// Static or custom enum backing cannot be cast into a standard enum reference:
  ///
  /// ```compile_fail
  /// use cower::{Cow, StaticPolicy};
  /// let value: Cow<'_, str, StaticPolicy> = Cow::borrowed("static");
  /// let _: &std::borrow::Cow<'_, str> = &value;
  /// ```
  #[inline]
  pub const fn as_std(&self) -> &StdCow<'a, T> {
    self.as_storage()
  }

  /// Mutably borrows the actual standard `Cow` field.
  #[inline]
  pub const fn as_std_mut(&mut self) -> &mut StdCow<'a, T> {
    self.as_storage_mut()
  }
}

impl<T: ?Sized + ToOwned<P> + StdToOwned, P: ?Sized> Cow<'_, T, P> {
  /// Creates a borrowed standard `Cow` view of any policy's contents.
  ///
  /// Unlike `as_std`, this returns a value and works with fixed-capacity or static
  /// backing. The returned borrow is tied to `self`, without allocation.
  #[inline]
  pub fn borrow_std(&self) -> StdCow<'_, T> {
    StdCow::Borrowed(self.as_ref())
  }
}

impl<'a, T, P: ?Sized> From<Cow<'a, T, P>> for StdCow<'a, T>
where
  T: ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned>,
{
  fn from(value: Cow<'a, T, P>) -> Self {
    match value.into_repr() {
      CowRepr::Borrowed(value) => Self::Borrowed(value),
      CowRepr::Owned(value) => Self::Owned(value),
    }
  }
}

impl<'a, T, P: ?Sized> From<StdCow<'a, T>> for Cow<'a, T, P>
where
  T: ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned>,
{
  fn from(value: StdCow<'a, T>) -> Self {
    match value {
      StdCow::Borrowed(value) => Self::borrowed(value),
      StdCow::Owned(value) => Self::owned(value),
    }
  }
}

/// Preserves the original borrow or clones owned contents into standard storage.
/// The result can outlive the reference to the wrapper.
impl<'a, T: ?Sized + StdToOwned + ToOwned<P>, P: ?Sized> From<&Cow<'a, T, P>> for StdCow<'a, T> {
  #[inline]
  fn from(value: &Cow<'a, T, P>) -> Self {
    match value.as_borrowed() {
      Some(borrowed) => Self::Borrowed(borrowed),
      None => Self::Owned(StdToOwned::to_owned(value.as_ref())),
    }
  }
}

impl<'a, 'b, T, P: ?Sized> From<&'b Cow<'a, T, P>> for &'b StdCow<'a, T>
where
  T:
    ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned, Storage<'a> = StdCow<'a, T>>,
{
  fn from(value: &'b Cow<'a, T, P>) -> Self {
    value.as_std()
  }
}

impl<'a, 'b, T, P: ?Sized> From<&'b mut Cow<'a, T, P>> for &'b mut StdCow<'a, T>
where
  T:
    ?Sized + StdToOwned + ToOwned<P, Owned = <T as StdToOwned>::Owned, Storage<'a> = StdCow<'a, T>>,
{
  fn from(value: &'b mut Cow<'a, T, P>) -> Self {
    value.as_std_mut()
  }
}

// Use concrete owned types here: a blanket From<T::Owned> would overlap the
// standard identity conversion and our borrowed From implementations.
macro_rules! owned_conversions {
  ($borrowed:ty, $owned:ty) => {
    impl<'a, P: ?Sized> From<&'a $owned> for Cow<'a, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      #[inline]
      fn from(value: &'a $owned) -> Self {
        Self::borrowed(value.borrow())
      }
    }

    impl<P: ?Sized> From<$owned> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P, Owned = $owned>,
    {
      #[inline]
      fn from(value: $owned) -> Self {
        Self::owned(value)
      }
    }

    impl<P: ?Sized> From<Cow<'_, $borrowed, P>> for $owned
    where
      $borrowed: ToOwned<P, Owned = $owned>,
    {
      #[inline]
      fn from(value: Cow<'_, $borrowed, P>) -> Self {
        std::borrow::Cow::from(value).into_owned()
      }
    }

    impl<P: ?Sized> From<Cow<'_, $borrowed, P>> for Box<$borrowed>
    where
      $borrowed: ToOwned<P, Owned = $owned>,
    {
      #[inline]
      fn from(value: Cow<'_, $borrowed, P>) -> Self {
        Self::from(std::borrow::Cow::from(value))
      }
    }
  };
}

#[cfg(feature = "std")]
pub(crate) use owned_conversions;

owned_conversions!(str, std::string::String);
owned_conversions!(core::ffi::CStr, std::ffi::CString);

impl<'a, T, P: ?Sized> From<&'a std::vec::Vec<T>> for Cow<'a, [T], P>
where
  [T]: ToOwned<P>,
{
  fn from(value: &'a std::vec::Vec<T>) -> Self {
    Self::borrowed(value)
  }
}

impl<T, P: ?Sized> From<std::vec::Vec<T>> for Cow<'_, [T], P>
where
  [T]: ToOwned<P, Owned = std::vec::Vec<T>>,
{
  fn from(value: std::vec::Vec<T>) -> Self {
    Self::owned(value)
  }
}

impl<T: Clone, P: ?Sized> From<Cow<'_, [T], P>> for std::vec::Vec<T>
where
  [T]: ToOwned<P, Owned = std::vec::Vec<T>>,
{
  fn from(value: Cow<'_, [T], P>) -> Self {
    std::borrow::Cow::from(value).into_owned()
  }
}

impl<T: Clone, P: ?Sized> From<Cow<'_, [T], P>> for Box<[T]>
where
  [T]: ToOwned<P, Owned = std::vec::Vec<T>>,
{
  fn from(value: Cow<'_, [T], P>) -> Self {
    Self::from(std::borrow::Cow::from(value))
  }
}

impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> From<Cow<'a, T, P>> for Rc<T>
where
  Rc<T>: From<&'a T> + From<T::Owned>,
{
  fn from(value: Cow<'a, T, P>) -> Self {
    match value.into_repr() {
      CowRepr::Borrowed(value) => Self::from(value),
      CowRepr::Owned(value) => Self::from(value),
    }
  }
}

#[cfg(target_has_atomic = "ptr")]
impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> From<Cow<'a, T, P>> for std::sync::Arc<T>
where
  std::sync::Arc<T>: From<&'a T> + From<T::Owned>,
{
  fn from(value: Cow<'a, T, P>) -> Self {
    match value.into_repr() {
      CowRepr::Borrowed(value) => Self::from(value),
      CowRepr::Owned(value) => Self::from(value),
    }
  }
}

impl<'b, P: ?Sized> From<Cow<'b, str, P>> for Box<dyn core::error::Error + Send + Sync + '_>
where
  str: ToOwned<P, Owned = std::string::String>,
{
  fn from(value: Cow<'b, str, P>) -> Self {
    Self::from(std::borrow::Cow::from(value))
  }
}

impl<'b, P: ?Sized> From<Cow<'b, str, P>> for Box<dyn core::error::Error + '_>
where
  str: ToOwned<P, Owned = std::string::String>,
{
  fn from(value: Cow<'b, str, P>) -> Self {
    Self::from(std::borrow::Cow::from(value))
  }
}
