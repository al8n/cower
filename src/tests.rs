use core::borrow::Borrow;

use super::*;

const MESSAGE: Cow<'static, str> = Cow::Borrowed("hello");
const BYTES: Cow<'static, [u8]> = Cow::Borrowed(b"hello");

#[test]
fn reads_static_and_local_borrows() {
  assert!(MESSAGE.is_borrowed());
  assert!(!MESSAGE.is_owned());
  assert_eq!(MESSAGE.as_ref(), "hello");
  assert_eq!(Borrow::<str>::borrow(&MESSAGE), "hello");
  assert_eq!(BYTES.as_ref(), b"hello");

  let bytes = *b"hi";
  let text = core::str::from_utf8(&bytes).unwrap();
  let text: Cow<'_, str> = Cow::from(text);
  let slice: Cow<'_, [u8]> = Cow::from(bytes.as_slice());
  assert_eq!(text.as_ref(), "hi");
  assert_eq!(slice.as_ref(), &bytes);
  assert_eq!(text.as_ptr(), bytes.as_ptr());
}

#[test]
fn static_policy_is_blanket_and_independent_of_features() {
  struct NotClone;
  static VALUE: NotClone = NotClone;
  fn assert_copy<T: Copy>() {}
  assert_copy::<Cow<'_, NotClone, StaticPolicy>>();
  assert_copy::<Cow<'_, str, StaticPolicy>>();
  assert_copy::<Cow<'_, [NotClone], StaticPolicy>>();

  let value: Cow<'_, NotClone, StaticPolicy> = Cow::owned(&VALUE);
  assert!(core::ptr::eq(value.into_owned(), &VALUE));
  let local = NotClone;
  let borrowed: Cow<'_, NotClone, StaticPolicy> = Cow::borrowed(&local);
  assert!(core::ptr::eq(borrowed.as_ref(), &local));

  let mut text: Cow<'_, str, StaticPolicy> = Cow::borrowed("hello");
  let storage: &mut &'static str = text.to_mut();
  *storage = "world";
  assert_eq!(text.as_ref(), "world");
  assert!(text.is_owned());
  assert_eq!(text.try_into_owned().unwrap(), "world");
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn default_static_representations_are_copy_without_element_clone() {
  struct NotClone;
  fn assert_copy<T: Copy>() {}
  assert_copy::<Cow<'_, str>>();
  assert_copy::<Cow<'_, [NotClone]>>();

  let text: <str as ToOwned>::Owned = "hello";
  let slice: <[u8] as ToOwned>::Owned = b"hello";
  let owned_text: Cow<'_, str> = Cow::owned(text);
  let owned_slice: Cow<'_, [u8]> = Cow::owned(slice);
  assert_eq!(owned_text, MESSAGE);
  assert_eq!(owned_slice, BYTES);
  assert_eq!(MESSAGE.into_owned().as_ptr(), text.as_ptr());
  assert_eq!(BYTES.try_into_owned().unwrap().as_ptr(), slice.as_ptr());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn standard_representations_clone_on_first_mutation() {
  use std::{string::String, vec::Vec};

  let _: <str as ToOwned>::Owned = String::new();
  let _: <[u8] as ToOwned>::Owned = Vec::new();
  let _: <u32 as ToOwned>::Owned = 42;

  let original = String::from("local");
  let mut text: Cow<'_, str> = Cow::borrowed(&original);
  text.to_mut().push('!');
  assert_eq!(original, "local");
  assert_eq!(text.as_ref(), "local!");
  assert!(text.is_owned());

  let mut slice: Cow<'_, [u8]> = Cow::borrowed(b"hi");
  slice.to_mut().push(b'!');
  assert_eq!(slice.as_ref(), b"hi!");

  let owned = String::from("move");
  let pointer = owned.as_ptr();
  let mut value: Cow<'_, str> = Cow::owned(owned);
  assert_eq!(value.to_mut().as_ptr(), pointer);
  assert_eq!(value.try_to_mut().unwrap().as_ptr(), pointer);
  let moved = value.into_owned();
  assert_eq!(moved.as_ptr(), pointer);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn blanket_mapping_accepts_standard_to_owned_without_from() {
  // Mapping does not impose extra conversion or Clone bounds on Owned.
  struct View;
  struct Storage(View);
  impl Borrow<View> for Storage {
    fn borrow(&self) -> &View {
      &self.0
    }
  }
  impl std::borrow::ToOwned for View {
    type Owned = Storage;
    fn to_owned(&self) -> Storage {
      Storage(View)
    }
  }

  let value: Cow<'_, View> = Cow::owned(Storage(View));
  let _: &View = &value;
  assert!(value.is_owned());
}
