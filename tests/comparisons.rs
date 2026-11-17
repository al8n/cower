use cower::{Cow, StaticPolicy};

fn equality<L, R>(left: &L, right: &R, expected: bool)
where
  L: ?Sized + PartialEq<R>,
  R: ?Sized + PartialEq<L>,
{
  assert_eq!(left.eq(right), expected);
  assert_eq!(right.eq(left), expected);
}

#[test]
fn string_and_c_string_equality_is_symmetric() {
  let text: Cow<'_, str, StaticPolicy> = Cow::owned("hello");
  for other in ["hello", "different", ""] {
    equality(&text, other, other == "hello");
    equality(&text, &other, other == "hello");
    #[cfg(any(feature = "std", feature = "alloc"))]
    equality(&text, &String::from(other), other == "hello");
  }

  let text: Cow<'_, core::ffi::CStr, StaticPolicy> = Cow::borrowed(c"hello");
  for other in [c"hello", c"different", c""] {
    equality(&text, other, other == c"hello");
    equality(&text, &other, other == c"hello");
    #[cfg(any(feature = "std", feature = "alloc"))]
    equality(&text, &std::ffi::CString::from(other), other == c"hello");
  }
}

#[test]
fn slice_comparisons_accept_shared_mutable_and_owned_containers() {
  let value = Cow::<'_, [u8], StaticPolicy>::owned(&[1, 2]);
  for mut other in [[1u8, 2], [1, 3]] {
    let expected = other == [1, 2];
    equality(&value, &other.as_slice(), expected);
    equality(&value, &other.as_mut_slice(), expected);
    #[cfg(any(feature = "std", feature = "alloc"))]
    equality(&value, &other.to_vec(), expected);
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn standard_cow_comparisons_preserve_order_and_incomparability() {
  use std::borrow::Cow as StdCow;
  let value: Cow<'_, str, StaticPolicy> = Cow::owned("middle");
  for other in ["", "middle", "zebra"] {
    for other in [StdCow::Borrowed(other), StdCow::Owned(other.to_owned())] {
      equality(&value, &other, other == "middle");
      assert_eq!(value.partial_cmp(&other), "middle".partial_cmp(&*other));
      assert_eq!(other.partial_cmp(&value), (*other).partial_cmp("middle"));
    }
  }
  let nan: Cow<'_, f64, StaticPolicy> = Cow::borrowed(&f64::NAN);
  let number: StdCow<'_, f64> = StdCow::Owned(1.0);
  equality(&nan, &number, false);
  assert_eq!(nan.partial_cmp(&number), None);
  assert_eq!(number.partial_cmp(&nan), None);
}

#[cfg(feature = "std")]
#[test]
fn path_and_os_string_comparisons_match_native_semantics() {
  use core::cmp::Ordering;
  use std::{ffi::OsStr, path::Path};

  fn ordered<L, R>(left: &L, right: &R, expected: Ordering)
  where
    L: ?Sized + PartialOrd<R>,
    R: ?Sized + PartialOrd<L>,
  {
    equality(left, right, expected == Ordering::Equal);
    assert_eq!(left.partial_cmp(right), Some(expected));
    assert_eq!(right.partial_cmp(left), Some(expected.reverse()));
  }

  // Path comparison normalizes separators and `.` components; OsStr comparison
  // keeps their literal spelling. Exercise both through every supported RHS type.
  for (left, right) in [
    ("a//b", "a/b"),
    ("a/./b", "a/b"),
    ("a", "z"),
    ("z", ""),
    ("你好", "你好"),
  ] {
    let path = Path::new(left);
    let os = OsStr::new(left);
    let other_path = Path::new(right);
    let other_os = OsStr::new(right);
    for value in [
      Cow::<'_, Path>::borrowed(path),
      Cow::owned(path.to_path_buf()),
    ] {
      let expected = path.cmp(other_path);
      ordered(&value, other_path, expected);
      ordered(&value, &other_path, expected);
      ordered(&value, &other_path.to_path_buf(), expected);
      ordered(&value, other_os, expected);
      ordered(&value, &other_os, expected);
      ordered(&value, &other_os.to_os_string(), expected);
    }
    for value in [
      Cow::<'_, OsStr>::borrowed(os),
      Cow::owned(os.to_os_string()),
    ] {
      let expected = os.cmp(other_os);
      ordered(&value, other_os, expected);
      ordered(&value, &other_os, expected);
      ordered(&value, &other_os.to_os_string(), expected);
      let expected = path.cmp(other_path);
      ordered(&value, other_path, expected);
      ordered(&value, &other_path, expected);
      ordered(&value, &other_path.to_path_buf(), expected);
    }
  }
}
