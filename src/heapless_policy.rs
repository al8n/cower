use crate::{Cow, CowRepr, ToOwned};

/// Fixed-capacity inline storage backed by `heapless`.
///
/// Maps `str` to [`heapless::String<N>`] and `[T]` to [`heapless::Vec<T, N>`],
/// using [`CowRepr`] as backing storage. `N` counts UTF-8 bytes for strings and
/// elements for slices. This policy works without `std`, `alloc`, or a global
/// allocator, and does not change [`crate::DefaultPolicy`].
///
/// Borrows may exceed `N`; capacity is checked when copying into owned storage.
/// Use [`Cow::try_to_mut`] or [`Cow::try_into_owned`], which return the original
/// `heapless` capacity error. A failed `try_to_mut` leaves the borrow unchanged.
/// Slice conversion requires `T: Clone`; borrowing and storing an existing
/// owned vector do not. Element cloning retains the element type's behavior.
///
/// ```
/// use cower::{Cow, Heapless};
///
/// let mut text: Cow<'_, str, Heapless<16>> = Cow::borrowed("hello");
/// text.try_to_mut().unwrap().push('!').unwrap();
/// assert_eq!(text, "hello!");
///
/// let values = [1, 2, 3];
/// let mut bytes: Cow<'_, [u8], Heapless<4>> = Cow::from(&values);
/// bytes.try_to_mut().unwrap().push(4).unwrap();
/// assert_eq!(bytes.as_ref(), &[1, 2, 3, 4]);
/// ```
///
/// Infallible promotion is unavailable because a borrow might exceed capacity:
///
/// ```compile_fail
/// use cower::{Cow, Heapless};
/// let mut text: Cow<'_, str, Heapless<4>> = Cow::borrowed("hello");
/// text.to_mut();
/// ```
///
/// This is an uninhabited type-level marker with no constructors or `Default`
/// implementation:
///
/// ```compile_fail
/// let policy = cower::Heapless::<16>;
/// ```
///
/// ```compile_fail
/// let policy: cower::Heapless<16> = Default::default();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Heapless<const N: usize> {}

impl<const N: usize> ToOwned<Heapless<N>> for str {
  type Owned = heapless::String<N>;
  type Storage<'a> = CowRepr<'a, Self, Heapless<N>>;
}

impl<T, const N: usize> ToOwned<Heapless<N>> for [T] {
  type Owned = heapless::Vec<T, N>;
  type Storage<'a>
    = CowRepr<'a, Self, Heapless<N>>
  where
    T: 'a;
}

impl<const N: usize> From<heapless::String<N>> for Cow<'_, str, Heapless<N>> {
  #[inline]
  fn from(value: heapless::String<N>) -> Self {
    Self::owned(value)
  }
}

impl<T, const N: usize> From<heapless::Vec<T, N>> for Cow<'_, [T], Heapless<N>> {
  #[inline]
  fn from(value: heapless::Vec<T, N>) -> Self {
    Self::owned(value)
  }
}

impl<'a, P: ?Sized, const N: usize> From<&'a heapless::String<N>> for Cow<'a, str, P>
where
  str: ToOwned<P>,
{
  #[inline]
  fn from(value: &'a heapless::String<N>) -> Self {
    Self::borrowed(value.as_str())
  }
}

impl<'a, T, P: ?Sized, const N: usize> From<&'a heapless::Vec<T, N>> for Cow<'a, [T], P>
where
  [T]: ToOwned<P>,
{
  #[inline]
  fn from(value: &'a heapless::Vec<T, N>) -> Self {
    Self::borrowed(value.as_slice())
  }
}
