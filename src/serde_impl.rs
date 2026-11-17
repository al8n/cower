use crate::{Cow, ToOwned};
use serde_core::{Deserialize, Deserializer, Serialize, Serializer};

/// Serializes the borrowed contents without encoding the policy or variant.
#[cfg_attr(docsrs, doc(cfg(feature = "serde_1")))]
impl<T, P> Serialize for Cow<'_, T, P>
where
  T: ?Sized + ToOwned<P> + Serialize,
  P: ?Sized,
{
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    self.as_ref().serialize(serializer)
  }
}

/// Deserializes into the policy's owned type, as standard `Cow` does.
///
/// Capacity and lifetime restrictions are those of `T::Owned`. In particular,
/// a static-reference policy cannot extend the input's lifetime to `'static`.
///
/// ```compile_fail
/// # use serde_core_1 as serde_core;
/// use cower::{Cow, StaticPolicy};
/// use serde_core::{Deserialize, de::value::{BorrowedStrDeserializer, Error}};
/// fn decode(input: &str) -> Cow<'static, str, StaticPolicy> {
///     Cow::deserialize(BorrowedStrDeserializer::<Error>::new(input)).unwrap()
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "serde_1")))]
impl<'de, T, P> Deserialize<'de> for Cow<'_, T, P>
where
  T: ?Sized + ToOwned<P>,
  P: ?Sized,
  T::Owned: Deserialize<'de>,
{
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    T::Owned::deserialize(deserializer).map(Self::owned)
  }
}
