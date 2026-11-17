use crate::{Cow, ToOwned};

impl<T: ?Sized + ToOwned<P>, P: ?Sized> Default for Cow<'_, T, P>
where
  T::Owned: Default,
{
  #[inline]
  fn default() -> Self {
    Self::owned(T::Owned::default())
  }
}

impl<'a, T, P: ?Sized, const N: usize> From<&'a [T; N]> for Cow<'a, [T], P>
where
  [T]: ToOwned<P>,
{
  #[inline]
  fn from(value: &'a [T; N]) -> Self {
    Self::borrowed(value)
  }
}

// Concrete RHS types avoid overlap with cross-policy Cow comparisons.
macro_rules! content_eq {
  ($borrowed:ty, $other:ty) => {
    impl<P: ?Sized> PartialEq<$other> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      #[inline]
      fn eq(&self, other: &$other) -> bool {
        <$borrowed as PartialEq>::eq(self.as_ref(), &**other)
      }
    }

    impl<P: ?Sized> PartialEq<Cow<'_, $borrowed, P>> for $other
    where
      $borrowed: ToOwned<P>,
    {
      #[inline]
      fn eq(&self, other: &Cow<'_, $borrowed, P>) -> bool {
        <$borrowed as PartialEq>::eq(self.as_ref(), other.as_ref())
      }
    }
  };
}

// Unsized values need a different dereference depth than references/containers.
macro_rules! borrowed_eq {
  ($borrowed:ty) => {
    impl<P: ?Sized> PartialEq<$borrowed> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &$borrowed) -> bool {
        <$borrowed as PartialEq>::eq(self.as_ref(), other)
      }
    }
    impl<P: ?Sized> PartialEq<Cow<'_, $borrowed, P>> for $borrowed
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &Cow<'_, $borrowed, P>) -> bool {
        <$borrowed as PartialEq>::eq(self, other.as_ref())
      }
    }
  };
}

borrowed_eq!(str);
content_eq!(str, &str);
borrowed_eq!(core::ffi::CStr);
content_eq!(core::ffi::CStr, &core::ffi::CStr);

// Use T for the element on the reverse comparison's left-hand side as well.
impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<&[U]> for Cow<'_, [T], P>
where
  [T]: ToOwned<P>,
{
  fn eq(&self, other: &&[U]) -> bool {
    PartialEq::eq(self.as_ref(), *other)
  }
}

impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<Cow<'_, [U], P>> for &[T]
where
  [U]: ToOwned<P>,
{
  fn eq(&self, other: &Cow<'_, [U], P>) -> bool {
    PartialEq::eq(*self, other.as_ref())
  }
}

impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<&mut [U]> for Cow<'_, [T], P>
where
  [T]: ToOwned<P>,
{
  fn eq(&self, other: &&mut [U]) -> bool {
    PartialEq::eq(self.as_ref(), &**other)
  }
}

impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<Cow<'_, [U], P>> for &mut [T]
where
  [U]: ToOwned<P>,
{
  fn eq(&self, other: &Cow<'_, [U], P>) -> bool {
    PartialEq::eq(&**self, other.as_ref())
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod heap {
  use core::{
    cmp::Ordering,
    ops::{Add, AddAssign},
  };
  use std::{borrow::Cow as StdCow, string::String, vec::Vec};

  use super::*;

  content_eq!(str, String);
  content_eq!(core::ffi::CStr, std::ffi::CString);

  impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<Vec<U>> for Cow<'_, [T], P>
  where
    [T]: ToOwned<P>,
  {
    fn eq(&self, other: &Vec<U>) -> bool {
      PartialEq::eq(self.as_ref(), other.as_slice())
    }
  }

  impl<T: PartialEq<U>, U, P: ?Sized> PartialEq<Cow<'_, [U], P>> for Vec<T>
  where
    [U]: ToOwned<P>,
  {
    fn eq(&self, other: &Cow<'_, [U], P>) -> bool {
      PartialEq::eq(self.as_slice(), other.as_ref())
    }
  }

  impl<T, P: ?Sized> FromIterator<T> for Cow<'_, [T], P>
  where
    [T]: ToOwned<P>,
    <[T] as ToOwned<P>>::Owned: FromIterator<T>,
  {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      Self::owned(iter.into_iter().collect())
    }
  }

  impl<Item, P: ?Sized> FromIterator<Item> for Cow<'_, str, P>
  where
    str: ToOwned<P, Owned = String>,
    String: FromIterator<Item>,
  {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
      Self::owned(iter.into_iter().collect())
    }
  }

  impl<'a, P: ?Sized> Add<&'a str> for Cow<'a, str, P>
  where
    str: ToOwned<P, Owned = String>,
  {
    type Output = Self;
    fn add(mut self, rhs: &'a str) -> Self {
      self += rhs;
      self
    }
  }

  impl<P: ?Sized> Add for Cow<'_, str, P>
  where
    str: ToOwned<P, Owned = String>,
  {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
      self += rhs;
      self
    }
  }

  impl<'a, P: ?Sized> AddAssign<&'a str> for Cow<'a, str, P>
  where
    str: ToOwned<P, Owned = String>,
  {
    fn add_assign(&mut self, rhs: &'a str) {
      // Delegate to std to retain its empty-string and allocation behavior.
      let mut value = StdCow::from(core::mem::replace(self, Self::borrowed("")));
      value += rhs;
      *self = Self::from(value);
    }
  }

  impl<P: ?Sized> AddAssign for Cow<'_, str, P>
  where
    str: ToOwned<P, Owned = String>,
  {
    fn add_assign(&mut self, rhs: Self) {
      let mut value = StdCow::from(core::mem::replace(self, Self::borrowed("")));
      value += StdCow::from(rhs);
      *self = Self::from(value);
    }
  }

  impl<'a, P: ?Sized> FromIterator<Cow<'a, str, P>> for String
  where
    str: ToOwned<P, Owned = String>,
  {
    fn from_iter<I: IntoIterator<Item = Cow<'a, str, P>>>(iter: I) -> Self {
      iter.into_iter().map(StdCow::from).collect()
    }
  }

  impl<'a, P: ?Sized> Extend<Cow<'a, str, P>> for String
  where
    str: ToOwned<P, Owned = String>,
  {
    fn extend<I: IntoIterator<Item = Cow<'a, str, P>>>(&mut self, iter: I) {
      self.extend(iter.into_iter().map(StdCow::from));
    }
  }

  impl<T, U, P: ?Sized> PartialEq<StdCow<'_, U>> for Cow<'_, T, P>
  where
    T: ?Sized + ToOwned<P> + PartialEq<U>,
    U: ?Sized + std::borrow::ToOwned,
  {
    fn eq(&self, other: &StdCow<'_, U>) -> bool {
      PartialEq::eq(self.as_ref(), &**other)
    }
  }

  impl<T, U, P: ?Sized> PartialEq<Cow<'_, U, P>> for StdCow<'_, T>
  where
    T: ?Sized + std::borrow::ToOwned + PartialEq<U>,
    U: ?Sized + ToOwned<P>,
  {
    fn eq(&self, other: &Cow<'_, U, P>) -> bool {
      PartialEq::eq(self.as_ref(), other.as_ref())
    }
  }

  impl<T, U, P: ?Sized> PartialOrd<StdCow<'_, U>> for Cow<'_, T, P>
  where
    T: ?Sized + ToOwned<P> + PartialOrd<U>,
    U: ?Sized + std::borrow::ToOwned,
  {
    fn partial_cmp(&self, other: &StdCow<'_, U>) -> Option<Ordering> {
      PartialOrd::partial_cmp(self.as_ref(), &**other)
    }
  }

  impl<T, U, P: ?Sized> PartialOrd<Cow<'_, U, P>> for StdCow<'_, T>
  where
    T: ?Sized + std::borrow::ToOwned + PartialOrd<U>,
    U: ?Sized + ToOwned<P>,
  {
    fn partial_cmp(&self, other: &Cow<'_, U, P>) -> Option<Ordering> {
      PartialOrd::partial_cmp(self.as_ref(), other.as_ref())
    }
  }
}
