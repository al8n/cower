#![cfg(feature = "heapless_0_9")]

use heapless_0_9 as heapless;

use core::cell::Cell;

use cower::{Cow, CowRepr, Heapless};

#[test]
fn strings_promote_local_borrows_and_preserve_the_source() {
  let source = heapless::String::<8>::try_from("hello").unwrap();
  let mut text: Cow<'_, str, Heapless<6>> = Cow::from(&source);
  assert!(core::ptr::eq(text.as_borrowed().unwrap(), source.as_str()));
  text.try_to_mut().unwrap().push('!').unwrap();
  assert_eq!(source.as_str(), "hello");
  assert_eq!(text, "hello!");
  assert!(text.is_owned());
  let owned: heapless::String<6> = text.try_into_owned().unwrap();
  assert_eq!(owned.as_str(), "hello!");

  let text: Cow<'_, str, Heapless<6>> = owned.into();
  assert!(text.is_owned());
  let default: Cow<'_, str> = Cow::from(&source);
  assert!(default.is_borrowed());
  assert_eq!(default, "hello");
}

#[test]
fn slices_can_contain_non_static_references() {
  let source = String::from("local");
  let items = [source.as_str()];
  let mut values: Cow<'_, [&str], Heapless<2>> = Cow::from(&items);
  values.try_to_mut().unwrap().push("other").unwrap();
  let owned: heapless::Vec<&str, 2> = values.try_into_owned().unwrap();
  assert_eq!(owned.as_slice(), &["local", "other"]);
  assert!(core::ptr::eq(owned[0], source.as_str()));

  let borrowed: Cow<'_, [&str], Heapless<0>> = Cow::from(&owned);
  assert!(borrowed.is_borrowed());
  assert!(borrowed.try_into_owned().is_err());
}

#[test]
fn capacity_errors_preserve_borrows_and_count_utf8_bytes() {
  let mut text: Cow<'_, str, Heapless<3>> = Cow::borrowed("🦀");
  let _: heapless::CapacityError = text.try_to_mut().unwrap_err();
  assert_eq!(text.as_borrowed(), Some("🦀"));
  let _: heapless::CapacityError = text.try_into_owned().unwrap_err();

  let exact: Cow<'_, str, Heapless<4>> = Cow::borrowed("🦀");
  assert_eq!(exact.try_into_owned().unwrap().as_str(), "🦀");
  let empty: Cow<'_, str, Heapless<0>> = Cow::borrowed("");
  assert!(empty.try_into_owned().unwrap().is_empty());

  let source = [1, 2, 3];
  let mut values: Cow<'_, [u8], Heapless<2>> = Cow::from(&source);
  let _: heapless::CapacityError = values.try_to_mut().unwrap_err();
  assert!(core::ptr::eq(
    values.as_borrowed().unwrap(),
    source.as_slice()
  ));
  let _: heapless::CapacityError = values.try_into_owned().unwrap_err();
  let empty: Cow<'_, [u8], Heapless<0>> = Cow::borrowed(&[]);
  assert!(empty.try_into_owned().unwrap().is_empty());
}

#[test]
fn slice_elements_are_cloned_only_during_initial_promotion() {
  #[derive(Debug)]
  struct Counted<'a>(&'a Cell<usize>);

  impl Clone for Counted<'_> {
    fn clone(&self) -> Self {
      self.0.set(self.0.get() + 1);
      Self(self.0)
    }
  }

  let clones = Cell::new(0);
  let source = [Counted(&clones), Counted(&clones)];
  let mut values: Cow<'_, [Counted<'_>], Heapless<2>> = Cow::from(&source);
  assert_eq!(clones.get(), 0);
  values.try_to_mut().unwrap();
  assert_eq!(clones.get(), 2);
  values.try_to_mut().unwrap();
  let owned = values.try_into_owned().unwrap();
  assert_eq!(owned.len(), 2);
  assert_eq!(clones.get(), 2);
}

#[test]
fn non_clone_elements_can_be_borrowed_and_moved_without_extra_drops() {
  #[derive(Debug)]
  struct NonClone<'a>(&'a Cell<usize>);

  impl Drop for NonClone<'_> {
    fn drop(&mut self) {
      self.0.set(self.0.get() + 1);
    }
  }

  let drops = Cell::new(0);
  let mut source = heapless::Vec::<_, 1>::new();
  source.push(NonClone(&drops)).unwrap();
  let borrowed: Cow<'_, [NonClone<'_>], Heapless<1>> = Cow::from(&source);
  assert!(borrowed.is_borrowed());
  assert_eq!(borrowed.len(), 1);

  let owned: Cow<'_, [NonClone<'_>], Heapless<1>> = source.into();
  assert!(owned.is_owned());
  let CowRepr::Owned(moved) = owned.into_storage() else {
    panic!("moving a vector must preserve owned storage");
  };
  assert_eq!(drops.get(), 0);
  drop(moved);
  assert_eq!(drops.get(), 1);
}

#[test]
fn const_storage_and_cloning_preserve_variants() {
  const TEXT: Cow<'static, str, Heapless<8>> = Cow::from_storage(CowRepr::Borrowed("hello"));
  const EMPTY: Cow<'static, str, Heapless<8>> =
    Cow::from_storage(CowRepr::Owned(heapless::String::new()));
  const BYTES: Cow<'static, [u8], Heapless<2>> =
    Cow::from_storage(CowRepr::Owned(heapless::Vec::new()));

  assert!(TEXT.clone().is_borrowed());
  assert!(EMPTY.is_owned());
  assert!(BYTES.is_owned());
  let mut text = TEXT;
  text.try_to_mut().unwrap().push('!').unwrap();
  let mut copy = text.clone();
  copy.try_to_mut().unwrap().push('?').unwrap();
  assert_eq!(text, "hello!");
  assert_eq!(copy, "hello!?");
  copy.clone_from(&TEXT);
  assert!(copy.is_borrowed());

  let empty: Cow<'_, [u8], Heapless<0>> = Cow::default();
  assert!(empty.is_owned());
  assert!(empty.is_empty());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn standard_views_borrow_inline_storage() {
  let text: Cow<'_, str, Heapless<8>> = heapless::String::try_from("hello").unwrap().into();
  let view = text.borrow_std();
  assert!(matches!(view, std::borrow::Cow::Borrowed(_)));
  assert!(core::ptr::eq(view.as_ref(), text.as_ref()));
}
