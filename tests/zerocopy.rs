#![cfg(feature = "zerocopy_0_8")]

use cower::{Cow, CowRepr, StaticPolicy, ToOwned};
use zerocopy_0_8::{ConvertError, Immutable, IntoBytes, KnownLayout};

#[test]
fn checked_borrows_preserve_identity_and_validate_bit_patterns() {
  let source = [1u8];
  let value: Cow<'_, bool, StaticPolicy> = Cow::try_from_bytes(&source).unwrap();
  assert!(value.is_borrowed());
  assert!(*value.as_ref());
  assert_eq!(value.as_ref().as_bytes().as_ptr(), source.as_ptr());
  assert!(matches!(
    Cow::<'_, bool, StaticPolicy>::try_from_bytes(&[2]),
    Err(ConvertError::Validity(_))
  ));
  assert!(matches!(
    Cow::<'_, bool, StaticPolicy>::try_from_bytes(&[]),
    Err(ConvertError::Size(_))
  ));

  let text = "你好".as_bytes();
  let value: Cow<'_, str> = Cow::try_from_bytes(text).unwrap();
  assert_eq!(value, "你好");
  assert_eq!(value.as_bytes().as_ptr(), text.as_ptr());
  assert!(Cow::<'_, str>::try_from_bytes(&[0xff]).is_err());
}

#[test]
fn alignment_errors_and_native_byte_order_are_preserved() {
  #[repr(align(8))]
  struct Aligned([u8; 8]);
  let source = Aligned([1, 2, 3, 4, 5, 6, 7, 8]);
  let value: Cow<'_, u32, StaticPolicy> = Cow::try_from_bytes(&source.0[..4]).unwrap();
  assert_eq!(*value.as_ref(), u32::from_ne_bytes([1, 2, 3, 4]));
  assert_eq!(value.as_ref().as_bytes(), &source.0[..4]);
  assert!(matches!(
    Cow::<'_, u32, StaticPolicy>::try_from_bytes(&source.0[1..5]),
    Err(ConvertError::Alignment(_))
  ));
}

#[test]
fn prefix_suffix_and_explicit_element_counts_keep_the_remainder() {
  let source = [1, 0, 9, 1];
  let (first, rest) = Cow::<'_, bool, StaticPolicy>::try_from_prefix(&source).unwrap();
  assert!(*first.as_ref());
  assert_eq!(rest, &source[1..]);
  let (rest, last) = Cow::<'_, bool, StaticPolicy>::try_from_suffix(&source).unwrap();
  assert_eq!(rest, &source[..3]);
  assert!(*last.as_ref());
  let (first, rest) = Cow::<'_, [u8]>::try_from_prefix_with_elems(&source, 2).unwrap();
  assert_eq!(first.as_ref(), &[1, 0]);
  assert_eq!(rest.as_ptr(), source[2..].as_ptr());
  let (rest, last) = Cow::<'_, [u8]>::try_from_suffix_with_elems(&source, 2).unwrap();
  assert_eq!(rest, &[1, 0]);
  assert_eq!(last.as_ref(), &[9, 1]);
  let whole: Cow<'_, [u8]> = Cow::try_from_bytes_with_elems(&source, 4).unwrap();
  assert_eq!(whole.as_ptr(), source.as_ptr());
  assert!(Cow::<'_, [u8]>::try_from_bytes_with_elems(&source, 3).is_err());
  assert!(Cow::<'_, [u8]>::try_from_prefix_with_elems(&source, 5).is_err());
  assert!(Cow::<'_, [u8]>::try_from_suffix_with_elems(&source, usize::MAX).is_err());
  let empty: Cow<'_, [u8]> = Cow::try_from_bytes(&[]).unwrap();
  assert!(empty.is_empty());
}

#[test]
fn explicit_counts_support_zero_sized_slice_elements() {
  let value: Cow<'_, [()], StaticPolicy> = Cow::try_from_bytes_with_elems(&[], 123).unwrap();
  assert_eq!(value.len(), 123);
  assert!(value.as_ref().as_bytes().is_empty());
  let source = [9u8, 8];
  let (value, rest) =
    Cow::<'_, [()], StaticPolicy>::try_from_prefix_with_elems(&source, 7).unwrap();
  assert_eq!(value.len(), 7);
  assert_eq!(rest, source);
  let (rest, value) =
    Cow::<'_, [()], StaticPolicy>::try_from_suffix_with_elems(&source, 7).unwrap();
  assert_eq!(rest, source);
  assert_eq!(value.len(), 7);
}

#[test]
fn fallible_mutation_promotes_and_preserves_borrows_on_error() {
  enum Inline {}
  impl ToOwned<Inline> for [u16] {
    type Owned = [u16; 2];
    type Storage<'a> = CowRepr<'a, Self, Inline>;
  }
  let source = [1u16, 2];
  let mut value: Cow<'_, [u16], Inline> = Cow::try_from_bytes(source.as_bytes()).unwrap();
  value.try_to_mut_bytes().unwrap()[..2].copy_from_slice(&42u16.to_ne_bytes());
  assert!(value.is_owned());
  assert_eq!(value.as_ref(), &[42, 2]);
  assert_eq!(source, [1, 2]);

  let source = [1u16, 2, 3];
  let mut value: Cow<'_, [u16], Inline> = Cow::try_from_bytes(source.as_bytes()).unwrap();
  assert!(value.try_to_mut_bytes().is_err());
  assert_eq!(value.as_borrowed().unwrap().as_ptr(), source.as_ptr());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn infallible_mutation_copies_only_on_initial_promotion() {
  let source = [1u32, 2];
  let mut value: Cow<'_, [u32]> = Cow::try_from_bytes(source.as_bytes()).unwrap();
  value.to_mut_bytes()[..4].copy_from_slice(&42u32.to_ne_bytes());
  assert_eq!(value.as_ref(), &[42, 2]);
  assert_eq!(source, [1, 2]);
  let pointer = value.as_ptr();
  value.to_mut_bytes()[4..].copy_from_slice(&99u32.to_ne_bytes());
  assert_eq!(value.as_ptr(), pointer);
  assert_eq!(value.as_ref(), &[42, 99]);
}

#[test]
fn layout_and_immutability_derive_from_storage_not_policy_markers() {
  fn layout<T: KnownLayout>() {}
  fn immutable<T: Immutable>() {}
  layout::<Cow<'_, str>>();
  layout::<CowRepr<'_, str, StaticPolicy>>();
  immutable::<Cow<'_, str, StaticPolicy>>();

  // The policy itself may contain UnsafeCell; no policy value is stored.
  struct Marker(core::cell::UnsafeCell<()>);
  impl ToOwned<Marker> for str {
    type Owned = &'static str;
    type Storage<'a> = CowRepr<'a, Self, Marker>;
  }
  immutable::<Cow<'_, str, Marker>>();
}
