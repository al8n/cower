use core::borrow::Borrow;

/// The default storage policy, adapting to the crate's enabled features.
///
/// With `std` or `alloc`, every type implementing the standard `ToOwned` uses its
/// standard owned representation. With neither feature, only `str` and `[T]`
/// (where `T: 'static`) have built-in implementations, using `&'static str` and
/// `&'static [T]` respectively. There is no blanket implementation in that mode.
///
/// Use [`StaticPolicy`] for static-reference storage for any `'static` type,
/// regardless of the enabled features.
///
/// This is an uninhabited type-level marker: no policy value is constructed or
/// stored in [`crate::Cow`], and the policy does not implement `Default`.
///
/// ```compile_fail
/// let policy = cower::DefaultPolicy;
/// ```
///
/// ```compile_fail
/// let policy: cower::DefaultPolicy = Default::default();
/// ```
#[cfg_attr(
  not(any(feature = "std", feature = "alloc")),
  doc = r#"
Sized types do not implicitly acquire a default storage representation:

```compile_fail
use cower::Cow;
let value: Cow<'_, u32> = Cow::borrowed(&42);
```

A local borrow can be read, but cannot be promoted to static storage:

```compile_fail
use cower::Cow;
let bytes = [b'h', b'i'];
let text = core::str::from_utf8(&bytes).unwrap();
let value: Cow<'_, str> = Cow::borrowed(text);
let _: &'static str = value.into_owned();
```

The same restriction applies to slices:

```compile_fail
use cower::Cow;
let bytes = [1, 2, 3];
let value: Cow<'_, [u8]> = Cow::borrowed(&bytes);
let _: &'static [u8] = value.into_owned();
```
"#
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefaultPolicy {}

/// Always selects `&'static T` as the owned representation, with or without alloc.
///
/// This policy has a blanket implementation for every `T: ?Sized + 'static`.
/// The type must be `'static`, but the `Borrowed` variant can contain a shorter
/// borrow. Only an already-static borrow can be converted into owned storage:
///
/// ```
/// use cower::{Cow, StaticPolicy};
///
/// static ANSWER: u32 = 42;
/// let value: Cow<'_, u32, StaticPolicy> = Cow::borrowed(&ANSWER);
/// let owned: &'static u32 = value.into_owned();
/// assert!(core::ptr::eq(owned, &ANSWER));
/// ```
///
/// ```compile_fail
/// use cower::{Cow, StaticPolicy};
/// let local = 42;
/// let value: Cow<'_, u32, StaticPolicy> = Cow::borrowed(&local);
/// let _: &'static u32 = value.into_owned();
/// ```
///
/// This is an uninhabited type-level marker with no constructors or `Default`
/// implementation:
///
/// ```compile_fail
/// let policy = cower::StaticPolicy;
/// ```
///
/// ```compile_fail
/// let policy: cower::StaticPolicy = Default::default();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StaticPolicy {}

/// Selects the owned storage representation of a borrowed type for policy `P`.
///
/// The associated type selects storage. [`to_owned`](Self::to_owned) and
/// [`clone_into`](Self::clone_into) require `Owned: From<&'a Self>`, preserving
/// the lifetime of the source borrow. [`crate::CowRepr`] uses these methods for
/// infallible conversion; standard backing uses standard `ToOwned` directly.
/// Fallible conversions use `TryFrom<&'a Self>`.
///
/// For example, `str` may map to `&'static str`, but an arbitrary `&str` cannot
/// be converted into a static reference. Similarly, a fixed-capacity string may
/// reject input that exceeds its capacity. No panicking fallback is supplied.
///
/// A downstream crate can implement this trait for a foreign type such as `str`
/// by supplying its own local policy type:
///
/// ```
/// # use heapless_0_9 as heapless;
/// use cower::{Cow, ToOwned};
///
/// struct Inline<const N: usize>;
///
/// impl<const N: usize> ToOwned<Inline<N>> for str {
///     type Owned = heapless::String<N>;
///     type Storage<'a> = cower::CowRepr<'a, Self, Inline<N>>;
/// }
///
/// let mut text: Cow<'_, str, Inline<16>> = Cow::borrowed("hello");
/// text.try_to_mut().unwrap().push_str("!").unwrap();
/// assert_eq!(text, "hello!");
/// ```
pub trait ToOwned<P: ?Sized = DefaultPolicy> {
  /// A storage representation that borrows as `Self`.
  type Owned: Borrow<Self>;

  /// The concrete backing type used by [`crate::Cow`].
  ///
  /// Custom policies normally select [`crate::CowRepr`]. The default heap policy
  /// selects the standard `Cow` itself, enabling sound reference coercions.
  type Storage<'a>: crate::CowStorage<'a, Self, P>
  where
    Self: 'a;

  /// Creates owned storage from this borrow.
  ///
  /// The default implementation uses `From`. With `std` or `alloc`,
  /// [`DefaultPolicy`] forwards to the standard `ToOwned::to_owned` method.
  /// The conversion bound applies in every implementation: static-reference
  /// storage requires an already-static borrow, and fallible storage should use
  /// [`crate::Cow::try_into_owned`] instead.
  ///
  /// ```
  /// use cower::ToOwned;
  ///
  /// let text = <str as ToOwned>::to_owned("hello");
  /// assert_eq!(text, "hello");
  /// ```
  ///
  /// ```compile_fail
  /// use cower::{StaticPolicy, ToOwned};
  ///
  /// let local = 42;
  /// let _: &'static u32 = <u32 as ToOwned<StaticPolicy>>::to_owned(&local);
  /// ```
  #[inline]
  fn to_owned<'a>(&'a self) -> Self::Owned
  where
    Self::Owned: From<&'a Self>,
  {
    Self::Owned::from(self)
  }

  /// Replaces owned storage with a copy of this borrowed value.
  ///
  /// The default implementation assigns the result of [`Self::to_owned`].
  /// Implementations may override this to reuse the target's resources. With
  /// `std` or `alloc`, [`DefaultPolicy`] forwards to the standard
  /// `ToOwned::clone_into`, including its storage-reuse behavior.
  ///
  /// The source borrow has the same conversion requirements as
  /// [`Self::to_owned`]; the mutable borrow of `target` can be shorter.
  ///
  /// ```
  /// use cower::ToOwned;
  ///
  /// let mut text = <str as ToOwned>::to_owned("old");
  /// <str as ToOwned>::clone_into("new", &mut text);
  /// assert_eq!(text, "new");
  /// ```
  ///
  /// ```compile_fail
  /// use cower::{StaticPolicy, ToOwned};
  ///
  /// let local = 42;
  /// let mut target: &'static u32 = &0;
  /// <u32 as ToOwned<StaticPolicy>>::clone_into(&local, &mut target);
  /// ```
  #[inline]
  fn clone_into<'a>(&'a self, target: &mut Self::Owned)
  where
    Self::Owned: From<&'a Self>,
  {
    *target = <Self as ToOwned<P>>::to_owned(self);
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ?Sized + std::borrow::ToOwned> ToOwned<DefaultPolicy> for T {
  type Owned = <T as std::borrow::ToOwned>::Owned;
  type Storage<'a>
    = std::borrow::Cow<'a, T>
  where
    T: 'a;

  #[inline]
  fn to_owned<'a>(&'a self) -> Self::Owned
  where
    Self::Owned: From<&'a Self>,
  {
    std::borrow::ToOwned::to_owned(self)
  }

  #[inline]
  fn clone_into<'a>(&'a self, target: &mut Self::Owned)
  where
    Self::Owned: From<&'a Self>,
  {
    std::borrow::ToOwned::clone_into(self, target);
  }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl ToOwned<DefaultPolicy> for str {
  type Owned = &'static str;
  type Storage<'a> = crate::CowRepr<'a, Self>;
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<T: 'static> ToOwned<DefaultPolicy> for [T] {
  type Owned = &'static [T];
  type Storage<'a> = crate::CowRepr<'a, Self>;
}

impl<T: ?Sized + 'static> ToOwned<StaticPolicy> for T {
  type Owned = &'static T;
  type Storage<'a> = crate::CowRepr<'a, Self, StaticPolicy>;
}
