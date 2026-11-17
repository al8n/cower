#![cfg(any(
  feature = "bytes_1",
  feature = "smallvec_1",
  feature = "arrayvec_0_7",
  feature = "compact_str_0_9",
  feature = "smol-bytes_0_2"
))]

use cower::Cow;

#[cfg(feature = "bytes_1")]
#[test]
fn bytes_promote_locally_reuse_unique_allocations_and_preserve_shared_data() {
  use bytes_1::{Bytes, BytesMut};
  use cower::{BytesPolicy, ToOwned};
  let source = [1u8, 2, 3];
  let mut value: Cow<'_, [u8], BytesPolicy> = Cow::from(&source);
  value.to_mut()[0] = 9;
  assert_eq!(value.as_ref(), &[9, 2, 3]);
  assert_eq!(source, [1, 2, 3]);

  let mut buffer = BytesMut::with_capacity(256);
  buffer.extend_from_slice(&[7; 128]);
  let pointer = buffer.as_ptr();
  let value: Cow<'_, [u8], BytesPolicy> = buffer.freeze().into();
  assert_eq!(value.as_ptr(), pointer);
  let mut buffer = value.into_owned();
  <[u8] as ToOwned<BytesPolicy>>::clone_into(&source, &mut buffer);
  assert_eq!(buffer.as_ptr(), pointer);
  assert_eq!(buffer.as_ref(), source);

  let shared = Bytes::from(vec![3; 128]);
  let mut value: Cow<'_, [u8], BytesPolicy> = shared.clone().into();
  value.to_mut()[0] = 4;
  assert_eq!(shared[0], 3);
  assert_ne!(value.as_ptr(), shared.as_ptr());
  let borrowed: Cow<'_, [u8]> = Cow::from(&shared);
  assert_eq!(borrowed.as_ptr(), shared.as_ptr());
}

#[cfg(feature = "bytes_1")]
#[test]
fn mutable_bytes_borrow_without_copying_and_move_without_reallocating() {
  use bytes_1::{Bytes, BytesMut};
  use cower::BytesPolicy;

  let mut buffer = BytesMut::with_capacity(64);
  buffer.extend_from_slice(b"local");
  let pointer = buffer.as_ptr();
  let borrowed: Cow<'_, [u8], BytesPolicy> = Cow::from(&buffer);
  assert!(borrowed.is_borrowed());
  assert_eq!(borrowed.as_ptr(), pointer);
  let value: Cow<'_, [u8], BytesPolicy> = buffer.into();
  assert!(value.is_owned());
  assert_eq!(value.into_owned().as_ptr(), pointer);

  let source = Bytes::from_static(b"static");
  let mut value: Cow<'_, [u8], BytesPolicy> = source.clone().into();
  value.to_mut()[0] = b'S';
  assert_eq!(value.as_ref(), b"Static");
  assert_eq!(source.as_ref(), b"static");
}

#[cfg(feature = "smallvec_1")]
#[test]
fn smallvec_clones_non_copy_elements_and_spills() {
  use cower::SmallVecPolicy;
  let items = [String::from("one"), String::from("two")];
  let mut value: Cow<'_, [String], SmallVecPolicy<2>> = Cow::from(&items);
  assert!(!value.to_mut().spilled());
  value.to_mut().push(String::from("three"));
  assert!(value.to_mut().spilled());
  assert_eq!(items.len(), 2);
  let storage = value.into_owned();
  let pointer = storage.as_ptr();
  let borrowed: Cow<'_, [String], SmallVecPolicy<0>> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), pointer);
  let moved: Cow<'_, [String], SmallVecPolicy<2>> = storage.into();
  assert_eq!(moved.as_ptr(), pointer);
}

#[cfg(feature = "arrayvec_0_7")]
#[test]
fn arrayvec_capacity_is_fallible_and_counts_utf8_bytes() {
  use cower::ArrayVecPolicy;
  let mut value: Cow<'_, str, ArrayVecPolicy<3>> = Cow::borrowed("🦀");
  assert!(value.try_to_mut().is_err());
  assert_eq!(value.as_borrowed(), Some("🦀"));
  let value: Cow<'_, str, ArrayVecPolicy<4>> = Cow::borrowed("🦀");
  assert_eq!(value.try_into_owned().unwrap().as_str(), "🦀");
  let value: Cow<'_, str, ArrayVecPolicy<0>> = Cow::borrowed("");
  assert!(value.try_into_owned().unwrap().is_empty());

  let items = [String::from("local")];
  let mut value: Cow<'_, [String], ArrayVecPolicy<2>> = Cow::from(&items);
  value.try_to_mut().unwrap().push(String::from("other"));
  let storage = value.try_into_owned().unwrap();
  let mut borrowed: Cow<'_, [String], ArrayVecPolicy<1>> = Cow::from(&storage);
  assert!(borrowed.try_to_mut().is_err());
  assert_eq!(borrowed.as_borrowed().unwrap().as_ptr(), storage.as_ptr());
}

#[cfg(feature = "arrayvec_0_7")]
#[test]
fn arrayvec_container_conversions_preserve_borrows_and_move_non_clone_items() {
  use arrayvec_0_7::{ArrayString, ArrayVec};
  use cower::ArrayVecPolicy;

  let text = ArrayString::<8>::from("你好").unwrap();
  // A borrow can exceed the destination policy's capacity.
  let borrowed: Cow<'_, str, ArrayVecPolicy<0>> = Cow::from(&text);
  assert_eq!(borrowed.as_borrowed(), Some("你好"));
  assert_eq!(borrowed.as_ptr(), text.as_ptr());
  let mut value: Cow<'_, str, ArrayVecPolicy<8>> = text.into();
  assert!(value.is_owned());
  value.try_to_mut().unwrap().push('!');
  assert_eq!(value.try_into_owned().unwrap().as_str(), "你好!");

  struct NonClone(u8);
  let mut items = ArrayVec::<NonClone, 2>::new();
  items.push(NonClone(7));
  let borrowed: Cow<'_, [NonClone], ArrayVecPolicy<0>> = Cow::from(&items);
  assert!(borrowed.is_borrowed());
  assert_eq!(borrowed.as_ptr(), items.as_ptr());
  let value: Cow<'_, [NonClone], ArrayVecPolicy<2>> = items.into();
  let cower::CowRepr::Owned(mut items) = value.into_repr() else {
    panic!("moving an ArrayVec lost its owned variant");
  };
  assert_eq!(items.pop().unwrap().0, 7);
}

#[cfg(feature = "compact_str_0_9")]
#[test]
fn compact_strings_promote_and_reuse_storage() {
  use compact_str_0_9::CompactString;
  use cower::{CompactStrPolicy, ToOwned};
  let source = String::from("hello");
  let mut value: Cow<'_, str, CompactStrPolicy> = Cow::borrowed(&source);
  assert!(!value.to_mut().is_heap_allocated());
  value.to_mut().push_str(" world");
  assert_eq!(source, "hello");
  assert_eq!(value, "hello world");

  let mut storage = CompactString::with_capacity(256);
  storage.push_str(&"x".repeat(128));
  let pointer = storage.as_ptr();
  <str as ToOwned<CompactStrPolicy>>::clone_into(&"y".repeat(64), &mut storage);
  assert_eq!(storage.as_ptr(), pointer);
  let borrowed: Cow<'_, str, CompactStrPolicy> = Cow::from(&storage);
  assert!(borrowed.is_borrowed());
  assert_eq!(borrowed.as_ptr(), pointer);
  assert_eq!(borrowed.as_ref(), "y".repeat(64));
  let value: Cow<'_, str, CompactStrPolicy> = storage.into();
  assert_eq!(value.into_owned().as_ptr(), pointer);
}

#[cfg(feature = "smol-bytes_0_2")]
#[test]
fn smol_fixed_buffers_preserve_capacity_errors_and_utf8() {
  use cower::SmolBytesBufferPolicy;
  use smol_bytes_0_2::{Buffer, INLINE_CAP, Utf8Buffer};
  let source = [7; INLINE_CAP + 1];
  let mut value: Cow<'_, [u8], SmolBytesBufferPolicy> = Cow::from(&source);
  assert!(value.try_to_mut().is_err());
  assert_eq!(value.as_borrowed().unwrap().as_ptr(), source.as_ptr());
  let value: Cow<'_, [u8], SmolBytesBufferPolicy> = Cow::borrowed(&source[..INLINE_CAP]);
  assert_eq!(value.try_into_owned().unwrap().len(), INLINE_CAP);
  let storage = Buffer::try_from(&source[..2]).unwrap();
  let borrowed: Cow<'_, [u8]> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), storage.as_slice().as_ptr());
  let moved: Cow<'_, [u8], SmolBytesBufferPolicy> = storage.into();
  assert!(moved.is_owned());

  let text = "🦀".repeat(16);
  let mut value: Cow<'_, str, SmolBytesBufferPolicy> = Cow::borrowed(&text);
  assert!(value.try_to_mut().is_err());
  assert_eq!(value.as_borrowed(), Some(text.as_str()));
  let storage = Utf8Buffer::try_from("你好").unwrap();
  let borrowed: Cow<'_, str> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), storage.as_str().as_ptr());
  let moved: Cow<'_, str, SmolBytesBufferPolicy> = storage.into();
  assert_eq!(moved.try_into_owned().unwrap().as_str(), "你好");
}

#[cfg(all(feature = "smol-bytes_0_2", any(feature = "std", feature = "alloc")))]
#[test]
fn smol_shared_compact_and_mutable_storage_preserve_their_semantics() {
  use cower::{SmolBytesCompactPolicy, SmolBytesMutPolicy, SmolBytesPolicy};
  use smol_bytes_0_2::{Bytes, BytesMut, Utf8Bytes, Utf8BytesMut, compact};
  let source = vec![7; 128];
  let bytes = Bytes::from(source.as_slice());
  let pointer = bytes.as_slice().as_ptr();
  let value: Cow<'_, [u8], SmolBytesPolicy> = bytes.into();
  assert_eq!(value.clone().as_ptr(), pointer);
  assert_eq!(value.into_owned().as_slice().as_ptr(), pointer);
  let compact = compact::Bytes::from(source.as_slice());
  let value: Cow<'_, [u8], SmolBytesCompactPolicy> = compact.into();
  assert_eq!(value.into_owned().as_slice(), source);
  let mut value: Cow<'_, [u8], SmolBytesMutPolicy> = Cow::borrowed(&source);
  value.to_mut().extend_from_slice(&[9]);
  assert_eq!(value.len(), 129);
  assert_eq!(source.len(), 128);
  let bytes = BytesMut::from(source.as_slice());
  let pointer = bytes.as_slice().as_ptr();
  let value: Cow<'_, [u8], SmolBytesMutPolicy> = bytes.into();
  assert_eq!(value.into_owned().as_slice().as_ptr(), pointer);

  let source = "你好".repeat(32);
  let text = Utf8Bytes::from(source.as_str());
  let value: Cow<'_, str, SmolBytesPolicy> = text.into();
  assert_eq!(value.clone().as_ptr(), value.as_ptr());
  let text = compact::Utf8Bytes::from(source.as_str());
  let value: Cow<'_, str, SmolBytesCompactPolicy> = text.into();
  assert_eq!(value.into_owned().as_str(), source);
  let mut value: Cow<'_, str, SmolBytesMutPolicy> = Cow::borrowed(&source);
  value.to_mut().push_str("!");
  assert_eq!(value.len(), source.len() + 1);
  let text = Utf8BytesMut::from(source.as_str());
  let pointer = text.as_str().as_ptr();
  let value: Cow<'_, str, SmolBytesMutPolicy> = text.into();
  assert_eq!(value.into_owned().as_str().as_ptr(), pointer);
}
