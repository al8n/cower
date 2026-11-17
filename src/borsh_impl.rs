use crate::{Cow, ToOwned};
use borsh_1::{BorshDeserialize, BorshSerialize, io};

/// Encodes the borrowed contents without a policy or variant tag.
#[cfg_attr(docsrs, doc(cfg(feature = "borsh_1")))]
impl<T: ?Sized + ToOwned<P> + BorshSerialize, P: ?Sized> BorshSerialize for Cow<'_, T, P> {
  fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
    self.as_ref().serialize(writer)
  }
}

/// Decodes into owned storage, using its validation and capacity checks.
#[cfg_attr(docsrs, doc(cfg(feature = "borsh_1")))]
impl<T: ?Sized + ToOwned<P>, P: ?Sized> BorshDeserialize for Cow<'_, T, P>
where
  T::Owned: BorshDeserialize,
{
  fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
    T::Owned::deserialize_reader(reader).map(Self::owned)
  }
}
