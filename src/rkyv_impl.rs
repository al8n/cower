use crate::{Cow, ToOwned};
use core::{error::Error, ops::Deref};
use rkyv::{
  Archive, Deserialize, Place, Portable, Serialize, SerializeUnsized,
  munge::munge,
  rancor::{Fallible, Source},
  ser::{Allocator, Writer},
  string::{ArchivedString, StringResolver},
  vec::{ArchivedVec, VecResolver},
};

/// A policy-independent archived Cow containing only its archived contents.
///
/// Dereferences to the underlying archive: `ArchivedString` for `str`,
/// `ArchivedVec<T::Archived>` for `[T]`, and `T::Archived` for sized `T`.
/// Borrowed and owned values use the same archive format. Validation is supplied
/// by `rkyv`'s `CheckBytes`; no casts of live Cow storage are involved.
///
/// Deserialization produces owned storage and checks fallible conversions.
/// Static-reference policies cannot deserialize shorter-lived data into static
/// references. Slice deserialization uses a temporary `Vec` with `std`/`alloc`;
/// with neither, `Heapless<N>` slices deserialize directly into inline storage.
///
/// ```compile_fail
/// # use rkyv_0_8 as rkyv;
/// use cower::{Cow, StaticPolicy};
/// fn decode(bytes: &[u8]) -> Cow<'static, str, StaticPolicy> {
///     rkyv::api::low::from_bytes::<_, rkyv::rancor::Failure>(bytes).unwrap()
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "rkyv_0_8")))]
#[derive(Debug, Portable, rkyv::bytecheck::CheckBytes)]
#[bytecheck(crate = rkyv::bytecheck)]
#[repr(transparent)]
pub struct ArchivedCow<T: Portable> {
  inner: T,
}

impl<T: Portable> ArchivedCow<T> {
  /// Returns the archived contents, without deserializing or allocating.
  pub fn as_inner(&self) -> &T {
    &self.inner
  }

  fn inner_place(out: Place<Self>) -> Place<T> {
    munge!(let Self { inner } = out);
    inner
  }
}

impl<T: Portable> Deref for ArchivedCow<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.as_inner()
  }
}

impl<T: Archive + ToOwned<P>, P: ?Sized> Archive for Cow<'_, T, P> {
  type Archived = ArchivedCow<T::Archived>;
  type Resolver = T::Resolver;

  fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
    self
      .as_ref()
      .resolve(resolver, ArchivedCow::inner_place(out));
  }
}

impl<T, P, S> Serialize<S> for Cow<'_, T, P>
where
  T: Serialize<S> + ToOwned<P>,
  P: ?Sized,
  S: Fallible + ?Sized,
{
  fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
    self.as_ref().serialize(serializer)
  }
}

impl<'a, T, P, D> Deserialize<Cow<'a, T, P>, D> for ArchivedCow<T::Archived>
where
  T: Archive + ToOwned<P>,
  T::Archived: Deserialize<T, D>,
  P: ?Sized,
  T::Owned: TryFrom<T>,
  <T::Owned as TryFrom<T>>::Error: Error + Send + Sync + 'static,
  D: Fallible + ?Sized,
  D::Error: Source,
{
  fn deserialize(&self, deserializer: &mut D) -> Result<Cow<'a, T, P>, D::Error> {
    let value = self.inner.deserialize(deserializer)?;
    T::Owned::try_from(value)
      .map(Cow::owned)
      .map_err(D::Error::new)
  }
}

impl<P: ?Sized> Archive for Cow<'_, str, P>
where
  str: ToOwned<P>,
{
  type Archived = ArchivedCow<ArchivedString>;
  type Resolver = StringResolver;

  fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
    ArchivedString::resolve_from_str(self.as_ref(), resolver, ArchivedCow::inner_place(out));
  }
}

impl<P: ?Sized, S> Serialize<S> for Cow<'_, str, P>
where
  str: ToOwned<P> + SerializeUnsized<S>,
  S: Fallible + ?Sized,
  S::Error: Source,
{
  fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
    ArchivedString::serialize_from_str(self.as_ref(), serializer)
  }
}

impl<'a, P: ?Sized, D> Deserialize<Cow<'a, str, P>, D> for ArchivedCow<ArchivedString>
where
  str: ToOwned<P>,
  for<'s> <str as ToOwned<P>>::Owned: TryFrom<&'s str>,
  for<'s> <<str as ToOwned<P>>::Owned as TryFrom<&'s str>>::Error: Error + Send + Sync + 'static,
  D: Fallible + ?Sized,
  D::Error: Source,
{
  fn deserialize(&self, _: &mut D) -> Result<Cow<'a, str, P>, D::Error> {
    <str as ToOwned<P>>::Owned::try_from(self.inner.as_str())
      .map(Cow::owned)
      .map_err(D::Error::new)
  }
}

impl<T: Archive, P: ?Sized> Archive for Cow<'_, [T], P>
where
  [T]: ToOwned<P>,
{
  type Archived = ArchivedCow<ArchivedVec<T::Archived>>;
  type Resolver = VecResolver;

  fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
    ArchivedVec::resolve_from_slice(self.as_ref(), resolver, ArchivedCow::inner_place(out));
  }
}

impl<T, P: ?Sized, S> Serialize<S> for Cow<'_, [T], P>
where
  T: Serialize<S>,
  [T]: ToOwned<P>,
  S: Fallible + Allocator + Writer + ?Sized,
{
  fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
    ArchivedVec::<T::Archived>::serialize_from_slice(self.as_ref(), serializer)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, T, P: ?Sized, D> Deserialize<Cow<'a, [T], P>, D> for ArchivedCow<ArchivedVec<T::Archived>>
where
  T: Archive,
  [T]: ToOwned<P>,
  ArchivedVec<T::Archived>: Deserialize<std::vec::Vec<T>, D>,
  <[T] as ToOwned<P>>::Owned: TryFrom<std::vec::Vec<T>>,
  <<[T] as ToOwned<P>>::Owned as TryFrom<std::vec::Vec<T>>>::Error: Error + Send + Sync + 'static,
  D: Fallible + ?Sized,
  D::Error: Source,
{
  fn deserialize(&self, deserializer: &mut D) -> Result<Cow<'a, [T], P>, D::Error> {
    let values = self.inner.deserialize(deserializer)?;
    <[T] as ToOwned<P>>::Owned::try_from(values)
      .map(Cow::owned)
      .map_err(D::Error::new)
  }
}

#[cfg(all(feature = "heapless_0_9", not(any(feature = "std", feature = "alloc"))))]
impl<'a, T, D, const N: usize> Deserialize<Cow<'a, [T], crate::Heapless<N>>, D>
  for ArchivedCow<ArchivedVec<T::Archived>>
where
  T: Archive,
  T::Archived: Deserialize<T, D>,
  D: Fallible + ?Sized,
  D::Error: Source,
{
  fn deserialize(
    &self,
    deserializer: &mut D,
  ) -> Result<Cow<'a, [T], crate::Heapless<N>>, D::Error> {
    #[derive(Debug)]
    struct CapacityError;
    impl core::fmt::Display for CapacityError {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("archived slice exceeds heapless capacity")
      }
    }
    impl Error for CapacityError {}

    if self.inner.len() > N {
      return Err(D::Error::new(CapacityError));
    }
    let mut values = heapless::Vec::<T, N>::new();
    for archived in self.inner.iter() {
      values
        .push(archived.deserialize(deserializer)?)
        .map_err(|_| D::Error::new(CapacityError))?;
    }
    Ok(Cow::owned(values))
  }
}
