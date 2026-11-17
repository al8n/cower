#![cfg(any(
  feature = "quickcheck_1",
  feature = "proptest_1",
  feature = "arbitrary_1"
))]

#[cfg(feature = "arbitrary_1")]
use arbitrary_1 as arbitrary;
#[cfg(feature = "proptest_1")]
use proptest_1 as proptest;
#[cfg(feature = "quickcheck_1")]
use quickcheck_1 as quickcheck;

use cower::{Cow, CowRepr, ToOwned};

// No framework traits on the policy; these also exercise the core-only tier.
enum Custom {}
impl ToOwned<Custom> for u16 {
  type Owned = u16;
  type Storage<'a> = CowRepr<'a, Self, Custom>;
}
type Value = Cow<'static, u16, Custom>;

#[cfg(feature = "quickcheck_1")]
#[test]
fn quickcheck_generates_owned_values_and_shrinks_them() {
  use quickcheck::{Arbitrary, Gen};
  let value = Value::arbitrary(&mut Gen::new(16));
  assert!(value.is_owned());
  let shrunk: Vec<_> = Value::owned(128).shrink().collect();
  assert!(!shrunk.is_empty());
  assert!(shrunk.iter().all(|v| v.is_owned() && *v.as_ref() < 128));
  assert!(shrunk.iter().any(|v| *v.as_ref() == 0));
  assert_eq!(Value::borrowed(&128).shrink().count(), 0);
}

#[cfg(feature = "proptest_1")]
#[test]
fn proptest_preserves_the_underlying_shrink_tree() {
  use proptest::{
    arbitrary::any,
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
  };
  let mut runner = TestRunner::deterministic();
  let mut tree = any::<Value>().new_tree(&mut runner).unwrap();
  assert!(tree.current().is_owned());
  while tree.simplify() {
    assert!(tree.current().is_owned());
  }
  assert_eq!(*tree.current().as_ref(), 0);
}

#[cfg(feature = "arbitrary_1")]
#[test]
fn arbitrary_forwards_generation_and_size_hints() {
  use arbitrary::{Arbitrary, Unstructured};
  let bytes = [0x12, 0x34];
  let value = Value::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
  let expected = u16::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
  assert!(value.is_owned());
  assert_eq!(*value.as_ref(), expected);
  assert_eq!(Value::size_hint(0), u16::size_hint(0));
  assert!(Value::try_size_hint(usize::MAX).is_err());
}

#[cfg(all(feature = "quickcheck_1", any(feature = "std", feature = "alloc")))]
#[test]
fn quickcheck_accepts_default_cows_as_test_arguments() {
  fn property(value: Cow<'static, str>) -> bool {
    value.is_owned() && value.clone().into_owned() == value.as_ref()
  }
  quickcheck::QuickCheck::new()
    .tests(16)
    .quickcheck(property as fn(Cow<'static, str>) -> bool);
}

#[cfg(all(feature = "proptest_1", any(feature = "std", feature = "alloc")))]
#[test]
fn proptest_forwards_owned_parameters() {
  use proptest::{
    arbitrary::any_with,
    collection::SizeRange,
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
  };
  let params = (SizeRange::from(3..=3), ());
  let strategy = any_with::<Cow<'static, [u8]>>(params);
  let value = strategy
    .new_tree(&mut TestRunner::deterministic())
    .unwrap()
    .current();
  assert!(value.is_owned());
  assert_eq!(value.len(), 3);
}

#[cfg(all(feature = "arbitrary_1", any(feature = "std", feature = "alloc")))]
#[test]
fn arbitrary_take_rest_uses_the_entire_input() {
  use arbitrary::{Arbitrary, Unstructured};
  let value = Cow::<'static, str>::arbitrary_take_rest(Unstructured::new(b"hello")).unwrap();
  assert!(value.is_owned());
  assert_eq!(value, "hello");
}
