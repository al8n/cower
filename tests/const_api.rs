// Also compiled as a downstream no_std library by ci/const-api on the MSRV
// lanes, without building this repository's higher-MSRV dev-dependencies.
#![cfg_attr(not(test), no_std)]

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;

use cower::{Cow, CowRepr, StaticPolicy};

pub static DEFAULT: Cow<'static, str> = Cow::<str>::from_static("hé🦀");
pub static STATIC_BORROWED: Cow<'static, str, StaticPolicy> =
  Cow::<str, StaticPolicy>::from_static("hé🦀");
pub static STATIC_OWNED: Cow<'static, str, StaticPolicy> =
  Cow::from_storage(CowRepr::Owned("owned🦀"));
pub static STATIC_BYTES: Cow<'static, [u8], StaticPolicy> =
  Cow::<[u8], StaticPolicy>::from_static(b"bytes");
pub static DEFAULT_BYTES: Cow<'static, [u8]> = Cow::<[u8]>::from_static(b"bytes");
pub static STATIC_NUMBER: Cow<'static, u32, StaticPolicy> =
  Cow::<u32, StaticPolicy>::from_static(&42);

pub const fn read_static<'s>(value: &'s Cow<'_, str, StaticPolicy>) -> &'s str {
  value.as_str()
}

pub const fn read_static_bytes<'s>(value: &'s Cow<'_, str, StaticPolicy>) -> &'s [u8] {
  value.as_bytes()
}

// String equality through PartialEq is not const on the MSRV. Compare the
// complete UTF-8 contents at compile time instead of only checking their length.
const fn assert_text(actual: &str, expected: &str) {
  assert_bytes(actual.as_bytes(), expected.as_bytes());
}

const fn assert_bytes(actual: &[u8], expected: &[u8]) {
  assert!(actual.len() == expected.len());
  let mut index = 0;
  while index < actual.len() {
    assert!(actual[index] == expected[index]);
    index += 1;
  }
}

const _: () = {
  const BORROWED_TEXT: &str = STATIC_BORROWED.as_str();
  const OWNED_TEXT: &str = STATIC_OWNED.as_str();
  const THROUGH_FN: &str = read_static(&STATIC_BORROWED);
  assert_text(BORROWED_TEXT, "hé🦀");
  assert_text(OWNED_TEXT, "owned🦀");
  assert_text(THROUGH_FN, BORROWED_TEXT);
  assert_text(read_static(&STATIC_OWNED), OWNED_TEXT);
  const BORROWED_BYTES: &[u8] = STATIC_BORROWED.as_bytes();
  const OWNED_BYTES: &[u8] = STATIC_OWNED.as_bytes();
  const BYTES_THROUGH_FN: &[u8] = read_static_bytes(&STATIC_BORROWED);
  assert_bytes(BORROWED_BYTES, b"h\xc3\xa9\xf0\x9f\xa6\x80");
  assert_bytes(OWNED_BYTES, b"owned\xf0\x9f\xa6\x80");
  assert_bytes(BYTES_THROUGH_FN, BORROWED_BYTES);
  assert_bytes(read_static_bytes(&STATIC_OWNED), OWNED_BYTES);
  assert!(STATIC_BYTES.as_storage().is_borrowed());
  assert!(STATIC_NUMBER.as_storage().is_borrowed());
};

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub const fn read_default<'s>(value: &'s Cow<'_, str>) -> &'s str {
  value.as_str()
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[rustversion::attr(since(1.87), const)]
pub fn read_default<'s>(value: &'s Cow<'_, str>) -> &'s str {
  value.as_str()
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub const fn read_default_bytes<'s>(value: &'s Cow<'_, str>) -> &'s [u8] {
  value.as_bytes()
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[rustversion::attr(since(1.87), const)]
pub fn read_default_bytes<'s>(value: &'s Cow<'_, str>) -> &'s [u8] {
  value.as_bytes()
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
const _: () = {
  static OWNED: Cow<'static, str> = Cow::Owned("owned🦀");
  const BORROWED_TEXT: &str = DEFAULT.as_str();
  const OWNED_TEXT: &str = OWNED.as_str();
  const THROUGH_FN: &str = read_default(&DEFAULT);
  assert_text(BORROWED_TEXT, "hé🦀");
  assert_text(OWNED_TEXT, "owned🦀");
  assert_text(THROUGH_FN, BORROWED_TEXT);
  assert_text(read_default(&OWNED), OWNED_TEXT);
  const BORROWED_BYTES: &[u8] = DEFAULT.as_bytes();
  const OWNED_BYTES: &[u8] = OWNED.as_bytes();
  const BYTES_THROUGH_FN: &[u8] = read_default_bytes(&DEFAULT);
  assert_bytes(BORROWED_BYTES, b"h\xc3\xa9\xf0\x9f\xa6\x80");
  assert_bytes(OWNED_BYTES, b"owned\xf0\x9f\xa6\x80");
  assert_bytes(BYTES_THROUGH_FN, BORROWED_BYTES);
  assert_bytes(read_default_bytes(&OWNED), OWNED_BYTES);
  assert!(DEFAULT_BYTES.as_storage().is_borrowed());
};

#[cfg(any(feature = "std", feature = "alloc"))]
#[rustversion::since(1.87)]
const _: () = {
  static EMPTY: Cow<'static, str> = Cow::Owned(alloc::string::String::new());
  const BORROWED_TEXT: &str = DEFAULT.as_str();
  const OWNED_TEXT: &str = EMPTY.as_str();
  const THROUGH_FN: &str = read_default(&DEFAULT);
  assert_text(BORROWED_TEXT, "hé🦀");
  assert_text(OWNED_TEXT, "");
  assert_text(THROUGH_FN, BORROWED_TEXT);
  assert_text(read_default(&EMPTY), OWNED_TEXT);
  const BORROWED_BYTES: &[u8] = DEFAULT.as_bytes();
  const OWNED_BYTES: &[u8] = EMPTY.as_bytes();
  const BYTES_THROUGH_FN: &[u8] = read_default_bytes(&DEFAULT);
  assert_bytes(BORROWED_BYTES, b"h\xc3\xa9\xf0\x9f\xa6\x80");
  assert_bytes(OWNED_BYTES, b"");
  assert_bytes(BYTES_THROUGH_FN, BORROWED_BYTES);
  assert_bytes(read_default_bytes(&EMPTY), OWNED_BYTES);
  assert!(matches!(
    DEFAULT_BYTES.as_storage(),
    alloc::borrow::Cow::Borrowed(_)
  ));
};

// Borrowing must not clone, allocate, or check inline capacity. Exercise every
// built-in policy's constructor in a const context, including zero capacities.
macro_rules! check_policy {
  ($policy:ty, $borrowed:ty, $value:expr) => {
    const _: () = {
      static VALUE: Cow<'static, $borrowed, $policy> =
        Cow::<$borrowed, $policy>::from_static($value);
      assert!(VALUE.as_storage().is_borrowed());
    };
  };
}

// Keep the macro exercised even when every optional integration is disabled.
check_policy!(StaticPolicy, str, "");
#[cfg(feature = "heapless_0_9")]
check_policy!(cower::Heapless<0>, str, "longer than capacity");
#[cfg(feature = "heapless_0_9")]
check_policy!(cower::Heapless<0>, [u8], b"longer than capacity");
#[cfg(feature = "arrayvec_0_7")]
check_policy!(cower::ArrayVecPolicy<0>, str, "longer than capacity");
#[cfg(feature = "arrayvec_0_7")]
check_policy!(cower::ArrayVecPolicy<0>, [u8], b"longer than capacity");
#[cfg(feature = "smallvec_1")]
check_policy!(cower::SmallVecPolicy<0>, [u8], b"longer than capacity");
#[cfg(feature = "bytes_1")]
check_policy!(cower::BytesPolicy, [u8], b"bytes");
#[cfg(feature = "compact_str_0_9")]
check_policy!(cower::CompactStrPolicy, str, "compact");
#[cfg(feature = "ecow_0_3")]
check_policy!(cower::EcoPolicy, str, "shared");
#[cfg(feature = "ecow_0_3")]
check_policy!(cower::EcoPolicy, [u8], b"shared");
#[cfg(feature = "ecow_0_3")]
check_policy!(cower::EcoBytesPolicy, [u8], b"shared");
#[cfg(feature = "arcstr_1")]
check_policy!(cower::ArcStrPolicy, str, "shared");
#[cfg(feature = "arcstr_1")]
check_policy!(cower::ArcSubstrPolicy, str, "substring");
#[cfg(feature = "smol-bytes_0_2")]
check_policy!(
  cower::SmolBytesBufferPolicy,
  str,
  "a borrow longer than the inline buffer capacity"
);
#[cfg(feature = "smol-bytes_0_2")]
check_policy!(cower::SmolBytesBufferPolicy, [u8], b"bytes");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesPolicy, str, "shared");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesPolicy, [u8], b"shared");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesCompactPolicy, str, "compact");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesCompactPolicy, [u8], b"compact");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesMutPolicy, str, "mutable");
#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
check_policy!(cower::SmolBytesMutPolicy, [u8], b"mutable");

#[test]
fn static_constructors_preserve_borrows() {
  assert!(DEFAULT.is_borrowed());
  assert!(STATIC_BORROWED.is_borrowed());
  assert_eq!(read_default(&DEFAULT), "hé🦀");
  assert_eq!(read_static(&STATIC_BORROWED), "hé🦀");
  assert_eq!(read_static(&STATIC_OWNED), "owned🦀");
  assert_eq!(read_default_bytes(&DEFAULT), "hé🦀".as_bytes());
  assert_eq!(read_static_bytes(&STATIC_BORROWED), "hé🦀".as_bytes());
  assert_eq!(read_static_bytes(&STATIC_OWNED), "owned🦀".as_bytes());
  assert_eq!(DEFAULT_BYTES.as_ref(), b"bytes");
  assert_eq!(STATIC_BYTES.as_ref(), b"bytes");
  assert_eq!(*STATIC_NUMBER.as_ref(), 42);
  assert!(core::ptr::eq(
    read_default(&DEFAULT),
    DEFAULT.as_borrowed().unwrap()
  ));
  assert!(core::ptr::eq(
    read_static(&STATIC_OWNED),
    *STATIC_OWNED.as_owned().unwrap()
  ));
}

#[test]
fn short_lived_borrows_remain_supported() {
  let bytes = [b'h', b'i'];
  let text = core::str::from_utf8(&bytes).unwrap();
  let default: Cow<'_, str> = Cow::borrowed(text);
  let static_policy: Cow<'_, str, StaticPolicy> = Cow::borrowed(text);
  assert!(core::ptr::eq(read_default(&default), text));
  assert!(core::ptr::eq(read_static(&static_policy), text));
  assert!(core::ptr::eq(read_default_bytes(&default), &bytes[..]));
  assert!(core::ptr::eq(read_static_bytes(&static_policy), &bytes[..]));
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn owned_heap_string_access_preserves_allocation_and_tracks_mutation() {
  let owned = alloc::string::String::from("hé🦀");
  let pointer = owned.as_ptr();
  let mut value: Cow<'static, str> = Cow::Owned(owned);
  assert_eq!(read_default(&value), "hé🦀");
  assert_eq!(read_default(&value).as_ptr(), pointer);
  assert_eq!(read_default_bytes(&value), "hé🦀".as_bytes());
  assert_eq!(read_default_bytes(&value).as_ptr(), pointer);
  value.to_mut().push('!');
  assert_eq!(read_default(&value), "hé🦀!");
  assert_eq!(read_default_bytes(&value), "hé🦀!".as_bytes());
  assert!(value.is_owned());
}
