#![cfg(any(
  feature = "serde_1",
  feature = "quickcheck_1",
  feature = "proptest_1",
  feature = "arbitrary_1"
))]

#[cfg(feature = "serde_1")]
fn json_roundtrip<T, P: ?Sized>(value: &cower::Cow<'_, T, P>)
where
  T: ?Sized + cower::ToOwned<P> + serde_core_1::Serialize + PartialEq + core::fmt::Debug,
  T::Owned: for<'de> serde_core_1::Deserialize<'de>,
{
  let json = serde_json::to_string(value).unwrap();
  let decoded: cower::Cow<'_, T, P> = serde_json::from_str(&json).unwrap();
  assert!(decoded.is_owned());
  assert_eq!(decoded.as_ref(), value.as_ref());
}

#[cfg(feature = "serde_1")]
#[test]
fn serde_forwarding_supports_enabled_storage_types() {
  #[cfg(feature = "ecow_0_3")]
  {
    json_roundtrip(&cower::Cow::<'_, str, cower::EcoPolicy>::borrowed("你好"));
    json_roundtrip(&cower::Cow::<'_, [u16], cower::EcoPolicy>::borrowed(&[
      1, 2, 3,
    ]));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::EcoBytesPolicy>::borrowed(&[
      1, 2, 3,
    ]));
  }
  #[cfg(feature = "arcstr_1")]
  {
    json_roundtrip(&cower::Cow::<'_, str, cower::ArcStrPolicy>::borrowed(
      "你好",
    ));
    json_roundtrip(&cower::Cow::<'_, str, cower::ArcSubstrPolicy>::borrowed(
      "你好",
    ));
  }
  #[cfg(feature = "arrayvec_0_7")]
  {
    json_roundtrip(&cower::Cow::<'_, str, cower::ArrayVecPolicy<8>>::borrowed(
      "你好",
    ));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::ArrayVecPolicy<8>>::borrowed(
      &[1, 2],
    ));
    assert!(
      serde_json::from_str::<cower::Cow<'_, str, cower::ArrayVecPolicy<2>>>("\"你好\"").is_err()
    );
  }
  #[cfg(feature = "bytes_1")]
  json_roundtrip(&cower::Cow::<'_, [u8], cower::BytesPolicy>::borrowed(&[
    1, 2,
  ]));
  #[cfg(feature = "smallvec_1")]
  json_roundtrip(&cower::Cow::<'_, [u8], cower::SmallVecPolicy<1>>::borrowed(
    &[1, 2],
  ));
  #[cfg(feature = "compact_str_0_9")]
  json_roundtrip(&cower::Cow::<'_, str, cower::CompactStrPolicy>::borrowed(
    "你好",
  ));
  #[cfg(feature = "smol-bytes_0_2")]
  {
    json_roundtrip(&cower::Cow::<'_, str, cower::SmolBytesBufferPolicy>::borrowed("你好"));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::SmolBytesBufferPolicy>::borrowed(&[1, 2]));
    assert!(
      serde_json::from_str::<cower::Cow<'_, str, cower::SmolBytesBufferPolicy>>(&format!(
        "\"{}\"",
        "x".repeat(63)
      ))
      .is_err()
    );
  }
  #[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
  {
    json_roundtrip(&cower::Cow::<'_, str, cower::SmolBytesPolicy>::borrowed(
      "hello",
    ));
    json_roundtrip(&cower::Cow::<'_, str, cower::SmolBytesCompactPolicy>::borrowed("hello"));
    json_roundtrip(&cower::Cow::<'_, str, cower::SmolBytesMutPolicy>::borrowed(
      "hello",
    ));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::SmolBytesPolicy>::borrowed(
      &[1, 2],
    ));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::SmolBytesCompactPolicy>::borrowed(&[1, 2]));
    json_roundtrip(&cower::Cow::<'_, [u8], cower::SmolBytesMutPolicy>::borrowed(&[1, 2]));
  }
  // Keep this helper exercised even when none of the storage integrations is enabled.
  enum Policy {}
  impl cower::ToOwned<Policy> for u8 {
    type Owned = u8;
    type Storage<'a> = cower::CowRepr<'a, Self, Policy>;
  }
  json_roundtrip(&cower::Cow::<'_, u8, Policy>::owned(42));
}

#[cfg(all(feature = "quickcheck_1", feature = "smol-bytes_0_2"))]
#[test]
fn smol_quickcheck_generates_bounded_owned_values_and_shrinks() {
  use quickcheck_1::{Arbitrary, Gen};
  type Value = cower::Cow<'static, str, cower::SmolBytesBufferPolicy>;
  for _ in 0..32 {
    let value = Value::arbitrary(&mut Gen::new(100));
    assert!(value.is_owned());
    assert!(value.len() <= smol_bytes_0_2::INLINE_CAP);
    for smaller in value.shrink().take(8) {
      assert!(smaller.is_owned());
      assert!(smaller.len() <= smol_bytes_0_2::INLINE_CAP);
    }
  }
}

#[cfg(all(feature = "proptest_1", feature = "compact_str_0_9"))]
#[test]
fn compact_strings_expose_the_native_strategy() {
  use proptest_1::{
    arbitrary::any,
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
  };
  type Value = cower::Cow<'static, str, cower::CompactStrPolicy>;
  let mut runner = TestRunner::deterministic();
  let tree = any::<Value>().new_tree(&mut runner).unwrap();
  assert!(tree.current().is_owned());
}

#[cfg(all(feature = "arbitrary_1", feature = "smol-bytes_0_2"))]
#[test]
fn smol_fuzzing_uses_native_owned_storage() {
  use arbitrary_1::{Arbitrary, Unstructured};
  type Value = cower::Cow<'static, [u8], cower::SmolBytesBufferPolicy>;
  let input = [3u8; 512];
  let value = Value::arbitrary(&mut Unstructured::new(&input)).unwrap();
  assert!(value.is_owned());
  assert!(value.len() <= smol_bytes_0_2::INLINE_CAP);
}

#[cfg(all(feature = "arbitrary_1", feature = "smallvec_1"))]
#[test]
fn smallvec_fuzzing_uses_native_owned_storage() {
  use arbitrary_1::{Arbitrary, Unstructured};
  type Value = cower::Cow<'static, [u8], cower::SmallVecPolicy<2>>;
  let input = [1u8; 64];
  let value = Value::arbitrary(&mut Unstructured::new(&input)).unwrap();
  assert!(value.is_owned());
}
