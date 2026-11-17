use cower::{Cow, StaticPolicy, ToOwned};

#[test]
fn default_methods_support_strings_and_slices_in_every_feature_tier() {
  let mut text = <str as ToOwned>::to_owned("hello");
  <str as ToOwned>::clone_into("world", &mut text);
  assert_eq!(text, "world");

  let mut bytes = <[u8] as ToOwned>::to_owned(b"hello");
  <[u8] as ToOwned>::clone_into(b"world", &mut bytes);
  assert_eq!(bytes, b"world");
}

#[test]
fn static_policy_methods_retain_static_references_without_cloning() {
  struct NotClone(u32);
  static FIRST: NotClone = NotClone(1);
  static SECOND: NotClone = NotClone(2);

  let mut value = <NotClone as ToOwned<StaticPolicy>>::to_owned(&FIRST);
  assert!(core::ptr::eq(value, &FIRST));
  <NotClone as ToOwned<StaticPolicy>>::clone_into(&SECOND, &mut value);
  assert!(core::ptr::eq(value, &SECOND));
  assert_eq!(value.0, 2);

  let mut text = <str as ToOwned<StaticPolicy>>::to_owned("old");
  <str as ToOwned<StaticPolicy>>::clone_into("new", &mut text);
  assert_eq!(text, "new");

  static VALUES: [NotClone; 2] = [NotClone(1), NotClone(2)];
  let mut values = <[NotClone] as ToOwned<StaticPolicy>>::to_owned(&VALUES);
  <[NotClone] as ToOwned<StaticPolicy>>::clone_into(&VALUES[1..], &mut values);
  assert!(core::ptr::eq(values, &VALUES[1..]));
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn default_slice_methods_do_not_require_clone_without_alloc() {
  struct NotClone;
  static SOURCE: [NotClone; 1] = [NotClone];

  let mut values = <[NotClone] as ToOwned>::to_owned(&SOURCE);
  assert!(core::ptr::eq(values, SOURCE.as_slice()));
  <[NotClone] as ToOwned>::clone_into(&SOURCE[..0], &mut values);
  assert!(values.is_empty());
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn default_clone_into_reuses_string_and_vector_storage() {
  let source = String::from("new");
  let mut text = String::with_capacity(128);
  text.push_str("old contents");
  let pointer = text.as_ptr();
  let capacity = text.capacity();
  <str as ToOwned>::clone_into(source.as_str(), &mut text);
  assert_eq!(text, "new");
  assert_eq!(text.as_ptr(), pointer);
  assert_eq!(text.capacity(), capacity);

  let source = [1, 2, 3];
  let mut values = Vec::with_capacity(128);
  values.extend_from_slice(&[4, 5, 6, 7]);
  let pointer = values.as_ptr();
  let capacity = values.capacity();
  <[i32] as ToOwned>::clone_into(&source, &mut values);
  assert_eq!(values, source);
  assert_eq!(values.as_ptr(), pointer);
  assert_eq!(values.capacity(), capacity);
}

// From and ToOwned produce equal borrowed data but record the conversion route.
struct View(u32);
struct Storage {
  view: View,
  route: &'static str,
}

impl core::borrow::Borrow<View> for Storage {
  fn borrow(&self) -> &View {
    &self.view
  }
}

impl From<&View> for Storage {
  fn from(view: &View) -> Self {
    Self {
      view: View(view.0),
      route: "from",
    }
  }
}

struct CustomPolicy;
impl ToOwned<CustomPolicy> for View {
  type Owned = Storage;
  type Storage<'a>
    = cower::CowRepr<'a, Self, CustomPolicy>
  where
    Self: 'a;

  fn to_owned<'a>(&'a self) -> Self::Owned
  where
    Self::Owned: From<&'a Self>,
  {
    Storage {
      view: View(self.0),
      route: "custom to_owned",
    }
  }
}

#[test]
fn default_clone_into_and_cow_honor_custom_to_owned_overrides() {
  let source = View(42);
  let mut target = Storage::from(&View(0));
  <View as ToOwned<CustomPolicy>>::clone_into(&source, &mut target);
  assert_eq!(target.view.0, 42);
  assert_eq!(target.route, "custom to_owned");

  let value: Cow<'_, View, CustomPolicy> = Cow::borrowed(&source);
  assert_eq!(value.into_owned().route, "custom to_owned");
  let mut value: Cow<'_, View, CustomPolicy> = Cow::borrowed(&source);
  assert_eq!(value.to_mut().route, "custom to_owned");

  // Already-owned storage bypasses conversion.
  let mut value: Cow<'_, View, CustomPolicy> = Cow::owned(Storage::from(&source));
  assert_eq!(value.to_mut().route, "from");
  assert_eq!(value.into_owned().route, "from");
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl std::borrow::ToOwned for View {
  type Owned = Storage;

  fn to_owned(&self) -> Storage {
    Storage {
      view: View(self.0),
      route: "standard to_owned",
    }
  }

  fn clone_into(&self, target: &mut Storage) {
    target.view.0 = self.0;
    target.route = "standard clone_into";
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn default_policy_forwards_both_standard_trait_methods() {
  let source = View(42);
  let mut target = <View as ToOwned>::to_owned(&source);
  assert_eq!(target.view.0, 42);
  assert_eq!(target.route, "standard to_owned");
  <View as ToOwned>::clone_into(&source, &mut target);
  assert_eq!(target.route, "standard clone_into");

  let value: Cow<'_, View> = Cow::borrowed(&source);
  assert_eq!(value.into_owned().route, "standard to_owned");
}
