#![cfg(any(feature = "std", feature = "alloc"))]

use cower::Cow;

#[test]
fn owned_values_convert_to_shared_pointers_and_boxed_errors() {
  use std::{error::Error, rc::Rc, sync::Arc};

  let value: Cow<'_, str> = Cow::owned(String::from("shared"));
  let shared: Rc<str> = value.into();
  assert_eq!(&*shared, "shared");
  assert_eq!(Rc::strong_count(&shared), 1);
  let value: Cow<'_, [u8]> = Cow::owned(vec![1, 2, 3]);
  let shared: Arc<[u8]> = value.into();
  assert_eq!(&*shared, &[1, 2, 3]);
  assert_eq!(Arc::strong_count(&shared), 1);

  for value in [
    Cow::<'_, str>::borrowed("message"),
    Cow::owned(String::from("message")),
  ] {
    let error: Box<dyn Error> = value.clone().into();
    assert_eq!(error.to_string(), "message");
    let error: Box<dyn Error + Send + Sync> = value.into();
    assert_eq!(error.to_string(), "message");
  }
}

#[test]
fn borrowed_container_conversions_create_independent_owned_contents() {
  use std::ffi::{CStr, CString};

  let source = vec![String::from("local")];
  let value: Cow<'_, [String]> = Cow::from(&source);
  assert_eq!(value.as_borrowed().unwrap().as_ptr(), source.as_ptr());
  let mut owned: Vec<String> = value.into();
  owned[0].push('!');
  assert_eq!(source[0], "local");
  let boxed: Box<[String]> = Cow::<'_, [String]>::from(&source).into();
  assert_eq!(&*boxed, source.as_slice());
  assert_ne!(boxed[0].as_ptr(), source[0].as_ptr());

  let source = CString::new("local").unwrap();
  let value: Cow<'_, CStr> = Cow::from(&source);
  let owned: CString = value.into();
  assert_eq!(owned, source);
  assert_ne!(owned.as_ptr(), source.as_ptr());
  let boxed: Box<CStr> = Cow::<'_, CStr>::from(&source).into();
  assert_eq!(&*boxed, source.as_c_str());
}

#[cfg(feature = "std")]
#[test]
fn os_string_conversions_preserve_native_data_and_owned_allocations() {
  use std::ffi::{OsStr, OsString};

  let mut source = OsString::with_capacity(128);
  source.push("local");
  let pointer = source.as_encoded_bytes().as_ptr();
  let capacity = source.capacity();
  let value: Cow<'_, OsStr> = source.into();
  let source: OsString = value.into();
  assert_eq!(source.as_encoded_bytes().as_ptr(), pointer);
  assert_eq!(source.capacity(), capacity);

  for value in [Cow::<'_, OsStr>::from(&source), Cow::from(source.clone())] {
    let boxed: Box<OsStr> = value.into();
    assert_eq!(&*boxed, source.as_os_str());
  }

  // Invalid UTF-8 must survive the conversions without a lossy text round trip.
  #[cfg(unix)]
  {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    let source = OsString::from_vec(vec![b'a', 0xff, b'b']);
    let boxed: Box<OsStr> = Cow::<'_, OsStr>::from(&source).into();
    assert_eq!(boxed.as_bytes(), source.as_bytes());
    let owned: OsString = Cow::<'_, OsStr>::from(&source).into();
    assert_eq!(owned.into_vec(), source.into_vec());
  }
}
