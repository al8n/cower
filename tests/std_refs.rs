#![cfg(any(feature = "std", feature = "alloc"))]

use heapless_0_9 as heapless;

use cower::{Cow, CowRepr, StaticPolicy, ToOwned};
use std::{borrow::Cow as StdCow, cell::Cell, rc::Rc};

static STATIC: Cow<'static, str> = Cow::Borrowed("constant");
const STD_REF: &StdCow<'static, str> = STATIC.as_std();
const BORROWED: StdCow<'static, str> = Cow::<'static, str>::Borrowed("constant").into_std();
const OWNED: Cow<'static, str> =
  Cow::from_std(Cow::<'static, str>::Owned(String::new()).into_std());
const SLICE: StdCow<'static, [u8]> = Cow::<'static, [u8]>::Owned(Vec::new()).into_std();
const CUSTOM: Cow<'static, str, StaticPolicy> = Cow::from_storage(CowRepr::Borrowed("static"));

const fn as_mut_std<'a, 'b>(value: &'b mut Cow<'a, str>) -> &'b mut StdCow<'a, str> {
  value.as_std_mut()
}

#[test]
fn const_values_and_references_work() {
  assert_eq!(STD_REF, "constant");
  assert!(core::ptr::eq(STD_REF, STATIC.as_std()));
  assert_eq!(BORROWED, "constant");
  assert!(OWNED.is_owned());
  assert!(OWNED.is_empty());
  assert!(matches!(SLICE, StdCow::Owned(_)));
  assert_eq!(CUSTOM, "static");
}

#[test]
fn deref_coercion_passes_the_actual_standard_field() {
  fn accepts_standard(value: &StdCow<'_, str>) -> *const () {
    (value as *const StdCow<'_, str>).cast()
  }
  fn mutates_standard(value: &mut StdCow<'_, str>) {
    value.to_mut().push('!');
  }
  let local = String::from("local");
  let mut value: Cow<'_, str> = Cow::Borrowed(&local);
  let reference: &StdCow<'_, str> = &value;
  assert!(core::ptr::eq(reference, value.as_std()));
  assert_eq!(
    accepts_standard(&value),
    (value.as_std() as *const StdCow<'_, str>).cast::<()>()
  );
  let reference: &StdCow<'_, str> = (&value).into();
  assert!(core::ptr::eq(reference, value.as_std()));
  mutates_standard(&mut value);
  assert_eq!(value, "local!");
  assert_eq!(local, "local");
  as_mut_std(&mut value).to_mut().push('?');
  let reference: &mut StdCow<'_, str> = (&mut value).into();
  reference.to_mut().push('.');
  assert_eq!(value, "local!?.");
}

#[test]
fn owned_round_trips_preserve_allocation_and_capacity() {
  let mut source = String::with_capacity(64);
  source.push_str("hello");
  let pointer = source.as_ptr();
  let capacity = source.capacity();
  let value: Cow<'_, str> = Cow::from_std(StdCow::Owned(source));
  let value = value.into_std();
  assert!(matches!(value, StdCow::Owned(_)));
  let value: Cow<'_, str> = value.into();
  let source = value.into_owned();
  assert_eq!(source.as_ptr(), pointer);
  assert_eq!(source.capacity(), capacity);
}

#[test]
fn converting_a_reference_preserves_the_variant_and_source_lifetime() {
  fn convert<'a>(value: &Cow<'a, str>) -> StdCow<'a, str> {
    value.into()
  }

  let source = String::from("local");
  let borrowed = {
    let value = Cow::<str>::from_ref(&source);
    convert(&value)
  };
  assert!(matches!(borrowed, StdCow::Borrowed(s) if core::ptr::eq(s, source.as_str())));

  let mut value = Cow::<[u8]>::from(vec![1, 2]);
  let standard: StdCow<'static, [u8]> = (&value).into();
  let wrapped: Cow<'static, [u8]> = (&value).into();
  value.to_mut()[0] = 3;
  assert!(matches!(standard, StdCow::Owned(ref v) if v == &[1, 2]));
  assert!(wrapped.is_owned());
  assert_eq!(wrapped.as_slice(), &[1, 2]);
}

#[test]
fn custom_policy_can_explicitly_choose_standard_backing() {
  struct Heap;
  impl ToOwned<Heap> for str {
    type Owned = String;
    type Storage<'a> = StdCow<'a, str>;
  }
  fn accepts<'a>(value: &'a StdCow<'_, str>) -> &'a str {
    value
  }
  let value: Cow<'_, str, Heap> = Cow::borrowed("heap");
  assert_eq!(accepts(&value), "heap");
  assert!(core::ptr::eq(value.as_std(), &*value));
}

#[test]
fn custom_storage_uses_borrowed_views_and_value_conversion() {
  struct Inline;
  impl ToOwned<Inline> for str {
    type Owned = heapless::String<16>;
    type Storage<'a> = CowRepr<'a, str, Inline>;
  }
  let mut value: Cow<'_, str, Inline> = Cow::borrowed("inline");
  value.try_to_mut().unwrap().push('!').unwrap();
  assert_eq!(value.borrow_std(), "inline!");

  struct HeapEnum;
  impl ToOwned<HeapEnum> for str {
    type Owned = String;
    type Storage<'a> = CowRepr<'a, str, HeapEnum>;
  }
  let value: Cow<'_, str, HeapEnum> = Cow::owned(String::from("enum"));
  let standard: StdCow<'_, str> = value.into();
  assert!(matches!(standard, StdCow::Owned(_)));
  let value: Cow<'_, str, HeapEnum> = standard.into();
  assert_eq!(value, "enum");
}

// Owned deliberately implements neither Clone nor From<&View>.
#[repr(align(64))]
struct View {
  drops: Rc<Cell<usize>>,
  value: u32,
}
struct Owned(View);
impl core::borrow::Borrow<View> for Owned {
  fn borrow(&self) -> &View {
    &self.0
  }
}
impl Drop for Owned {
  fn drop(&mut self) {
    self.0.drops.set(self.0.drops.get() + 1);
  }
}
impl std::borrow::ToOwned for View {
  type Owned = Owned;
  fn to_owned(&self) -> Owned {
    Owned(View {
      drops: self.drops.clone(),
      value: self.value,
    })
  }
}

#[test]
fn standard_generic_to_owned_and_drop_semantics_are_preserved() {
  fn own<B: ?Sized + std::borrow::ToOwned>(value: Cow<'_, B>) -> B::Owned {
    value.into_owned()
  }
  fn clone<'a, B: ?Sized + std::borrow::ToOwned>(value: &Cow<'a, B>) -> Cow<'a, B> {
    value.clone()
  }
  let drops = Rc::new(Cell::new(0));
  let source = View {
    drops: drops.clone(),
    value: 42,
  };
  let mut value: Cow<'_, View> = Cow::Borrowed(&source);
  value.to_mut().0.value = 7;
  let copied = clone(&value);
  let standard = value.into_std();
  assert_eq!(drops.get(), 0);
  let value = Cow::from_std(standard);
  let owned = own(value);
  assert_eq!(owned.0.value, 7);
  assert_eq!(drops.get(), 0);
  drop(owned);
  assert_eq!(drops.get(), 1);
  drop(copied);
  assert_eq!(drops.get(), 2);
}

#[test]
fn extracting_custom_storage_moves_drop_values_exactly_once() {
  struct Custom;
  impl ToOwned<Custom> for View {
    type Owned = Owned;
    type Storage<'a> = CowRepr<'a, Self, Custom>;
  }
  let drops = Rc::new(Cell::new(0));
  let value: Cow<'_, View, Custom> = Cow::owned(Owned(View {
    drops: drops.clone(),
    value: 42,
  }));
  let storage = value.into_storage();
  assert_eq!(drops.get(), 0);
  match storage {
    CowRepr::Owned(value) => {
      assert_eq!(value.0.value, 42);
      drop(value);
    }
    CowRepr::Borrowed(_) => panic!("lost owned variant"),
  }
  assert_eq!(drops.get(), 1);
}
