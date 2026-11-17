#![cfg(feature = "serde_1")]

#[cfg(feature = "serde_1")]
use serde_core_1 as serde_core;

use cower::{Cow, StaticPolicy, ToOwned};
use serde_core::{Deserialize, de::value::BorrowedStrDeserializer};

#[test]
fn serialization_uses_contents_without_owned_or_policy_trait_bounds() {
  enum Policy {}
  struct NotSerialize;
  impl core::borrow::Borrow<str> for NotSerialize {
    fn borrow(&self) -> &str {
      "hello"
    }
  }
  impl ToOwned<Policy> for str {
    type Owned = NotSerialize;
    type Storage<'a> = cower::CowRepr<'a, str, Policy>;
  }
  let borrowed: Cow<'_, str, Policy> = Cow::borrowed("hello");
  let owned: Cow<'_, str, Policy> = Cow::owned(NotSerialize);
  assert_eq!(serde_json::to_string(&borrowed).unwrap(), "\"hello\"");
  assert_eq!(serde_json::to_string(&owned).unwrap(), "\"hello\"");
  let static_value: Cow<'_, [u8], StaticPolicy> = Cow::owned(&[1u8, 2] as &[u8]);
  assert_eq!(serde_json::to_string(&static_value).unwrap(), "[1,2]");
}

#[test]
fn static_storage_can_deserialize_an_already_static_borrow() {
  let input = BorrowedStrDeserializer::<serde_core::de::value::Error>::new("static");
  let value = Cow::<'_, str, StaticPolicy>::deserialize(input).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "static");
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn default_deserialization_owns_escaped_strings_and_slices() {
  let text: Cow<'static, str> = serde_json::from_str("\"hello\\nworld\"").unwrap();
  assert!(text.is_owned());
  assert_eq!(text, "hello\nworld");
  let bytes: Cow<'static, [u8]> = serde_json::from_str("[1,2,3]").unwrap();
  assert!(bytes.is_owned());
  assert_eq!(bytes.as_ref(), &[1, 2, 3]);
  let value: Cow<'static, u32> = serde_json::from_str("42").unwrap();
  assert_eq!(*value.as_ref(), 42);
}

#[cfg(feature = "heapless_0_9")]
#[test]
fn heapless_deserialization_checks_capacity_and_utf8() {
  use cower::Heapless;

  let text: Cow<'static, str, Heapless<4>> = serde_json::from_str("\"🦀\"").unwrap();
  assert!(text.is_owned());
  assert_eq!(text, "🦀");
  assert_eq!(serde_json::to_string(&text).unwrap(), "\"🦀\"");
  assert!(serde_json::from_str::<Cow<'_, str, Heapless<3>>>("\"🦀\"").is_err());
  assert!(serde_json::from_str::<Cow<'_, str, Heapless<0>>>("\"\"").is_ok());

  let values: Cow<'_, [u8], Heapless<2>> = serde_json::from_str("[1,2]").unwrap();
  assert!(values.is_owned());
  assert_eq!(values.as_ref(), &[1, 2]);
  assert!(serde_json::from_str::<Cow<'_, [u8], Heapless<1>>>("[1,2]").is_err());
  assert!(serde_json::from_str::<Cow<'_, [u8], Heapless<0>>>("[]").is_ok());
}
