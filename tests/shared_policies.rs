#![cfg(any(feature = "ecow_0_3", feature = "arcstr_1"))]

use cower::Cow;

#[cfg(feature = "ecow_0_3")]
#[test]
fn eco_strings_share_heap_storage_until_mutation() {
  use cower::{EcoPolicy, ToOwned};
  use ecow_0_3::EcoString;
  let source = "hello".repeat(32);
  let mut value: Cow<'_, str, EcoPolicy> = Cow::borrowed(&source);
  value.to_mut();
  let shared = value.clone();
  assert_eq!(value.as_ptr(), shared.as_ptr());
  value.to_mut().push('!');
  assert_ne!(value.as_ptr(), shared.as_ptr());
  assert_eq!(shared.as_ref(), source);
  assert_eq!(value.len(), source.len() + 1);
  let pointer = value.as_ptr();
  let storage = value.into_owned();
  let moved: Cow<'_, str, EcoPolicy> = storage.into();
  assert_eq!(moved.as_ptr(), pointer);

  let short: Cow<'_, str, EcoPolicy> = Cow::borrowed("short");
  assert!(short.into_owned().is_inline());
  let mut storage = EcoString::with_capacity(256);
  storage.push_str(&source);
  let pointer = storage.as_ptr();
  <str as ToOwned<EcoPolicy>>::clone_into(&"x".repeat(32), &mut storage);
  assert_eq!(storage.as_ptr(), pointer);
  let borrowed: Cow<'_, str> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), pointer);
}

#[cfg(feature = "ecow_0_3")]
#[test]
fn eco_vectors_clone_elements_only_for_promotion_and_detachment() {
  use cower::EcoPolicy;
  use std::{cell::Cell, rc::Rc};
  struct Counted(Rc<Cell<usize>>);
  impl Clone for Counted {
    fn clone(&self) -> Self {
      self.0.set(self.0.get() + 1);
      Self(self.0.clone())
    }
  }
  let clones = Rc::new(Cell::new(0));
  let source = [Counted(clones.clone()), Counted(clones.clone())];
  let mut value: Cow<'_, [Counted], EcoPolicy> = Cow::from(&source);
  value.to_mut();
  assert_eq!(clones.get(), 2);
  let shared = value.clone();
  assert_eq!(clones.get(), 2);
  assert_eq!(value.as_ptr(), shared.as_ptr());
  value.to_mut().make_mut();
  assert_eq!(clones.get(), 4);
  assert_ne!(value.as_ptr(), shared.as_ptr());
  let storage = value.into_owned();
  let pointer = storage.as_slice().as_ptr();
  let borrowed: Cow<'_, [Counted], EcoPolicy> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), pointer);
  let moved: Cow<'_, [Counted], EcoPolicy> = storage.into();
  assert_eq!(moved.as_ptr(), pointer);

  struct NotClone;
  let storage = ecow_0_3::EcoVec::<NotClone>::new();
  let moved: Cow<'_, [NotClone], EcoPolicy> = storage.into();
  assert!(moved.is_empty());
}

#[cfg(feature = "ecow_0_3")]
#[test]
fn eco_bytes_share_heap_storage_and_preserve_source_on_edits() {
  use cower::{EcoBytesPolicy, ToOwned};
  use ecow_0_3::EcoBytes;
  let source = [7u8; 128];
  let mut value: Cow<'_, [u8], EcoBytesPolicy> = Cow::borrowed(&source);
  value.to_mut();
  let shared = value.clone();
  assert_eq!(value.as_ptr(), shared.as_ptr());
  value.to_mut().make_mut()[0] = 9;
  assert_eq!(shared[0], 7);
  assert_eq!(source[0], 7);
  let storage = value.into_owned();
  let pointer = storage.as_slice().as_ptr();
  let borrowed: Cow<'_, [u8]> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), pointer);
  let moved: Cow<'_, [u8], EcoBytesPolicy> = storage.into();
  assert_eq!(moved.as_ptr(), pointer);
  let mut storage = EcoBytes::with_capacity(256);
  storage.extend_from_slice(&source);
  let pointer = storage.as_slice().as_ptr();
  <[u8] as ToOwned<EcoBytesPolicy>>::clone_into(&source[..32], &mut storage);
  assert_eq!(storage.as_slice().as_ptr(), pointer);
}

#[cfg(feature = "arcstr_1")]
#[test]
fn arc_strings_keep_static_literals_and_share_owned_allocations() {
  use arcstr_1::ArcStr;
  use cower::ArcStrPolicy;
  let literal = arcstr_1::literal!("hello");
  let value: Cow<'_, str, ArcStrPolicy> = literal.into();
  let copy = value.clone().into_owned();
  assert_eq!(ArcStr::as_static(&copy), Some("hello"));
  assert_eq!(copy.as_ptr(), value.as_ptr());

  let source = String::from("local input");
  let value: Cow<'_, str, ArcStrPolicy> = Cow::borrowed(&source);
  let storage = value.into_owned();
  assert_eq!(storage.as_str(), source);
  assert!(ArcStr::as_static(&storage).is_none());
  let pointer = storage.as_ptr();
  let borrowed: Cow<'_, str> = Cow::from(&storage);
  assert_eq!(borrowed.as_ptr(), pointer);
  let value: Cow<'_, str, ArcStrPolicy> = storage.into();
  let shared = value.clone();
  assert!(ArcStr::ptr_eq(
    value.as_owned().unwrap(),
    shared.as_owned().unwrap()
  ));
  assert_eq!(value.into_owned().as_ptr(), pointer);
}

#[cfg(feature = "arcstr_1")]
#[test]
fn substrings_retain_the_parent_range_and_survive_parent_drop() {
  use arcstr_1::ArcStr;
  use cower::ArcSubstrPolicy;
  let parent = ArcStr::from("prefix:payload:suffix");
  let substring = parent.substr(7..14);
  let pointer = substring.as_ptr();
  let borrowed: Cow<'_, str> = Cow::from(&substring);
  assert_eq!(borrowed.as_ptr(), pointer);
  let value: Cow<'_, str, ArcSubstrPolicy> = substring.into();
  assert_eq!(value.as_ptr(), pointer);
  assert!(ArcStr::ptr_eq(value.as_owned().unwrap().parent(), &parent));
  let shared = value.clone();
  drop(parent);
  drop(value);
  assert_eq!(shared, "payload");
  assert_eq!(shared.into_owned().as_ptr(), pointer);

  let local = String::from("copied into a new parent");
  let value: Cow<'_, str, ArcSubstrPolicy> = Cow::borrowed(&local);
  assert_eq!(value.into_owned().as_str(), local);
}
