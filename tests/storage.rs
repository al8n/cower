use core::{borrow::Borrow, cmp::Ordering, hash::Hash};
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use cower::{CowRepr, CowStorage, DefaultPolicy, StaticPolicy, ToOwned};

struct Inline<const N: usize>;

impl<const N: usize> ToOwned<Inline<N>> for str {
  type Owned = heapless_0_9::String<N>;
  type Storage<'a> = CowRepr<'a, str, Inline<N>>;
}

#[test]
fn storage_accessors_preserve_variants_and_contents() {
  fn check<S, P: ?Sized>(owned: <str as ToOwned<P>>::Owned)
  where
    str: ToOwned<P>,
    S: CowStorage<'static, str, P>,
  {
    let mut borrowed = S::borrowed("borrowed");
    assert_eq!(borrowed.as_borrowed(), Some("borrowed"));
    assert!(borrowed.as_owned().is_none());
    assert!(borrowed.as_owned_mut().is_none());
    assert!(matches!(
      borrowed.into_repr(),
      CowRepr::Borrowed("borrowed")
    ));

    let mut owned = S::owned(owned);
    assert!(owned.as_borrowed().is_none());
    assert_eq!(Borrow::<str>::borrow(owned.as_owned().unwrap()), "owned");
    assert_eq!(
      Borrow::<str>::borrow(&*owned.as_owned_mut().unwrap()),
      "owned"
    );
    let CowRepr::Owned(value) = owned.into_repr() else {
      panic!("owned storage lost its variant");
    };
    assert_eq!(Borrow::<str>::borrow(&value), "owned");
  }

  check::<<str as ToOwned>::Storage<'static>, DefaultPolicy>(<str as ToOwned>::to_owned("owned"));
  check::<CowRepr<'static, str, StaticPolicy>, StaticPolicy>("owned");
}

#[test]
fn repr_static_promotion_replaces_the_reference_not_the_data() {
  let mut value: CowRepr<'_, str, StaticPolicy> = CowRepr::from("before");
  assert!(value.is_borrowed());
  assert!(!value.is_owned());
  *value.to_mut() = "after";
  assert!(!value.is_borrowed());
  assert!(value.is_owned());
  assert_eq!(*value.to_mut(), "after");
  assert_eq!(value.into_owned(), "after");

  let value: CowRepr<'_, str, StaticPolicy> = CowRepr::Borrowed("original");
  assert_eq!(value.into_owned(), "original");
}

#[test]
fn repr_fallible_promotion_preserves_failed_borrows_and_owned_edits() {
  let source = String::from("🦀!");
  let mut too_large: CowRepr<'_, str, Inline<4>> = CowRepr::from(source.as_str());
  assert!(too_large.try_to_mut().is_err());
  assert!(too_large.is_borrowed());
  assert!(core::ptr::eq(too_large.as_ref(), source.as_str()));
  assert!(too_large.try_into_owned().is_err());

  let mut exact: CowRepr<'_, str, Inline<4>> = CowRepr::Borrowed("🦀");
  assert_eq!(exact.try_to_mut().unwrap().pop(), Some('🦀'));
  exact.try_to_mut().unwrap().push_str("next").unwrap();
  assert!(exact.is_owned());
  assert_eq!(exact.try_into_owned().unwrap().as_str(), "next");

  let empty: CowRepr<'_, str, Inline<0>> = CowRepr::Borrowed("");
  assert!(empty.try_into_owned().unwrap().is_empty());
}

#[test]
fn repr_clone_from_handles_every_variant_pair_without_aliasing() {
  type Repr<'a> = CowRepr<'a, str, Inline<16>>;
  for source_owned in [false, true] {
    for target_owned in [false, true] {
      let source = if source_owned {
        Repr::Owned("source".try_into().unwrap())
      } else {
        Repr::Borrowed("source")
      };
      let mut target = if target_owned {
        Repr::Owned("old".try_into().unwrap())
      } else {
        Repr::Borrowed("old")
      };
      target.clone_from(&source);
      assert_eq!(target.is_owned(), source_owned);
      assert_eq!(target.as_ref(), "source");
      target.try_to_mut().unwrap().push('!').unwrap();
      assert_eq!(source.as_ref(), "source");
      assert_eq!(target.as_ref(), "source!");
    }
  }
}

#[test]
fn repr_content_traits_ignore_policy_and_ownership() {
  fn hash(value: &impl Hash) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
  }

  let borrowed: CowRepr<'_, str, Inline<16>> = CowRepr::Borrowed("hello");
  let owned: CowRepr<'_, str, Inline<16>> = CowRepr::Owned("hello".try_into().unwrap());
  let other: CowRepr<'_, str, StaticPolicy> = CowRepr::Owned("hello");
  assert_eq!(borrowed, owned);
  assert_eq!(owned, other);
  assert_eq!(hash(&borrowed), hash(&owned));
  assert_eq!(hash(&owned), hash(&"hello"));
  for value in [&borrowed, &owned] {
    assert_eq!(Borrow::<str>::borrow(value), "hello");
    assert_eq!(format!("{value}"), "hello");
    assert_eq!(format!("{value:?}"), "\"hello\"");
    assert_eq!(value.partial_cmp(&other), Some(Ordering::Equal));
  }
  let later: CowRepr<'_, str, Inline<16>> = CowRepr::Borrowed("world");
  assert_eq!(owned.cmp(&later), Ordering::Less);
  assert_ne!(owned, later);

  let nan: CowRepr<'_, f64, StaticPolicy> = CowRepr::Borrowed(&f64::NAN);
  let number: CowRepr<'_, f64, StaticPolicy> = CowRepr::Owned(&1.0);
  assert_ne!(nan, number);
  assert_eq!(nan.partial_cmp(&number), None);
}
