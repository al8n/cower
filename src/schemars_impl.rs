extern crate alloc;

use crate::{Cow, ToOwned};
use alloc::borrow::Cow as Name;
use schemars_1::{JsonSchema, Schema, SchemaGenerator};

/// Uses the borrowed type's schema, independent of storage and capacity.
#[cfg_attr(docsrs, doc(cfg(feature = "schemars_1")))]
impl<T: ?Sized + ToOwned<P> + JsonSchema, P: ?Sized> JsonSchema for Cow<'_, T, P> {
  fn inline_schema() -> bool {
    T::inline_schema()
  }

  fn schema_name() -> Name<'static, str> {
    T::schema_name()
  }

  fn schema_id() -> Name<'static, str> {
    T::schema_id()
  }

  fn json_schema(generator: &mut SchemaGenerator) -> Schema {
    T::json_schema(generator)
  }

  fn _schemars_private_non_optional_json_schema(generator: &mut SchemaGenerator) -> Schema {
    T::_schemars_private_non_optional_json_schema(generator)
  }

  fn _schemars_private_is_option() -> bool {
    T::_schemars_private_is_option()
  }
}
