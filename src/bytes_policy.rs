use crate::{Cow, CowRepr, ToOwned};
use bytes_1::{Bytes, BytesMut};

/// Growable byte storage mapping `[u8]` to [`BytesMut`].
///
/// Local borrows support infallible promotion and mutation. Consuming a unique
/// [`Bytes`] reuses its allocation; shared or static bytes are copied. Requires
/// an allocator and pointer atomics, even when cower's `alloc` tier is disabled.
#[cfg_attr(docsrs, doc(cfg(feature = "bytes_1")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BytesPolicy {}

impl ToOwned<BytesPolicy> for [u8] {
  type Owned = BytesMut;
  type Storage<'a> = CowRepr<'a, Self, BytesPolicy>;

  fn clone_into<'a>(&'a self, target: &mut BytesMut)
  where
    BytesMut: From<&'a Self>,
  {
    target.clear();
    target.extend_from_slice(self);
  }
}

impl From<BytesMut> for Cow<'_, [u8], BytesPolicy> {
  fn from(value: BytesMut) -> Self {
    Self::owned(value)
  }
}

impl From<Bytes> for Cow<'_, [u8], BytesPolicy> {
  fn from(value: Bytes) -> Self {
    Self::owned(
      value
        .try_into_mut()
        .unwrap_or_else(|shared| BytesMut::from(shared.as_ref())),
    )
  }
}

impl<'a, P: ?Sized> From<&'a Bytes> for Cow<'a, [u8], P>
where
  [u8]: ToOwned<P>,
{
  fn from(value: &'a Bytes) -> Self {
    Self::borrowed(value.as_ref())
  }
}

impl<'a, P: ?Sized> From<&'a BytesMut> for Cow<'a, [u8], P>
where
  [u8]: ToOwned<P>,
{
  fn from(value: &'a BytesMut) -> Self {
    Self::borrowed(value.as_ref())
  }
}
