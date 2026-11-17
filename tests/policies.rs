use heapless_0_9 as heapless;

use core::{borrow::Borrow, cmp::Ordering, hash::Hash};
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use cower::{Cow, StaticPolicy, ToOwned};

// Deliberately no Clone / Copy / Debug / comparison implementations on the policy.
struct Inline<const N: usize>;

impl<const N: usize> ToOwned<Inline<N>> for str {
  type Owned = heapless::String<N>;
  type Storage<'a>
    = cower::CowRepr<'a, Self, Inline<N>>
  where
    Self: 'a;
}

impl<T, const N: usize> ToOwned<Inline<N>> for [T] {
  type Owned = heapless::Vec<T, N>;
  type Storage<'a>
    = cower::CowRepr<'a, Self, Inline<N>>
  where
    Self: 'a;
}

#[test]
fn downstream_string_policy_handles_local_borrows_and_mutation() {
  let original = String::from("hello");
  let mut text: Cow<'_, str, Inline<16>> = Cow::borrowed(&original);
  assert!(text.is_borrowed());
  text.try_to_mut().unwrap().push('!').unwrap();
  assert!(text.is_owned());
  assert_eq!(text.as_ref(), "hello!");
  assert_eq!(original, "hello");
  assert_eq!(text.try_into_owned().unwrap().as_str(), "hello!");
}

#[test]
fn downstream_slice_policy_handles_local_borrows() {
  let original = [1, 2, 3];
  let mut values: Cow<'_, [u8], Inline<4>> = Cow::borrowed(&original);
  values.try_to_mut().unwrap().push(4).unwrap();
  assert_eq!(values.as_ref(), &[1, 2, 3, 4]);
  assert_eq!(original, [1, 2, 3]);
  assert_eq!(values.try_into_owned().unwrap().as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn capacity_failures_preserve_the_original_borrow() {
  let mut text: Cow<'_, str, Inline<4>> = Cow::borrowed("hello");
  assert!(text.try_to_mut().is_err());
  assert!(text.is_borrowed());
  assert_eq!(text.as_ref(), "hello");
  assert!(text.try_into_owned().is_err());

  let mut values: Cow<'_, [u8], Inline<2>> = Cow::borrowed(&[1, 2, 3]);
  assert!(values.try_to_mut().is_err());
  assert!(values.is_borrowed());
  assert_eq!(values.as_ref(), &[1, 2, 3]);
  assert!(values.try_into_owned().is_err());
}

#[test]
fn exact_and_zero_capacity_and_utf8_are_respected() {
  let empty: Cow<'_, str, Inline<0>> = Cow::borrowed("");
  assert!(empty.try_into_owned().unwrap().is_empty());
  let exact: Cow<'_, str, Inline<4>> = Cow::borrowed("🦀");
  assert_eq!(exact.try_into_owned().unwrap().as_str(), "🦀");
  let too_small: Cow<'_, str, Inline<3>> = Cow::borrowed("🦀");
  assert!(too_small.try_into_owned().is_err());
}

#[test]
fn traits_use_content_across_variants_and_policies() {
  fn hash(value: &impl Hash) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
  }

  let borrowed: Cow<'_, str> = Cow::borrowed("hello");
  let owned: Cow<'_, str, Inline<16>> = Cow::owned("hello".try_into().unwrap());
  let static_owned: Cow<'_, str, StaticPolicy> = Cow::owned("hello");
  assert_eq!(borrowed, owned);
  assert_eq!(owned, static_owned);
  assert_eq!(hash(&borrowed), hash(&owned));
  assert_eq!(hash(&owned), hash(&static_owned));
  assert_eq!(hash(&owned), hash(&"hello"));
  assert_eq!(owned.as_ref(), "hello");
  assert_eq!(Borrow::<str>::borrow(&owned), "hello");
  assert_eq!(format!("{owned}"), "hello");
  assert_eq!(format!("{owned:?}"), "\"hello\"");
  assert_eq!(borrowed.partial_cmp(&owned), Some(Ordering::Equal));

  let later: Cow<'_, str, Inline<16>> = Cow::borrowed("world");
  assert!(borrowed < later);
  assert_eq!(owned.cmp(&later), Ordering::Less);
}

#[test]
fn cloning_preserves_variants_and_independence() {
  let borrowed: Cow<'_, str, Inline<16>> = Cow::borrowed("hello");
  assert!(borrowed.clone().is_borrowed());
  let mut owned = borrowed.clone();
  owned.try_to_mut().unwrap().push('!').unwrap();
  let mut copy = owned.clone();
  copy.try_to_mut().unwrap().push('?').unwrap();
  assert_eq!(owned.as_ref(), "hello!");
  assert_eq!(copy.as_ref(), "hello!?");

  copy.clone_from(&owned);
  assert_eq!(copy, owned);
  assert!(copy.is_owned());
  copy.clone_from(&borrowed);
  assert!(copy.is_borrowed());
  copy.clone_from(&owned);
  assert!(copy.is_owned());
}

#[test]
fn owned_storage_skips_fallible_conversion() {
  // A conversion that always fails makes accidental reconversion observable.
  struct Reject;
  struct Storage(u32);
  impl ToOwned<Reject> for u32 {
    type Owned = Storage;
    type Storage<'a>
      = cower::CowRepr<'a, Self, Reject>
    where
      Self: 'a;
  }
  impl Borrow<u32> for Storage {
    fn borrow(&self) -> &u32 {
      &self.0
    }
  }
  impl TryFrom<&u32> for Storage {
    type Error = ();
    fn try_from(_: &u32) -> Result<Self, ()> {
      Err(())
    }
  }

  let mut owned: Cow<'_, u32, Reject> = Cow::owned(Storage(42));
  owned.try_to_mut().unwrap().0 += 1;
  assert_eq!(owned.try_into_owned().unwrap().0, 43);
  let mut borrowed: Cow<'_, u32, Reject> = Cow::borrowed(&42);
  assert!(borrowed.try_to_mut().is_err());
  assert!(borrowed.is_borrowed());
}

#[test]
fn policies_can_be_unsized() {
  trait Policy {}
  impl ToOwned<dyn Policy> for str {
    type Owned = &'static str;
    type Storage<'a>
      = cower::CowRepr<'a, Self, dyn Policy>
    where
      Self: 'a;
  }
  fn assert_copy<T: Copy>() {}
  assert_copy::<Cow<'_, str, dyn Policy>>();
  let value: Cow<'_, str, dyn Policy> = Cow::borrowed("hello");
  assert_eq!(value.into_owned(), "hello");
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn default_policy_has_no_no_alloc_blanket_impl() {
  // A blanket implementation in cower would conflict with this downstream impl.
  #[derive(Clone)]
  struct Local(u8);
  impl ToOwned for Local {
    type Owned = Self;
    type Storage<'a>
      = cower::CowRepr<'a, Self>
    where
      Self: 'a;
  }
  impl From<&Local> for Local {
    fn from(value: &Local) -> Self {
      Self(value.0)
    }
  }

  let local = Local(42);
  let value: Cow<'_, Local> = Cow::borrowed(&local);
  assert_eq!(value.into_owned().0, 42);
}
