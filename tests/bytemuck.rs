#![cfg(feature = "bytemuck_1")]

use bytemuck_1::TransparentWrapper;
use cower::{Cow, CowRepr, StaticPolicy, ToOwned};
use std::{cell::Cell, rc::Rc};

enum Policy {}
struct Owner(String, Rc<Cell<usize>>);

impl core::borrow::Borrow<str> for Owner {
  fn borrow(&self) -> &str {
    &self.0
  }
}
impl Drop for Owner {
  fn drop(&mut self) {
    self.1.set(self.1.get() + 1);
  }
}
impl ToOwned<Policy> for str {
  type Owned = Owner;
  type Storage<'a> = CowRepr<'a, Self, Policy>;
}

#[test]
fn owned_storage_moves_without_cloning_or_double_drop() {
  let drops = Rc::new(Cell::new(0));
  let owner = Owner("owned".into(), drops.clone());
  let pointer = owner.0.as_ptr();
  let value = Cow::<'_, str, Policy>::wrap(CowRepr::Owned(owner));
  assert_eq!(value.as_ptr(), pointer);
  assert_eq!(drops.get(), 0);
  let storage = Cow::peel(value);
  assert_eq!(storage.as_ptr(), pointer);
  drop(storage);
  assert_eq!(drops.get(), 1);
}

#[test]
fn references_and_slices_preserve_identity_and_mutations() {
  type Value<'a> = Cow<'a, str, StaticPolicy>;
  let local = String::from("local");
  let storage = CowRepr::Borrowed(local.as_str());
  let value = Value::wrap_ref(&storage);
  assert!(core::ptr::eq(Value::peel_ref(value), &storage));
  assert!(core::ptr::eq(value.as_borrowed().unwrap(), local.as_str()));

  let mut storage = [CowRepr::Borrowed(local.as_str()), CowRepr::Owned("old")];
  let values = Value::wrap_slice(&storage);
  assert_eq!(Value::peel_slice(values).as_ptr(), storage.as_ptr());
  let values = Value::wrap_slice_mut(&mut storage);
  values[0] = Value::owned("new");
  *Value::peel_mut(&mut values[1]) = CowRepr::Borrowed("changed");
  assert_eq!(&*storage[0], "new");
  assert_eq!(&*storage[1], "changed");
  assert_eq!(local, "local");

  let empty: &[CowRepr<'_, str, StaticPolicy>] = &[];
  assert!(Value::wrap_slice(empty).is_empty());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn standard_cow_references_roundtrip_both_ways() {
  let mut storage = std::borrow::Cow::Owned(String::from("hello"));
  let value = Cow::<'_, str>::wrap_mut(&mut storage);
  assert!(core::ptr::eq(Cow::peel_ref(value), value.as_std()));
  value.to_mut().push('!');
  assert_eq!(storage, "hello!");
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn allocated_wrappers_preserve_allocation_capacity_and_drop_once() {
  use bytemuck_1::TransparentWrapperAlloc;
  type Value = Cow<'static, str, Policy>;
  let drops = Rc::new(Cell::new(0));
  let mut storage = Vec::with_capacity(8);
  storage.push(CowRepr::Owned(Owner("first".into(), drops.clone())));
  let pointer = storage.as_ptr();
  let capacity = storage.capacity();
  let values = Value::wrap_vec(storage);
  assert_eq!(values.as_ptr().cast::<CowRepr<'_, str, Policy>>(), pointer);
  assert_eq!(values.capacity(), capacity);
  let storage = Value::peel_vec(values);
  assert_eq!(storage.as_ptr(), pointer);
  drop(storage);
  assert_eq!(drops.get(), 1);

  let storage = Box::new(CowRepr::Owned(Owner("second".into(), drops.clone())));
  let pointer = &*storage as *const CowRepr<'_, str, Policy>;
  let value = Value::wrap_box(storage);
  assert_eq!((&*value as *const Value).cast(), pointer);
  drop(value);
  assert_eq!(drops.get(), 2);
}
