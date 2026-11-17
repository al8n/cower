use core::{borrow::Borrow, cmp::Ordering};
use std::{
  ffi::{OsStr, OsString},
  path::{Path, PathBuf},
};

use crate::{Cow, ToOwned, convert::owned_conversions};

owned_conversions!(OsStr, OsString);
owned_conversions!(Path, PathBuf);

impl<P: ?Sized> AsRef<Path> for Cow<'_, OsStr, P>
where
  OsStr: ToOwned<P>,
{
  fn as_ref(&self) -> &Path {
    Path::new::<OsStr>(self.as_ref())
  }
}

macro_rules! compare {
  ($borrowed:ty, $other:ty) => {
    impl<P: ?Sized> PartialEq<$other> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &$other) -> bool {
        <$borrowed as PartialEq>::eq(self, other)
      }
    }

    impl<P: ?Sized> PartialEq<Cow<'_, $borrowed, P>> for $other
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &Cow<'_, $borrowed, P>) -> bool {
        <$borrowed as PartialEq>::eq(self, other)
      }
    }

    impl<P: ?Sized> PartialOrd<$other> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn partial_cmp(&self, other: &$other) -> Option<Ordering> {
        <$borrowed as PartialOrd>::partial_cmp(self, other)
      }
    }

    impl<P: ?Sized> PartialOrd<Cow<'_, $borrowed, P>> for $other
    where
      $borrowed: ToOwned<P>,
    {
      fn partial_cmp(&self, other: &Cow<'_, $borrowed, P>) -> Option<Ordering> {
        <$borrowed as PartialOrd>::partial_cmp(self, other)
      }
    }
  };
}

compare!(OsStr, OsStr);
compare!(OsStr, &OsStr);
compare!(OsStr, OsString);
compare!(Path, Path);
compare!(Path, &Path);
compare!(Path, PathBuf);

macro_rules! compare_as_path {
  ($borrowed:ty, $other:ty) => {
    impl<P: ?Sized> PartialEq<$other> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &$other) -> bool {
        Path::new::<$borrowed>(self.as_ref()) == Path::new(other)
      }
    }

    impl<P: ?Sized> PartialEq<Cow<'_, $borrowed, P>> for $other
    where
      $borrowed: ToOwned<P>,
    {
      fn eq(&self, other: &Cow<'_, $borrowed, P>) -> bool {
        Path::new(self) == Path::new::<$borrowed>(other.as_ref())
      }
    }

    impl<P: ?Sized> PartialOrd<$other> for Cow<'_, $borrowed, P>
    where
      $borrowed: ToOwned<P>,
    {
      fn partial_cmp(&self, other: &$other) -> Option<Ordering> {
        Path::new::<$borrowed>(self.as_ref()).partial_cmp(Path::new(other))
      }
    }

    impl<P: ?Sized> PartialOrd<Cow<'_, $borrowed, P>> for $other
    where
      $borrowed: ToOwned<P>,
    {
      fn partial_cmp(&self, other: &Cow<'_, $borrowed, P>) -> Option<Ordering> {
        Path::new(self).partial_cmp(Path::new::<$borrowed>(other.as_ref()))
      }
    }
  };
}

compare_as_path!(Path, OsStr);
compare_as_path!(Path, &OsStr);
compare_as_path!(Path, OsString);
compare_as_path!(OsStr, Path);
compare_as_path!(OsStr, &Path);
compare_as_path!(OsStr, PathBuf);

impl<'a, P: ?Sized> FromIterator<Cow<'a, OsStr, P>> for OsString
where
  OsStr: ToOwned<P, Owned = OsString>,
{
  fn from_iter<I: IntoIterator<Item = Cow<'a, OsStr, P>>>(iter: I) -> Self {
    iter.into_iter().map(std::borrow::Cow::from).collect()
  }
}

impl<'a, P: ?Sized> Extend<Cow<'a, OsStr, P>> for OsString
where
  OsStr: ToOwned<P, Owned = OsString>,
{
  fn extend<I: IntoIterator<Item = Cow<'a, OsStr, P>>>(&mut self, iter: I) {
    self.extend(iter.into_iter().map(std::borrow::Cow::from));
  }
}
