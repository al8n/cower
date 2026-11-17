//! Const operations selected by concrete storage, independently of policy names.

use crate::{CowRepr, ToOwned, cow::CowWithStorage};

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, P: ?Sized> CowWithStorage<'static, T, P, std::borrow::Cow<'static, T>>
where
  T: ?Sized
    + std::borrow::ToOwned
    + ToOwned<
      P,
      Owned = <T as std::borrow::ToOwned>::Owned,
      Storage<'static> = std::borrow::Cow<'static, T>,
    > + 'static,
{
  /// Borrows a static value without cloning or allocating, including in consts.
  ///
  /// Specify the borrowed type (and policy, if needed) to select the constructor:
  ///
  /// ```
  /// use cower::Cow;
  /// const TEXT: Cow<'static, str> = Cow::<str>::from_static("hello");
  /// const BYTES: Cow<'static, [u8]> = Cow::<[u8]>::from_static(b"hello");
  /// assert!(TEXT.is_borrowed());
  /// assert_eq!(BYTES.as_ref(), b"hello");
  /// ```
  ///
  /// A local borrow cannot become static:
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let bytes = [b'h', b'i'];
  /// let _ = Cow::<[u8]>::from_static(&bytes);
  /// ```
  #[inline]
  pub const fn from_static(value: &'static T) -> Self {
    Self::from_storage(std::borrow::Cow::Borrowed(value))
  }
}

impl<T, P: ?Sized> CowWithStorage<'static, T, P, CowRepr<'static, T, P>>
where
  T: ?Sized + ToOwned<P, Storage<'static> = CowRepr<'static, T, P>> + 'static,
{
  /// Borrows a static value without cloning or allocating, including in consts.
  ///
  /// Works with every policy selecting `CowRepr`, including custom and unsized
  /// policies. No conversion to the owned representation is required.
  ///
  /// ```
  /// use cower::{Cow, CowRepr, ToOwned};
  /// struct Custom;
  /// impl ToOwned<Custom> for str {
  ///     type Owned = &'static str;
  ///     type Storage<'a> = CowRepr<'a, str, Custom>;
  /// }
  /// const VALUE: Cow<'static, str, Custom> = Cow::<str, Custom>::from_static("hello");
  /// assert!(VALUE.is_borrowed());
  /// ```
  #[inline]
  pub const fn from_static(value: &'static T) -> Self {
    Self::from_storage(CowRepr::Borrowed(value))
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, P: ?Sized> CowWithStorage<'a, str, P, std::borrow::Cow<'a, str>>
where
  str: ToOwned<P, Owned = std::string::String, Storage<'a> = std::borrow::Cow<'a, str>>,
{
  /// Borrows the string contents without cloning or allocating.
  ///
  /// Const on Rust 1.87 and later, when `String::as_str` became const. On Rust
  /// 1.85 and 1.86 this remains an ordinary method. The returned reference is
  /// tied to the borrow of `self`, including when the contents are owned.
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let text;
  /// {
  ///     let value: Cow<'static, str> = Cow::Owned(String::from("owned"));
  ///     text = value.as_str();
  /// }
  /// assert_eq!(text, "owned");
  /// ```
  #[rustversion::attr(since(1.87), const)]
  #[inline]
  pub fn as_str(&self) -> &str {
    match self.as_storage() {
      std::borrow::Cow::Borrowed(value) => value,
      std::borrow::Cow::Owned(value) => value.as_str(),
    }
  }

  /// Borrows the string's UTF-8 bytes without cloning or allocating.
  ///
  /// Like [`Self::as_str`], this is const on Rust 1.87 and later, and an
  /// ordinary method on Rust 1.85/1.86. The bytes borrow `self`, not just the
  /// lifetime of the `Cow`'s borrowed variant.
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let bytes;
  /// {
  ///     let value: Cow<'static, str> = Cow::Owned(String::from("owned"));
  ///     bytes = value.as_bytes();
  /// }
  /// assert_eq!(bytes, b"owned");
  /// ```
  #[rustversion::attr(since(1.87), const)]
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.as_str().as_bytes()
  }
}

impl<'a, P: ?Sized> CowWithStorage<'a, str, P, CowRepr<'a, str, P>>
where
  str: ToOwned<P, Owned = &'static str, Storage<'a> = CowRepr<'a, str, P>>,
{
  /// Borrows the string contents without copying, including in consts.
  ///
  /// Const on Rust 1.85 for any policy using `CowRepr` with static-reference
  /// owned storage, including core-only `DefaultPolicy` and custom policies.
  /// The returned reference is tied to the borrow of `self`.
  #[inline]
  pub const fn as_str(&self) -> &str {
    match self.as_storage() {
      CowRepr::Borrowed(value) | CowRepr::Owned(value) => value,
    }
  }

  /// Borrows the string's UTF-8 bytes without copying, including in consts.
  ///
  /// Const on Rust 1.85 for both borrowed and owned static-reference storage.
  /// The returned slice is tied to the borrow of `self`.
  #[inline]
  pub const fn as_bytes(&self) -> &[u8] {
    self.as_str().as_bytes()
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T: Clone, P: ?Sized> CowWithStorage<'a, [T], P, std::borrow::Cow<'a, [T]>>
where
  [T]: ToOwned<P, Owned = std::vec::Vec<T>, Storage<'a> = std::borrow::Cow<'a, [T]>>,
{
  /// Borrows the elements without cloning or allocating.
  ///
  /// Const on Rust 1.87 and later, matching `Vec::as_slice`. On Rust 1.85/1.86
  /// this remains an ordinary method. The result borrows `self`.
  ///
  /// ```compile_fail
  /// use cower::Cow;
  /// let slice;
  /// {
  ///     let value: Cow<'static, [u8]> = Cow::Owned(vec![1, 2]);
  ///     slice = value.as_slice();
  /// }
  /// assert_eq!(slice, &[1, 2]);
  /// ```
  #[rustversion::attr(since(1.87), const)]
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    match self.as_storage() {
      std::borrow::Cow::Borrowed(value) => value,
      std::borrow::Cow::Owned(value) => value.as_slice(),
    }
  }
}

impl<'a, T: 'static, P: ?Sized> CowWithStorage<'a, [T], P, CowRepr<'a, [T], P>>
where
  [T]: ToOwned<P, Owned = &'static [T], Storage<'a> = CowRepr<'a, [T], P>>,
{
  /// Borrows the elements without copying, including in consts on Rust 1.85.
  ///
  /// Works with any static-reference policy and does not require `T: Clone`.
  /// The result is tied to the borrow of `self`.
  #[inline]
  pub const fn as_slice(&self) -> &[T] {
    match self.as_storage() {
      CowRepr::Borrowed(value) | CowRepr::Owned(value) => value,
    }
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, P: ?Sized> CowWithStorage<'a, [u8], P, std::borrow::Cow<'a, [u8]>>
where
  [u8]: ToOwned<P, Owned = std::vec::Vec<u8>, Storage<'a> = std::borrow::Cow<'a, [u8]>>,
{
  /// Borrows the bytes, including in consts on Rust 1.87 and later.
  ///
  /// Equivalent to [`Self::as_slice`], without copying. On Rust 1.85/1.86 this
  /// remains an ordinary method. The result borrows `self`.
  #[rustversion::attr(since(1.87), const)]
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}

impl<'a, P: ?Sized> CowWithStorage<'a, [u8], P, CowRepr<'a, [u8], P>>
where
  [u8]: ToOwned<P, Owned = &'static [u8], Storage<'a> = CowRepr<'a, [u8], P>>,
{
  /// Borrows the bytes without copying, including in consts on Rust 1.85.
  ///
  /// Equivalent to [`Self::as_slice`]; the result borrows `self`.
  #[inline]
  pub const fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}
