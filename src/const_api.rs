//! Const operations that need to know the policy's concrete backing type.

#[cfg(not(any(feature = "std", feature = "alloc")))]
use crate::DefaultPolicy;
use crate::{Cow, CowRepr, StaticPolicy, ToOwned};

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ?Sized + std::borrow::ToOwned + 'static> Cow<'static, T> {
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
    Self::Borrowed(value)
  }
}

// Stable Rust cannot call CowStorage::borrowed from a const fn. Implement the
// constructor for concrete policies instead of assuming custom storage layouts.
macro_rules! repr_from_static {
  ($policy:ty $(, const $capacity:ident: $size:ty)?) => {
    impl<T: ?Sized + 'static $(, const $capacity: $size)?> Cow<'static, T, $policy>
    where
      T: ToOwned<$policy, Storage<'static> = CowRepr<'static, T, $policy>>,
    {
      /// Borrows a static value without cloning or allocating, including in consts.
      ///
      /// Specify the borrowed type and policy, for example
      /// `Cow::<str, StaticPolicy>::from_static("hello")`. The result is borrowed,
      /// even when the policy uses static references as its owned representation.
      #[inline]
      pub const fn from_static(value: &'static T) -> Self {
        Self::from_storage(CowRepr::Borrowed(value))
      }
    }
  };
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
repr_from_static!(DefaultPolicy);
repr_from_static!(StaticPolicy);
#[cfg(feature = "heapless_0_9")]
repr_from_static!(crate::Heapless<N>, const N: usize);
#[cfg(feature = "arrayvec_0_7")]
repr_from_static!(crate::ArrayVecPolicy<N>, const N: usize);
#[cfg(feature = "smallvec_1")]
repr_from_static!(crate::SmallVecPolicy<N>, const N: usize);
#[cfg(feature = "bytes_1")]
repr_from_static!(crate::BytesPolicy);
#[cfg(feature = "compact_str_0_9")]
repr_from_static!(crate::CompactStrPolicy);
#[cfg(feature = "ecow_0_3")]
repr_from_static!(crate::EcoPolicy);
#[cfg(feature = "ecow_0_3")]
repr_from_static!(crate::EcoBytesPolicy);
#[cfg(feature = "arcstr_1")]
repr_from_static!(crate::ArcStrPolicy);
#[cfg(feature = "arcstr_1")]
repr_from_static!(crate::ArcSubstrPolicy);
#[cfg(feature = "smol-bytes_0_2")]
repr_from_static!(crate::SmolBytesBufferPolicy);
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
repr_from_static!(crate::SmolBytesPolicy);
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
repr_from_static!(crate::SmolBytesCompactPolicy);
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
repr_from_static!(crate::SmolBytesMutPolicy);

#[cfg(any(feature = "std", feature = "alloc"))]
impl Cow<'_, str> {
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

macro_rules! static_string_accessors {
  ($policy:ty) => {
    impl Cow<'_, str, $policy> {
      /// Borrows the string contents without copying, including in consts.
      ///
      /// Both borrowed and owned static-reference storage support const access
      /// on Rust 1.85. The returned reference is tied to the borrow of `self`.
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
  };
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
static_string_accessors!(DefaultPolicy);
static_string_accessors!(StaticPolicy);
