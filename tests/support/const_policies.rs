use cower::{Cow, CowRepr, ToOwned};

// The policy is deliberately unsized, and is never instantiated.
pub struct Custom {
  _marker: [u8],
}

impl ToOwned<Custom> for str {
  type Owned = &'static str;
  type Storage<'a> = CowRepr<'a, str, Custom>;
}

impl<T: 'static> ToOwned<Custom> for [T] {
  type Owned = &'static [T];
  type Storage<'a> = CowRepr<'a, [T], Custom>;
}

// These functions must type-check with an unknown P, not just named policies.
const fn from_static<T, P: ?Sized>(value: &'static T) -> Cow<'static, T, P>
where
  T: ?Sized + ToOwned<P, Storage<'static> = CowRepr<'static, T, P>>,
{
  Cow::<T, P>::from_static(value)
}

const fn text<'a, 's, P: ?Sized>(value: &'s Cow<'a, str, P>) -> &'s str
where
  str: ToOwned<P, Owned = &'static str, Storage<'a> = CowRepr<'a, str, P>>,
{
  value.as_str()
}

const fn utf8<'a, 's, P: ?Sized>(value: &'s Cow<'a, str, P>) -> &'s [u8]
where
  str: ToOwned<P, Owned = &'static str, Storage<'a> = CowRepr<'a, str, P>>,
{
  value.as_bytes()
}

const fn slice<'a, 's, T: 'static, P: ?Sized>(value: &'s Cow<'a, [T], P>) -> &'s [T]
where
  [T]: ToOwned<P, Owned = &'static [T], Storage<'a> = CowRepr<'a, [T], P>>,
{
  value.as_slice()
}

const fn bytes<'a, 's, P: ?Sized>(value: &'s Cow<'a, [u8], P>) -> &'s [u8]
where
  [u8]: ToOwned<P, Owned = &'static [u8], Storage<'a> = CowRepr<'a, [u8], P>>,
{
  value.as_bytes()
}

// No Clone implementation: construction and slice access must not require one.
pub struct Item(pub u16);

struct DifferentOwned;
struct OpaqueOwned;
impl core::borrow::Borrow<str> for OpaqueOwned {
  fn borrow(&self) -> &str {
    "opaque"
  }
}
impl ToOwned<DifferentOwned> for str {
  type Owned = OpaqueOwned;
  type Storage<'a> = CowRepr<'a, str, DifferentOwned>;
}
const _: () = {
  static VALUE: Cow<'static, str, DifferentOwned> = from_static::<str, DifferentOwned>("borrowed");
  match VALUE.as_storage() {
    CowRepr::Borrowed(value) => super::assert_text(value, "borrowed"),
    CowRepr::Owned(_) => panic!("from_static must preserve borrowed storage"),
  }
};

static TEXT: Cow<'static, str, Custom> = from_static::<str, Custom>("hé🦀");
static OWNED_TEXT: Cow<'static, str, Custom> = Cow::from_storage(CowRepr::Owned("owned"));
static BYTES: Cow<'static, [u8], Custom> = from_static::<[u8], Custom>(b"bytes");
static OWNED_BYTES: Cow<'static, [u8], Custom> =
  Cow::from_storage(CowRepr::<[u8], Custom>::Owned(b"owned"));
static ITEMS: Cow<'static, [Item], Custom> = from_static::<[Item], Custom>(&[Item(256), Item(513)]);
static OWNED_ITEMS: Cow<'static, [Item], Custom> =
  Cow::from_storage(CowRepr::<[Item], Custom>::Owned(&[Item(1024)]));
static EMPTY: Cow<'static, [Item], Custom> = from_static::<[Item], Custom>(&[]);
static ZST: Cow<'static, [()], Custom> = from_static::<[()], Custom>(&[(), (), ()]);

const _: () = {
  const TEXT_REF: &str = TEXT.as_str();
  const UTF8_REF: &[u8] = TEXT.as_bytes();
  const BYTE_REF: &[u8] = BYTES.as_bytes();
  const SLICE_REF: &[Item] = ITEMS.as_slice();
  super::assert_text(TEXT_REF, "hé🦀");
  super::assert_bytes(UTF8_REF, b"h\xc3\xa9\xf0\x9f\xa6\x80");
  super::assert_bytes(BYTE_REF, b"bytes");
  assert!(SLICE_REF[0].0 == 256 && SLICE_REF[1].0 == 513);
  super::assert_text(text(&TEXT), TEXT_REF);
  super::assert_text(text(&OWNED_TEXT), "owned");
  super::assert_bytes(utf8(&TEXT), UTF8_REF);
  super::assert_bytes(utf8(&OWNED_TEXT), b"owned");
  super::assert_bytes(bytes(&BYTES), BYTE_REF);
  super::assert_bytes(bytes(&OWNED_BYTES), b"owned");
  assert!(slice(&ITEMS)[1].0 == 513);
  assert!(slice(&OWNED_ITEMS)[0].0 == 1024);
  assert!(slice(&EMPTY).is_empty());
  assert!(slice(&ZST).len() == 3);
};

#[cfg(not(any(feature = "std", feature = "alloc")))]
const _: () = {
  static DEFAULT: Cow<'static, [Item]> = Cow::<[Item]>::from_static(&[Item(256)]);
  static OWNED: Cow<'static, [Item]> = Cow::<[Item]>::Owned(&[Item(513)]);
  static OWNED_BYTES: Cow<'static, [u8]> = Cow::<[u8]>::Owned(b"owned");
  const VIEW: &[Item] = DEFAULT.as_slice();
  const OWNED_VIEW: &[Item] = OWNED.as_slice();
  const BYTES: &[u8] = super::DEFAULT_BYTES.as_bytes();
  assert!(VIEW[0].0 == 256);
  assert!(OWNED_VIEW[0].0 == 513);
  super::assert_bytes(BYTES, b"bytes");
  super::assert_bytes(OWNED_BYTES.as_bytes(), b"owned");
};

#[test]
fn custom_policy_access_preserves_local_borrows() {
  let local = [Item(42), Item(84)];
  let value: Cow<'_, [Item], Custom> = Cow::borrowed(&local);
  assert!(core::ptr::eq(slice(&value), &local[..]));
  let local = [0_u8, 127, 255];
  let value: Cow<'_, [u8], Custom> = Cow::borrowed(&local);
  assert!(core::ptr::eq(bytes(&value), &local[..]));
  assert_eq!(bytes(&value), &[0, 127, 255]);
}

#[test]
fn wrapper_retains_storage_layout_and_marker_auto_traits() {
  use core::mem::{align_of, size_of};
  assert_eq!(
    size_of::<Cow<'_, str, Custom>>(),
    size_of::<CowRepr<'_, str, Custom>>()
  );
  assert_eq!(
    align_of::<Cow<'_, str, Custom>>(),
    align_of::<CowRepr<'_, str, Custom>>()
  );
  assert_eq!(
    &TEXT as *const Cow<'_, str, Custom> as *const (),
    TEXT.as_storage() as *const CowRepr<'_, str, Custom> as *const (),
  );

  struct NonThreadSafePolicy {
    _marker: core::cell::UnsafeCell<*const ()>,
  }
  impl ToOwned<NonThreadSafePolicy> for str {
    type Owned = &'static str;
    type Storage<'a> = CowRepr<'a, str, NonThreadSafePolicy>;
  }
  fn send_sync<T: Send + Sync>() {}
  send_sync::<Cow<'static, str, NonThreadSafePolicy>>();
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod heap {
  use super::*;
  use alloc::{
    borrow::{Cow as StdCow, ToOwned as StdToOwned},
    string::String,
    vec::Vec,
  };

  struct CustomHeap;
  impl ToOwned<CustomHeap> for str {
    type Owned = String;
    type Storage<'a> = StdCow<'a, str>;
  }
  impl<T: Clone> ToOwned<CustomHeap> for [T] {
    type Owned = Vec<T>;
    type Storage<'a>
      = StdCow<'a, [T]>
    where
      T: 'a;
  }

  const fn from_static<T, P: ?Sized>(value: &'static T) -> Cow<'static, T, P>
  where
    T: ?Sized
      + StdToOwned
      + ToOwned<P, Owned = <T as StdToOwned>::Owned, Storage<'static> = StdCow<'static, T>>,
  {
    Cow::<T, P>::from_static(value)
  }

  #[rustversion::attr(since(1.87), const)]
  fn text<'a, 's, P: ?Sized>(value: &'s Cow<'a, str, P>) -> &'s str
  where
    str: ToOwned<P, Owned = String, Storage<'a> = StdCow<'a, str>>,
  {
    value.as_str()
  }

  #[rustversion::attr(since(1.87), const)]
  fn utf8<'a, 's, P: ?Sized>(value: &'s Cow<'a, str, P>) -> &'s [u8]
  where
    str: ToOwned<P, Owned = String, Storage<'a> = StdCow<'a, str>>,
  {
    value.as_bytes()
  }

  #[rustversion::attr(since(1.87), const)]
  fn slice<'a, 's, T: Clone, P: ?Sized>(value: &'s Cow<'a, [T], P>) -> &'s [T]
  where
    [T]: ToOwned<P, Owned = Vec<T>, Storage<'a> = StdCow<'a, [T]>>,
  {
    value.as_slice()
  }

  #[rustversion::attr(since(1.87), const)]
  fn bytes<'a, 's, P: ?Sized>(value: &'s Cow<'a, [u8], P>) -> &'s [u8]
  where
    [u8]: ToOwned<P, Owned = Vec<u8>, Storage<'a> = StdCow<'a, [u8]>>,
  {
    value.as_bytes()
  }

  static TEXT: Cow<'static, str, CustomHeap> = from_static::<str, CustomHeap>("hé🦀");
  static BYTES: Cow<'static, [u8], CustomHeap> = from_static::<[u8], CustomHeap>(b"bytes");
  static ITEMS: Cow<'static, [u16], CustomHeap> = from_static::<[u16], CustomHeap>(&[256, 513]);

  // Also check ordinary method availability on the 1.85/1.86 fixture lanes.
  const _: for<'s> fn(&'s Cow<'static, str, CustomHeap>) -> &'s str = text::<CustomHeap>;
  const _: for<'s> fn(&'s Cow<'static, str, CustomHeap>) -> &'s [u8] = utf8::<CustomHeap>;
  const _: for<'s> fn(&'s Cow<'static, [u8], CustomHeap>) -> &'s [u8] = bytes::<CustomHeap>;
  const _: for<'s> fn(&'s Cow<'static, [u16], CustomHeap>) -> &'s [u16] = slice::<u16, CustomHeap>;
  const _: &Cow<'static, str, CustomHeap> = &TEXT;
  const _: &Cow<'static, [u8], CustomHeap> = &BYTES;
  const _: &Cow<'static, [u16], CustomHeap> = &ITEMS;

  #[rustversion::since(1.87)]
  const _: () = {
    static EMPTY: Cow<'static, [u16], CustomHeap> = Cow::from_std(StdCow::Owned(Vec::new()));
    static EMPTY_TEXT: Cow<'static, str, CustomHeap> = Cow::from_std(StdCow::Owned(String::new()));
    static DEFAULT: Cow<'static, [u16]> = Cow::<[u16]>::from_static(&[256, 513]);
    static OWNED: Cow<'static, [u16]> = Cow::Owned(Vec::new());
    static OWNED_BYTES: Cow<'static, [u8]> = Cow::Owned(Vec::new());
    const VIEW: &[u16] = ITEMS.as_slice();
    const BYTE_VIEW: &[u8] = BYTES.as_bytes();
    super::super::assert_text(text(&TEXT), "hé🦀");
    super::super::assert_bytes(utf8(&TEXT), b"h\xc3\xa9\xf0\x9f\xa6\x80");
    super::super::assert_text(text(&EMPTY_TEXT), "");
    super::super::assert_bytes(utf8(&EMPTY_TEXT), b"");
    super::super::assert_bytes(BYTE_VIEW, b"bytes");
    super::super::assert_bytes(bytes(&BYTES), BYTE_VIEW);
    assert!(VIEW[0] == 256 && slice(&ITEMS)[1] == 513);
    assert!(slice(&EMPTY).is_empty());
    assert!(DEFAULT.as_slice()[1] == 513);
    assert!(OWNED.as_slice().is_empty());
    assert!(OWNED_BYTES.as_bytes().is_empty());
    super::super::assert_bytes(super::super::DEFAULT_BYTES.as_bytes(), b"bytes");
  };

  #[test]
  fn heap_slice_access_preserves_allocation_and_tracks_mutation() {
    let owned = alloc::vec![256_u16, 513];
    let pointer = owned.as_ptr();
    let mut value: Cow<'static, [u16], CustomHeap> = Cow::from_std(StdCow::Owned(owned));
    assert_eq!(slice(&value), &[256, 513]);
    assert_eq!(slice(&value).as_ptr(), pointer);
    value.to_mut().push(1024);
    assert_eq!(slice(&value), &[256, 513, 1024]);
    let owned = alloc::vec![0_u8, 127, 255];
    let pointer = owned.as_ptr();
    let value: Cow<'static, [u8], CustomHeap> = Cow::from_std(StdCow::Owned(owned));
    assert_eq!(bytes(&value), &[0, 127, 255]);
    assert_eq!(bytes(&value).as_ptr(), pointer);
  }
}
