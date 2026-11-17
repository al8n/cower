#![cfg(feature = "borsh_1")]

use borsh_1::{from_slice, to_vec};
use cower::{Cow, StaticPolicy};

#[test]
fn borrowed_storage_has_the_standard_content_format() {
  let text: Cow<'_, str, StaticPolicy> = Cow::borrowed("你好");
  assert_eq!(to_vec(&text).unwrap(), to_vec("你好").unwrap());
  let values: Cow<'_, [u16], StaticPolicy> = Cow::borrowed(&[1, 2, 256]);
  assert_eq!(
    to_vec(&values).unwrap(),
    to_vec(&[1u16, 2, 256][..]).unwrap()
  );
}

#[test]
fn deserialization_needs_only_owned_traits() {
  enum Policy {}
  impl cower::ToOwned<Policy> for u16 {
    type Owned = u16;
    type Storage<'a> = cower::CowRepr<'a, Self, Policy>;
  }
  let value: Cow<'_, u16, Policy> = from_slice(&[42, 0]).unwrap();
  assert!(value.is_owned());
  assert_eq!(*value.as_ref(), 42);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn default_storage_decodes_owned_and_rejects_malformed_data() {
  let bytes = to_vec("hello").unwrap();
  let value: Cow<'_, str> = from_slice(&bytes).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "hello");
  assert_eq!(
    to_vec(&value).unwrap(),
    to_vec(&std::borrow::Cow::Borrowed("hello")).unwrap()
  );
  assert!(from_slice::<Cow<'_, str>>(&bytes[..bytes.len() - 1]).is_err());
  assert!(from_slice::<Cow<'_, str>>(&[1, 0, 0, 0, 0xff]).is_err());
  let bytes = to_vec(&vec![1u16, 2, 256]).unwrap();
  let value: Cow<'_, [u16]> = from_slice(&bytes).unwrap();
  assert!(value.is_owned());
  assert_eq!(value.as_ref(), &[1, 2, 256]);
}

#[cfg(feature = "arrayvec_0_7")]
#[test]
fn arrayvec_decoding_enforces_byte_and_element_capacities() {
  use cower::ArrayVecPolicy;
  let bytes = to_vec("🦀").unwrap();
  assert!(from_slice::<Cow<'_, str, ArrayVecPolicy<3>>>(&bytes).is_err());
  let value: Cow<'_, str, ArrayVecPolicy<4>> = from_slice(&bytes).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "🦀");
  let bytes = to_vec(&vec![1u16, 2, 3]).unwrap();
  assert!(from_slice::<Cow<'_, [u16], ArrayVecPolicy<2>>>(&bytes).is_err());
  let value: Cow<'_, [u16], ArrayVecPolicy<3>> = from_slice(&bytes).unwrap();
  assert_eq!(value.as_ref(), &[1, 2, 3]);
}

#[cfg(feature = "smol-bytes_0_2")]
#[test]
fn smol_buffer_decoding_enforces_capacity() {
  use cower::SmolBytesBufferPolicy;
  let bytes = to_vec(&vec![1u8; smol_bytes_0_2::INLINE_CAP + 1]).unwrap();
  assert!(from_slice::<Cow<'_, [u8], SmolBytesBufferPolicy>>(&bytes).is_err());
  let bytes = to_vec("你好").unwrap();
  let value: Cow<'_, str, SmolBytesBufferPolicy> = from_slice(&bytes).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "你好");
}

#[cfg(feature = "bytes_1")]
#[test]
fn bytes_owned_deserialization_is_forwarded() {
  let bytes = to_vec(&vec![1u8, 2, 3]).unwrap();
  let value: Cow<'_, [u8], cower::BytesPolicy> = from_slice(&bytes).unwrap();
  assert!(value.is_owned());
  assert_eq!(value.as_ref(), &[1, 2, 3]);
}

#[cfg(feature = "compact_str_0_9")]
#[test]
fn compact_string_owned_deserialization_is_forwarded() {
  let value: Cow<'_, str, cower::CompactStrPolicy> = from_slice(&to_vec("hello").unwrap()).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "hello");
}
