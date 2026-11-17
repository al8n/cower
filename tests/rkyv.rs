#![cfg(feature = "rkyv_0_8")]

#[cfg(feature = "rkyv_0_8")]
extern crate rkyv_0_8 as rkyv;

use core::mem::MaybeUninit;
use cower::{ArchivedCow, Cow, CowRepr, StaticPolicy, ToOwned};
use rkyv::{
  Serialize,
  api::low::{LowSerializer, access, from_bytes, to_bytes_in_with_alloc},
  rancor::Failure,
  ser::{allocator::SubAllocator, writer::Buffer},
  string::ArchivedString,
  util::Align,
  vec::ArchivedVec,
};

// Stack-backed output and scratch space exercise the core-only rkyv API too.
fn encode<T>(value: &T) -> (Align<[u8; 1024]>, usize)
where
  T: for<'b, 'c> Serialize<LowSerializer<Buffer<'b>, SubAllocator<'c>, Failure>>,
{
  let mut output = Align([0u8; 1024]);
  let mut scratch = [MaybeUninit::<u8>::uninit(); 1024];
  let len = to_bytes_in_with_alloc::<_, _, Failure>(
    value,
    Buffer::from(&mut *output),
    SubAllocator::new(&mut scratch),
  )
  .unwrap()
  .len();
  (output, len)
}

#[test]
fn strings_archive_contents_and_validate_without_allocating() {
  for source in [
    "",
    "🦀",
    "a string long enough to require out-of-line storage",
  ] {
    let borrowed: Cow<'_, str, StaticPolicy> = Cow::borrowed(source);
    let owned: Cow<'_, str, StaticPolicy> = Cow::owned(source);
    let (bytes, len) = encode(&borrowed);
    let (owned_bytes, owned_len) = encode(&owned);
    assert_eq!(&bytes[..len], &owned_bytes[..owned_len]);
    let archived = access::<ArchivedCow<ArchivedString>, Failure>(&bytes[..len]).unwrap();
    assert_eq!(archived.as_str(), source);
    let view: Cow<'_, str, StaticPolicy> = Cow::borrowed(archived.as_str());
    assert!(view.is_borrowed());
    assert_eq!(view, source);
  }

  let (mut bytes, len) = encode(&Cow::<'_, str>::borrowed("hello"));
  let archived = access::<ArchivedCow<ArchivedString>, Failure>(&bytes[..len]).unwrap();
  let text_offset = archived.as_str().as_ptr().addr() - bytes.as_ptr().addr();
  // 0xfe is invalid UTF-8. Avoid 0xff, rkyv's inline-string terminator.
  bytes[text_offset] = 0xfe;
  assert!(access::<ArchivedCow<ArchivedString>, Failure>(&bytes[..len]).is_err());
  assert!(access::<ArchivedCow<ArchivedString>, Failure>(&[]).is_err());
}

#[cfg(any(feature = "ecow_0_3", feature = "arcstr_1"))]
#[test]
fn shared_string_policies_deserialize_the_same_content_archive() {
  let source = "shared text".repeat(8);
  let value: Cow<'_, str, StaticPolicy> = Cow::borrowed(&source);
  let (bytes, len) = encode(&value);
  macro_rules! decode {
    ($policy:ty) => {
      let value: Cow<'_, str, $policy> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
      assert!(value.is_owned());
      assert_eq!(value.as_ref(), source);
    };
  }
  #[cfg(feature = "ecow_0_3")]
  decode!(cower::EcoPolicy);
  #[cfg(feature = "arcstr_1")]
  {
    decode!(cower::ArcStrPolicy);
    decode!(cower::ArcSubstrPolicy);
  }
}

#[cfg(all(feature = "ecow_0_3", any(feature = "std", feature = "alloc")))]
#[test]
fn eco_vectors_and_bytes_deserialize_from_slice_archives() {
  let value: Cow<'_, [u32], cower::EcoPolicy> = Cow::borrowed(&[1, 2, 3]);
  let (bytes, len) = encode(&value);
  let decoded: Cow<'_, [u32], cower::EcoPolicy> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded.as_ref(), &[1, 2, 3]);

  let value: Cow<'_, [u8], cower::EcoBytesPolicy> = Cow::borrowed(&[7; 128]);
  let (bytes, len) = encode(&value);
  let decoded: Cow<'_, [u8], cower::EcoBytesPolicy> =
    from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded.as_ref(), &[7; 128]);
}

#[cfg(feature = "smol-bytes_0_2")]
#[test]
fn smol_inline_string_archives_roundtrip_and_reject_over_capacity() {
  type Value = Cow<'static, str, cower::SmolBytesBufferPolicy>;
  let (bytes, len) = encode(&Value::borrowed("你好"));
  let decoded: Value = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded, "你好");
  let source = "x".repeat(smol_bytes_0_2::INLINE_CAP + 1);
  let value: Cow<'_, str, cower::SmolBytesBufferPolicy> = Cow::borrowed(&source);
  let (bytes, len) = encode(&value);
  let archived = access::<ArchivedCow<ArchivedString>, Failure>(&bytes[..len]).unwrap();
  assert_eq!(archived.as_str(), source);
  assert!(from_bytes::<Value, Failure>(&bytes[..len]).is_err());
}

#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
#[test]
fn smol_growable_byte_archives_deserialize_into_each_storage_strategy() {
  let (bytes, len) = encode(&Cow::<'_, [u8], cower::SmolBytesPolicy>::borrowed(
    &[7; 128],
  ));
  macro_rules! decode {
    ($policy:ty) => {
      let value: Cow<'_, [u8], $policy> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
      assert!(value.is_owned());
      assert_eq!(value.as_ref(), &[7; 128]);
    };
  }
  decode!(cower::SmolBytesPolicy);
  decode!(cower::SmolBytesCompactPolicy);
  decode!(cower::SmolBytesMutPolicy);
}

#[test]
fn slices_archive_elements_and_sized_values_support_custom_policies() {
  let source = [1u32, 2, 3];
  let value: Cow<'_, [u32], StaticPolicy> = Cow::borrowed(&source);
  let (bytes, len) = encode(&value);
  let archived =
    access::<ArchivedCow<ArchivedVec<rkyv::Archived<u32>>>, Failure>(&bytes[..len]).unwrap();
  assert_eq!(archived.len(), 3);
  assert_eq!(archived[1].to_native(), 2);

  enum Custom {}
  impl ToOwned<Custom> for u32 {
    type Owned = u32;
    type Storage<'a> = CowRepr<'a, Self, Custom>;
  }
  let value: Cow<'_, u32, Custom> = Cow::borrowed(&42);
  let (bytes, len) = encode(&value);
  let decoded: Cow<'static, u32, Custom> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(*decoded.as_ref(), 42);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn default_policy_roundtrips_in_derived_containers() {
  #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
  struct Record {
    text: Cow<'static, str>,
    numbers: Cow<'static, [u32]>,
  }
  let value = Record {
    text: Cow::borrowed("hello"),
    numbers: Cow::borrowed(&[1, 2, 3]),
  };
  let (bytes, len) = encode(&value);
  let archived = access::<ArchivedRecord, Failure>(&bytes[..len]).unwrap();
  assert_eq!(archived.text.as_str(), "hello");
  assert_eq!(archived.numbers.len(), 3);
  let decoded: Record = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.text.is_owned());
  assert!(decoded.numbers.is_owned());
  assert_eq!(decoded.text, "hello");
  assert_eq!(decoded.numbers.as_ref(), &[1, 2, 3]);
}

#[cfg(feature = "heapless_0_9")]
#[test]
fn heapless_roundtrips_and_rejects_insufficient_capacity() {
  use cower::Heapless;

  let value: Cow<'_, str, StaticPolicy> = Cow::borrowed("🦀");
  let (bytes, len) = encode(&value);
  let decoded: Cow<'static, str, Heapless<4>> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded, "🦀");
  assert!(from_bytes::<Cow<'_, str, Heapless<3>>, Failure>(&bytes[..len]).is_err());

  let value: Cow<'_, [u32], Heapless<0>> = Cow::borrowed(&[1, 2, 3]);
  let (bytes, len) = encode(&value);
  let decoded: Cow<'static, [u32], Heapless<3>> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded.as_ref(), &[1, 2, 3]);
  assert!(from_bytes::<Cow<'_, [u32], Heapless<2>>, Failure>(&bytes[..len]).is_err());

  let empty: Cow<'_, [u32], Heapless<0>> = Cow::default();
  let (bytes, len) = encode(&empty);
  let decoded: Cow<'static, [u32], Heapless<0>> = from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert!(decoded.is_empty());
}

#[cfg(feature = "heapless_0_9")]
#[test]
fn heapless_deserialization_does_not_require_clone_on_elements() {
  #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
  struct NotClone(u32);
  let source = [NotClone(42)];
  let value: Cow<'_, [NotClone], cower::Heapless<1>> = Cow::borrowed(&source);
  let (bytes, len) = encode(&value);
  let decoded: Cow<'_, [NotClone], cower::Heapless<1>> =
    from_bytes::<_, Failure>(&bytes[..len]).unwrap();
  assert_eq!(decoded[0].0, 42);
}
