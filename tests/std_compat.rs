use cower::{Cow, CowRepr};

#[test]
fn borrowed_policy_string_views_retain_the_original_borrow() {
  use core::ptr;
  use cower::{BorrowedPolicy, CowStorage};

  let utf8 = [b'a', 0xc3, 0xa9];
  let source = core::str::from_utf8(&utf8).unwrap();
  let (by_ref, by_value): (&str, &str) = {
    let mut text = Cow::<str, BorrowedPolicy>::from_ref(source);
    assert!(ptr::eq(text.as_borrowed().unwrap(), source));
    assert!(text.as_owned().is_none());
    assert!(CowStorage::as_owned_mut(text.as_storage_mut()).is_none());
    assert!(ptr::eq(text.as_str(), source));
    assert!(ptr::eq(text.as_bytes(), &utf8[..]));
    ((&text).into(), text.into())
  };
  assert!(ptr::eq(by_ref, source));
  assert!(ptr::eq(by_value, source));
}

#[test]
fn static_constructors_preserve_the_input_slice_for_each_backing() {
  use core::ptr;
  use cower::{BorrowedPolicy, StaticPolicy};

  static INPUT: [u8; 3] = [0, 1, 255];
  let reference = Cow::<[u8], BorrowedPolicy>::from_static(&INPUT);
  let repr = Cow::<[u8], StaticPolicy>::from_static(&INPUT);
  let default = Cow::<[u8]>::from_static(&INPUT);
  assert!(ptr::eq(reference.as_bytes(), &INPUT[..]));
  for value in [repr.as_borrowed(), default.as_borrowed()] {
    assert!(ptr::eq(value.unwrap(), &INPUT[..]));
  }
}

#[test]
fn borrowed_policy_preserves_slice_identity_and_layout() {
  use core::mem::{align_of, size_of};
  use cower::BorrowedPolicy;

  struct Item<'a>(&'a u8);
  type Items<'a> = Cow<'a, [Item<'a>], BorrowedPolicy>;
  fn new<'a>(values: impl Into<Items<'a>>) -> Items<'a> {
    values.into()
  }
  let number = 7;
  let items = [Item(&number)];
  let (by_value, by_ref): (&[Item<'_>], &[Item<'_>]) = {
    let value = new(&items);
    let copy = value;
    let cloned: Items<'_> = (&value).into();
    assert!(matches!(cloned.into_repr(), CowRepr::Borrowed(_)));
    (copy.into(), (&value).into())
  };
  assert!(core::ptr::eq(by_value, &items[..]));
  assert!(core::ptr::eq(by_ref, by_value));
  assert_eq!(*by_ref[0].0, 7);
  assert_eq!(size_of::<Items<'_>>(), size_of::<&[Item<'_>]>());
  assert_eq!(align_of::<Items<'_>>(), align_of::<&[Item<'_>]>());
}

#[test]
fn default_and_array_borrows_work_in_every_tier() {
  let text: Cow<'_, str> = Cow::default();
  assert!(text.is_owned());
  assert_eq!(text, "");
  assert_eq!("", text);
  let source = [1, 2, 3];
  let values: Cow<'_, [i32]> = (&source).into();
  assert!(values.is_borrowed());
  assert_eq!(values, &source[..]);
  assert_eq!(&source[..], values);
  assert!(matches!(values.into_repr(), CowRepr::Borrowed([1, 2, 3])));
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod heap {
  use super::*;
  use std::{
    borrow::Cow as StdCow,
    ffi::{CStr, CString},
    rc::Rc,
    sync::Arc,
  };

  #[test]
  fn owned_conversions_move_allocations_and_borrowed_conversions_retain_references() {
    let source = String::from("hello");
    let pointer = source.as_ptr();
    let value: Cow<'_, str> = source.into();
    assert!(value.is_owned());
    let source = String::from(value);
    assert_eq!(source.as_ptr(), pointer);
    let borrowed: Cow<'_, str> = (&source).into();
    assert!(borrowed.is_borrowed());
    assert_eq!(borrowed.as_ptr(), pointer);
    assert_eq!(borrowed, source);
    assert_eq!(source, borrowed);
    let boxed: Box<str> = borrowed.into();
    assert_eq!(&*boxed, "hello");

    let source = vec![1, 2, 3];
    let pointer = source.as_ptr();
    let borrowed: Cow<'_, [i32]> = (&source).into();
    assert_eq!(borrowed, source);
    assert_eq!(source, borrowed);
    let value: Cow<'_, [i32]> = source.into();
    let source: Vec<_> = value.into();
    assert_eq!(source.as_ptr(), pointer);
    let value: Cow<'_, [i32]> = source.into();
    let boxed: Box<[i32]> = value.into();
    assert_eq!(&*boxed, &[1, 2, 3]);
  }

  #[test]
  fn string_addition_matches_standard_variants_and_contents() {
    for left in ["", "a"] {
      for right in ["", "b"] {
        for left_owned in [false, true] {
          for right_owned in [false, true] {
            let lhs: Cow<'_, str> = if left_owned {
              Cow::Owned(left.into())
            } else {
              Cow::Borrowed(left)
            };
            let rhs: Cow<'_, str> = if right_owned {
              Cow::Owned(right.into())
            } else {
              Cow::Borrowed(right)
            };
            let expected = StdCow::from(lhs.clone()) + StdCow::from(rhs.clone());
            let actual = lhs + rhs;
            assert_eq!(actual, expected);
            assert_eq!(expected, actual);
            assert_eq!(
              actual.is_borrowed(),
              matches!(expected, StdCow::Borrowed(_))
            );
          }
        }
      }
    }
    let mut text: Cow<'_, str> = Cow::Borrowed("a");
    text += "b";
    text += Cow::Owned(String::from("c"));
    assert_eq!(text + "d", "abcd");
  }

  #[test]
  fn iterator_and_shared_pointer_conversions_work() {
    let chars: Cow<'_, str> = ['a', 'b'].into_iter().collect();
    let pieces: Cow<'_, str> = ["a", "b"].into_iter().collect();
    let strings: Cow<'_, str> = [String::from("a"), String::from("b")].into_iter().collect();
    assert_eq!(chars, pieces);
    assert_eq!(pieces, strings);
    let numbers: Cow<'_, [u8]> = (0..3).collect();
    assert_eq!(numbers, &[0, 1, 2][..]);
    let mut joined: String = [chars, pieces].into_iter().collect();
    joined.extend([strings]);
    assert_eq!(joined, "ababab");
    let value: Cow<'_, str> = Cow::Borrowed("hello");
    let shared: Rc<str> = value.clone().into();
    let atomic: Arc<str> = value.into();
    assert_eq!(&*shared, &*atomic);
    let value: Cow<'_, str> = Cow::Borrowed("failure");
    let error: Box<dyn std::error::Error + Send + Sync> = value.into();
    assert_eq!(error.to_string(), "failure");
  }

  #[test]
  fn c_string_conversions_and_comparisons_work() {
    let source = CString::new("hello").unwrap();
    let pointer = source.as_ptr();
    let borrowed: Cow<'_, CStr> = (&source).into();
    assert_eq!(borrowed, source.as_c_str());
    assert_eq!(source, borrowed);
    let value: Cow<'_, CStr> = source.into();
    let source = CString::from(value);
    assert_eq!(source.as_ptr(), pointer);
    let value: Cow<'_, CStr> = source.into();
    let boxed: Box<CStr> = value.into();
    assert_eq!(boxed.to_bytes(), b"hello");
  }
}

#[cfg(feature = "std")]
#[test]
fn path_and_os_string_interfaces_match_standard_cow() {
  use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
  };
  let path: Cow<'_, Path> = PathBuf::from("a/b").into();
  assert_eq!(path, Path::new("a/b"));
  assert_eq!(PathBuf::from("a/b"), path);
  assert!(path < Path::new("b"));
  assert_eq!(path, OsStr::new("a/b"));
  assert_eq!(OsString::from("a/b"), path);
  let path = PathBuf::from(path);
  let borrowed: Cow<'_, Path> = (&path).into();
  let boxed: Box<Path> = borrowed.into();
  assert_eq!(&*boxed, path);
  let os: Cow<'_, OsStr> = OsString::from("a/b").into();
  assert_eq!(os, Path::new("a/b"));
  assert_eq!(AsRef::<Path>::as_ref(&os), Path::new("a/b"));
  let mut joined: OsString = [os.clone()].into_iter().collect();
  joined.extend([os]);
  assert_eq!(joined, "a/ba/b");
}
