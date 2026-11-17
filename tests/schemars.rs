#![cfg(feature = "schemars_1")]

use cower::{Cow, StaticPolicy, ToOwned};
use schemars_1::{JsonSchema, SchemaGenerator, schema_for};

#[test]
fn schemas_match_contents_and_do_not_require_owned_or_policy_traits() {
  enum Policy {}
  struct Storage;
  impl core::borrow::Borrow<str> for Storage {
    fn borrow(&self) -> &str {
      ""
    }
  }
  impl ToOwned<Policy> for str {
    type Owned = Storage;
    type Storage<'a> = cower::CowRepr<'a, Self, Policy>;
  }
  type Value = Cow<'static, str, Policy>;
  assert_eq!(schema_for!(Value), schema_for!(str));
  assert_eq!(Value::schema_name(), str::schema_name());
  assert_eq!(Value::schema_id(), str::schema_id());
  assert_eq!(Value::inline_schema(), str::inline_schema());
  assert_eq!(
    schema_for!(Cow<'_, [u16], StaticPolicy>),
    schema_for!([u16])
  );
}

#[test]
fn option_hooks_forward_to_the_wrapped_type() {
  type Value = Cow<'static, Option<u16>, StaticPolicy>;
  assert!(Value::_schemars_private_is_option());
  assert_eq!(
    Value::_schemars_private_non_optional_json_schema(&mut SchemaGenerator::default()),
    Option::<u16>::_schemars_private_non_optional_json_schema(&mut SchemaGenerator::default()),
  );
  assert_eq!(schema_for!(Value), schema_for!(Option<u16>));
}

#[cfg(feature = "arrayvec_0_7")]
#[test]
fn schema_does_not_limit_borrows_to_the_owned_capacity() {
  type Value = Cow<'static, str, cower::ArrayVecPolicy<1>>;
  let value: Value = Cow::borrowed("longer than capacity");
  assert!(value.try_into_owned().is_err());
  assert_eq!(schema_for!(Value), schema_for!(str));
}
