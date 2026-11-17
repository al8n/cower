<div align="center">
<h1>cower</h1>
</div>
<div align="center">

Policy-based clone-on-write storage for `no_std`, with or without `alloc`.

[<img alt="github" src="https://img.shields.io/badge/github-al8n/cower-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
<img alt="LoC" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fgist.githubusercontent.com%2Fal8n%2F327b2a8aef9003246e45c6e47fe63937%2Fraw%2Fcower" height="22">
[<img alt="Build" src="https://img.shields.io/github/actions/workflow/status/al8n/cower/ci.yml?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
[<img alt="codecov" src="https://img.shields.io/codecov/c/gh/al8n/cower?style=for-the-badge&token=6R3QFWRWHL&logo=codecov" height="22">][codecov-url]

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-cower-66c2a5?style=for-the-badge&labelColor=555555&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/cower?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPg0KPHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDUxMiA1MTIiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPGc+DQoJPGc+DQoJCTxwYXRoIGQ9Ik0yNTYsMEwzMS41MjgsMTEyLjIzNnYyODcuNTI4TDI1Niw1MTJsMjI0LjQ3Mi0xMTIuMjM2VjExMi4yMzZMMjU2LDB6IE0yMzQuMjc3LDQ1Mi41NjRMNzQuOTc0LDM3Mi45MTNWMTYwLjgxDQoJCQlsMTU5LjMwMyw3OS42NTFWNDUyLjU2NHogTTEwMS44MjYsMTI1LjY2MkwyNTYsNDguNTc2bDE1NC4xNzQsNzcuMDg3TDI1NiwyMDIuNzQ5TDEwMS44MjYsMTI1LjY2MnogTTQzNy4wMjYsMzcyLjkxMw0KCQkJbC0xNTkuMzAzLDc5LjY1MVYyNDAuNDYxbDE1OS4zMDMtNzkuNjUxVjM3Mi45MTN6IiBmaWxsPSIjRkZGIi8+DQoJPC9nPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPC9zdmc+DQo=" height="22">][crates-url]
[<img alt="crates.io" src="https://img.shields.io/crates/d/cower?color=critical&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/PjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+PHN2ZyB0PSIxNjQ1MTE3MzMyOTU5IiBjbGFzcz0iaWNvbiIgdmlld0JveD0iMCAwIDEwMjQgMTAyNCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHAtaWQ9IjM0MjEiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkzIiB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48ZGVmcz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwvc3R5bGU+PC9kZWZzPjxwYXRoIGQ9Ik00NjkuMzEyIDU3MC4yNHYtMjU2aDg1LjM3NnYyNTZoMTI4TDUxMiA3NTYuMjg4IDM0MS4zMTIgNTcwLjI0aDEyOHpNMTAyNCA2NDAuMTI4QzEwMjQgNzgyLjkxMiA5MTkuODcyIDg5NiA3ODcuNjQ4IDg5NmgtNTEyQzEyMy45MDQgODk2IDAgNzYxLjYgMCA1OTcuNTA0IDAgNDUxLjk2OCA5NC42NTYgMzMxLjUyIDIyNi40MzIgMzAyLjk3NiAyODQuMTYgMTk1LjQ1NiAzOTEuODA4IDEyOCA1MTIgMTI4YzE1Mi4zMiAwIDI4Mi4xMTIgMTA4LjQxNiAzMjMuMzkyIDI2MS4xMkM5NDEuODg4IDQxMy40NCAxMDI0IDUxOS4wNCAxMDI0IDY0MC4xOTJ6IG0tMjU5LjItMjA1LjMxMmMtMjQuNDQ4LTEyOS4wMjQtMTI4Ljg5Ni0yMjIuNzItMjUyLjgtMjIyLjcyLTk3LjI4IDAtMTgzLjA0IDU3LjM0NC0yMjQuNjQgMTQ3LjQ1NmwtOS4yOCAyMC4yMjQtMjAuOTI4IDIuOTQ0Yy0xMDMuMzYgMTQuNC0xNzguMzY4IDEwNC4zMi0xNzguMzY4IDIxNC43MiAwIDExNy45NTIgODguODMyIDIxNC40IDE5Ni45MjggMjE0LjRoNTEyYzg4LjMyIDAgMTU3LjUwNC03NS4xMzYgMTU3LjUwNC0xNzEuNzEyIDAtODguMDY0LTY1LjkyLTE2NC45MjgtMTQ0Ljk2LTE3MS43NzZsLTI5LjUwNC0yLjU2LTUuODg4LTMwLjk3NnoiIGZpbGw9IiNmZmZmZmYiIHAtaWQ9IjM0MjIiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkwIiBjbGFzcz0iIj48L3BhdGg+PC9zdmc+&style=for-the-badge" height="22">][crates-url]
<img alt="license" src="https://img.shields.io/badge/License-Apache%202.0/MIT-blue.svg?style=for-the-badge&fontColor=white&logoColor=f5c076&logo=data:image/svg+xml;base64,PCFET0NUWVBFIHN2ZyBQVUJMSUMgIi0vL1czQy8vRFREIFNWRyAxLjEvL0VOIiAiaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkIj4KDTwhLS0gVXBsb2FkZWQgdG86IFNWRyBSZXBvLCB3d3cuc3ZncmVwby5jb20sIFRyYW5zZm9ybWVkIGJ5OiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4KPHN2ZyBmaWxsPSIjZmZmZmZmIiBoZWlnaHQ9IjgwMHB4IiB3aWR0aD0iODAwcHgiIHZlcnNpb249IjEuMSIgaWQ9IkNhcGFfMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmlld0JveD0iMCAwIDI3Ni43MTUgMjc2LjcxNSIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgc3Ryb2tlPSIjZmZmZmZmIj4KDTxnIGlkPSJTVkdSZXBvX2JnQ2FycmllciIgc3Ryb2tlLXdpZHRoPSIwIi8+Cg08ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4KDTxnIGlkPSJTVkdSZXBvX2ljb25DYXJyaWVyIj4gPGc+IDxwYXRoIGQ9Ik0xMzguMzU3LDBDNjIuMDY2LDAsMCw2Mi4wNjYsMCwxMzguMzU3czYyLjA2NiwxMzguMzU3LDEzOC4zNTcsMTM4LjM1N3MxMzguMzU3LTYyLjA2NiwxMzguMzU3LTEzOC4zNTcgUzIxNC42NDgsMCwxMzguMzU3LDB6IE0xMzguMzU3LDI1OC43MTVDNzEuOTkyLDI1OC43MTUsMTgsMjA0LjcyMywxOCwxMzguMzU3UzcxLjk5MiwxOCwxMzguMzU3LDE4IHMxMjAuMzU3LDUzLjk5MiwxMjAuMzU3LDEyMC4zNTdTMjA0LjcyMywyNTguNzE1LDEzOC4zNTcsMjU4LjcxNXoiLz4gPHBhdGggZD0iTTE5NC43OTgsMTYwLjkwM2MtNC4xODgtMi42NzctOS43NTMtMS40NTQtMTIuNDMyLDIuNzMyYy04LjY5NCwxMy41OTMtMjMuNTAzLDIxLjcwOC0zOS42MTQsMjEuNzA4IGMtMjUuOTA4LDAtNDYuOTg1LTIxLjA3OC00Ni45ODUtNDYuOTg2czIxLjA3Ny00Ni45ODYsNDYuOTg1LTQ2Ljk4NmMxNS42MzMsMCwzMC4yLDcuNzQ3LDM4Ljk2OCwyMC43MjMgYzIuNzgyLDQuMTE3LDguMzc1LDUuMjAxLDEyLjQ5NiwyLjQxOGM0LjExOC0yLjc4Miw1LjIwMS04LjM3NywyLjQxOC0xMi40OTZjLTEyLjExOC0xNy45MzctMzIuMjYyLTI4LjY0NS01My44ODItMjguNjQ1IGMtMzUuODMzLDAtNjQuOTg1LDI5LjE1Mi02NC45ODUsNjQuOTg2czI5LjE1Miw2NC45ODYsNjQuOTg1LDY0Ljk4NmMyMi4yODEsMCw0Mi43NTktMTEuMjE4LDU0Ljc3OC0zMC4wMDkgQzIwMC4yMDgsMTY5LjE0NywxOTguOTg1LDE2My41ODIsMTk0Ljc5OCwxNjAuOTAzeiIvPiA8L2c+IDwvZz4KDTwvc3ZnPg==" height="22">

</div>

`Cow<'a, T, P = DefaultPolicy>` is a transparent wrapper around the policy's
actual backing storage. With `std` or `alloc`, the default backing is the
standard `Cow` itself, allowing safe reference coercions without layout guesses.
The library has no required runtime dependencies; `rustversion` selects const APIs at compile time.
Optional features add inline and growable storage policies,
Serde, Borsh, JSON Schema, rkyv, and property-testing/fuzzing integrations.

## Standard Cow interoperability

With `std` or `alloc`, functions accepting a standard `Cow` reference can accept
our default `Cow` through ordinary deref coercion:

```rust,ignore
use cower::Cow;
use std::borrow::Cow as StdCow;

fn inspect(value: &StdCow<'_, str>) -> usize { value.len() }
fn edit(value: &mut StdCow<'_, str>) { value.to_mut().push('!'); }

let mut text: Cow<'_, str> = Cow::Borrowed("hello");
assert_eq!(inspect(&text), 5);
edit(&mut text);
assert_eq!(text, "hello!");

let reference: &StdCow<'_, str> = text.as_std();
assert_eq!(reference, "hello!");
let standard: StdCow<'_, str> = text.into();
let text: Cow<'_, str> = standard.into();
assert!(text.is_owned());
```

`as_std`, `as_std_mut`, `into_std`, and `from_std` also work in const contexts on
Rust 1.85. Reference access borrows the actual field; it does not reinterpret a
separate enum or construct a temporary.

```rust,ignore
use cower::Cow;
use std::borrow::Cow as StdCow;

static TEXT: Cow<'static, str> = Cow::Borrowed("constant");
const REFERENCE: &StdCow<'static, str> = TEXT.as_std();
const VALUE: StdCow<'static, str> = Cow::<'static, str>::Borrowed("constant").into_std();
const EMPTY: Cow<'static, str> = Cow::from_std(StdCow::Owned(String::new()));
assert_eq!(REFERENCE, "constant");
assert_eq!(VALUE, "constant");
assert!(EMPTY.is_owned());
```

Reference interoperability requires the policy's `Storage<'a>` to be exactly
`std::borrow::Cow<'a, T>`. Matching only the owned type, size, or alignment is not
enough. Custom policies may explicitly select standard backing to opt in.
Policies using `CowRepr` or fixed-capacity storage can use `borrow_std()` for a
borrowed standard value instead. Value conversions through `From` also work when
the owned types match, regardless of the backing enum.

## Const construction and content access

`from_static` constructs a borrowed `Cow` from `&'static T` without cloning or
allocating. It is generic over `P: ?Sized` for both `CowRepr` and standard `Cow`
backing, including custom policies. It is const on Rust 1.85; optional
integrations retain their own MSRVs. Specify the borrowed type and, if needed,
the policy to select the constructor:

```rust
use cower::{Cow, StaticPolicy};

const TEXT: Cow<'static, str> = Cow::<str>::from_static("hello");
const BYTES: Cow<'static, [u8]> = Cow::<[u8]>::from_static(b"hello");
static STATIC: Cow<'static, str, StaticPolicy> =
    Cow::<str, StaticPolicy>::from_static("hello");
const VIEW: &str = STATIC.as_str();
const UTF8: &[u8] = STATIC.as_bytes();
assert!(TEXT.is_borrowed());
assert_eq!(BYTES.as_ref(), b"hello");
assert_eq!(VIEW, "hello");
assert_eq!(UTF8, b"hello");
```

This keeps the borrowed variant even for `StaticPolicy` and does not check inline
capacity. The public `Cow<'a, T, P>` alias selects an internal transparent wrapper
whose explicit storage parameter lets Rust distinguish the const implementations.
The alias is the same type: call these inherent methods directly on `Cow`, with
no storage conversion or `Deref` call required.
A completely different custom `Storage` type can use const `Cow::from_storage`;
stable Rust cannot call its `CowStorage::borrowed` trait method in a const context.

Content access is also generic over the policy when its storage matches:

| Borrowed type | Method | Result |
| --- | --- | --- |
| `str` | `as_str()` | `&str` |
| `str` | `as_bytes()` | UTF-8 `&[u8]` |
| `[u8]` | `as_bytes()` | `&[u8]` |
| `[T]` | `as_slice()` | `&[T]` |

For the public `Cow<'a, T>` with its default policy:

| Enabled features | Const content access |
| --- | --- |
| Neither `alloc` nor `std` | Rust 1.85+ |
| `alloc` without `std` | Rust 1.87+ |
| `std` (default) | Rust 1.87+ |

With standard `Cow` backing, these methods are const on Rust **1.87+**, matching
`String::as_str` and `Vec::as_slice`; on Rust 1.85/1.86 they remain ordinary
methods. With `CowRepr` backing and static-reference owned storage, they are const
on Rust **1.85**, including core-only `DefaultPolicy`, `StaticPolicy`, and custom
policies. Static-reference slice access does not require `T: Clone`. Other owned
containers do not automatically gain const accessors. Every returned reference
borrows `self` and cannot outlive owned storage.

```rust
// Custom policy: works without std or alloc, on Rust 1.85.
use cower::{Cow, CowRepr, ToOwned};

struct Custom;
impl<T: 'static> ToOwned<Custom> for [T] {
    type Owned = &'static [T];
    type Storage<'a> = CowRepr<'a, [T], Custom>;
}
static NUMBERS: Cow<'static, [u16], Custom> =
    Cow::<[u16], Custom>::from_static(&[256, 513]);
static BYTES: Cow<'static, [u8], Custom> =
    Cow::<[u8], Custom>::from_static(b"hello");
const SLICE: &[u16] = NUMBERS.as_slice();
const VIEW: &[u8] = BYTES.as_bytes();
assert_eq!(SLICE, &[256, 513]);
assert_eq!(VIEW, b"hello");
```

```rust,ignore
// The same public Cow API in core-only, alloc-only, and std builds on Rust 1.87+.
use cower::Cow;

static TEXT: Cow<'static, str> = Cow::<str>::from_static("hello");
const VIEW: &str = TEXT.as_str();
const UTF8: &[u8] = TEXT.as_bytes();
const fn view<'s>(value: &'s Cow<'_, str>) -> &'s str { value.as_str() }
assert_eq!(view(&TEXT), VIEW);
assert_eq!(UTF8, b"hello");
```

## Policies

| Policy and borrowed type | With `std` or `alloc` | With neither feature |
| --- | --- | --- |
| `DefaultPolicy`, `str` | `String` | `&'static str` |
| `DefaultPolicy`, `[T]` | `Vec<T>` (`T: Clone`) | `&'static [T]` (`T: 'static`) |
| `DefaultPolicy`, other types | Standard `ToOwned::Owned` | No built-in mapping |
| `StaticPolicy`, any `T: ?Sized + 'static` | `&'static T` | `&'static T` |
| `Heapless<N>`, `str` (feature `heapless`) | `heapless::String<N>` | `heapless::String<N>` |
| `Heapless<N>`, `[T]` (feature `heapless`) | `heapless::Vec<T, N>` | `heapless::Vec<T, N>` |
| User-defined policy | User-defined storage | User-defined storage |

`DefaultPolicy` uses a blanket implementation only with `std` or `alloc`.
Without either feature, its built-in implementations cover only `str` and `[T]`.
`StaticPolicy` always provides blanket static-reference storage.

The built-in policies are uninhabited enums used only as type parameters. They
have no constructors or `Default` implementations, and `Cow` stores no policy
instance. This does not affect `Cow::default()` when its owned storage supports it.

```rust
use cower::{Cow, CowRepr, StaticPolicy};

const MESSAGE: Cow<'static, str> = Cow::Borrowed("hello");
static ANSWER: u32 = 42;
const STATIC: Cow<'static, u32, StaticPolicy> =
    Cow::from_storage(CowRepr::Borrowed(&ANSWER));
assert_eq!(MESSAGE, "hello");
assert!(core::ptr::eq(STATIC.into_owned(), &ANSWER));
```

## Heapless storage

Enable the optional `heapless` feature to use `Heapless<N>`. It works without
`std`, `alloc`, or a global allocator, and leaves `DefaultPolicy` unchanged:

```toml
[dependencies]
cower = { version = "0.1.1", default-features = false, features = ["heapless"] }
```

```rust,ignore
use cower::{Cow, Heapless};

let mut text: Cow<'_, str, Heapless<16>> = Cow::borrowed("hello");
text.try_to_mut().unwrap().push('!').unwrap();
assert_eq!(text, "hello!");

let source = [1, 2, 3];
let mut bytes: Cow<'_, [u8], Heapless<4>> = Cow::from(&source);
bytes.try_to_mut().unwrap().push(4).unwrap();
assert_eq!(bytes.try_into_owned().unwrap().as_slice(), &[1, 2, 3, 4]);
```

`N` counts UTF-8 bytes for strings and elements for slices. Borrowed values may
exceed that capacity; conversion to owned storage returns `heapless::CapacityError`
if they do. Failed `try_to_mut` calls preserve the original borrow. Slice promotion
requires `T: Clone`, while borrowing and moving existing owned vectors do not.
Use `.into()` to move a `heapless::String<N>` or `heapless::Vec<T, N>` into its
corresponding `Cow`, or `Cow::from(&container)` to borrow one.

## Third-party storage

All policies are uninhabited type markers. Owned container conversions move the
existing storage; conversions from references borrow the contents under any
compatible policy. The default policy is unchanged by these features.

| Feature (short alias) | Policy | Borrowed → owned | Requirements |
| --- | --- | --- | --- |
| `bytes_1` (`bytes`) | `BytesPolicy` | `[u8]` → `bytes::BytesMut` | Allocator, pointer atomics |
| `smallvec_1` (`smallvec`) | `SmallVecPolicy<N>` | `[T]` → `SmallVec<[T; N]>` | Allocator; spills beyond `N` |
| `arrayvec_0_7` (`arrayvec`) | `ArrayVecPolicy<N>` | `str` / `[T]` → `ArrayString<N>` / `ArrayVec<T, N>` | No allocator |
| `compact_str_0_9` (`compact_str`) | `CompactStrPolicy` | `str` → `CompactString` | Allocator; short strings inline |
| `ecow_0_3` (`ecow`) | `EcoPolicy` | `str` / `[T]` → `EcoString` / `EcoVec<T>` | Allocator, pointer atomics; shared heap storage |
| `ecow_0_3` (`ecow`) | `EcoBytesPolicy` | `[u8]` → `EcoBytes` | Allocator, pointer atomics; inline or shared bytes |
| `arcstr_1` (`arcstr`) | `ArcStrPolicy` | `str` → `ArcStr` | Allocator, pointer atomics; shared immutable text |
| `arcstr_1` (`arcstr`) | `ArcSubstrPolicy` | `str` → `Substr` | Allocator, pointer atomics; shared substring views |
| `smol-bytes_0_2` (`smol-bytes`) | `SmolBytesBufferPolicy` | `[u8]` / `str` → `Buffer` / `Utf8Buffer` | No allocator; 62 bytes inline |
| `smol-bytes_0_2` (`smol-bytes`) | `SmolBytesPolicy` | `[u8]` / `str` → shared `Bytes` / `Utf8Bytes` | cower `std` or `alloc`; pointer atomics |
| `smol-bytes_0_2` (`smol-bytes`) | `SmolBytesCompactPolicy` | `[u8]` / `str` → compact `Bytes` / `Utf8Bytes` | cower `std` or `alloc`; pointer atomics |
| `smol-bytes_0_2` (`smol-bytes`) | `SmolBytesMutPolicy` | `[u8]` / `str` → `BytesMut` / `Utf8BytesMut` | cower `std` or `alloc`; pointer atomics |

Growable policies support `to_mut` / `into_owned` for local borrows. Shared and
compact smol-bytes storage retain their immutable data APIs and cheap clones;
choose `SmolBytesMutPolicy` for growable edits. Fixed storage uses `try_to_mut` /
`try_into_owned`, returning native capacity errors. String capacities count UTF-8
bytes. Borrows may exceed the owned capacity; a failed promotion preserves them.
Slice promotion requires `T: Clone`; moving existing vector storage does not.

```rust,ignore
use cower::{Cow, SmolBytesMutPolicy};

let source = String::from("hello");
let mut text: Cow<'_, str, SmolBytesMutPolicy> = Cow::borrowed(&source);
text.to_mut().push_str(" world");
assert_eq!(text, "hello world");
assert_eq!(source, "hello");
```

With `bytes`, consuming a unique `bytes::Bytes` reuses its backing allocation;
shared or static bytes are copied into mutable storage. Moving an existing
`BytesMut` preserves its buffer. `BytesPolicy` and `CompactStrPolicy` also reuse
capacity in `ToOwned::clone_into`, as do the EcoString and EcoBytes policies.

Ecow clones share heap storage; `EcoString` / `EcoBytes` also inline short data.
Use the owned containers' mutation APIs (including `make_mut`) to detach shared
storage as needed. `ArcStrPolicy` preserves static `arcstr::literal!` values and
shared allocations; `ArcSubstrPolicy` preserves a substring's parent and range.
Arcstr storage is immutable: `to_mut` can replace its handle, but does not expose
mutable string bytes.

## Optional integrations

Each integration is enabled independently and leaves the default policy's
storage selection unchanged. `std` and `alloc` forward capabilities to enabled
dependencies without activating them. Each integration has a versioned feature
and a short alias: `serde = ["serde_1"]`, `rkyv = ["rkyv_0_8"]`,
`heapless = ["heapless_0_9"]`, and similarly for the others. Enable the versioned
name to select a specific compatibility series. The dependency key is also
versioned (`serde-core_1`, with `package = "serde_core"`), with optional dependencies
and their defaults disabled. Existing short aliases remain supported.

Integration features do not turn on cower's `std` or `alloc` tier implicitly.
Some dependencies still require an allocator or `std`; the table records those
requirements. In particular, `smol-bytes` alone keeps only the fixed buffer policy
available, while adding cower's `alloc` or `std` exposes its growable policies.

| Feature (short alias) | Support | Dependency requirements |
| --- | --- | --- |
| `serde_1` (`serde`) | `Serialize` / `Deserialize` through `serde_core`, without derive macros | Works without `std` or `alloc` |
| `rkyv_0_8` (`rkyv`) | `Archive` / `Serialize` / `Deserialize`, checked zero-copy archive access | Core API works without `std` or `alloc` |
| `quickcheck_1` (`quickcheck`) | Generate owned values and shrink owned storage | Dependency requires `std` |
| `proptest_1` (`proptest`) | Owned strategies, parameters, and shrinking | Dependency requires `alloc`; `std` enables its standard runner facilities |
| `arbitrary_1` (`arbitrary`) | Owned fuzz-input generation, take-rest, and recursion-guarded size hints | Dependency requires `std` |
| `schemars_1` (`schemars`) | `JsonSchema` matches `T`, including schema IDs and optional-field behavior | Requires an allocator and pointer atomics, supports `no_std` |
| `borsh_1` (`borsh`) | Content-only `BorshSerialize` and owned `BorshDeserialize` | Requires an allocator, supports `no_std` |
| `bytemuck_1` (`bytemuck`) | Transparent conversions between Cow and its actual Storage | Core API needs no allocator; `std` / `alloc` enable allocated wrapper conversions |
| `zerocopy_0_8` (`zerocopy`) | Checked content views, byte access, derived `KnownLayout` / conditional `Immutable` | Core API needs no allocator; derive macros run on the host |

Serde serializes `T`'s contents with no policy or variant tag. Like standard
`Cow`, deserialization creates `Owned`, requiring `T::Owned: Deserialize<'de>`.
Enabling both `heapless` and `serde` also enables the container implementations;
oversized input returns a deserialization error. Static-reference storage still
requires static input and cannot extend the lifetime of local input. Serde's
special derive handling for standard `Cow` does not make this wrapper borrow
automatically when annotated with `#[serde(borrow)]`.

Borsh follows the same content-only / owned-decoding model, without a variant
or policy tag. Owned deserialization requires `T::Owned: BorshDeserialize`.
Combined features enable native implementations for `arrayvec`, `bytes`,
`compact_str`, and `smol-bytes`. `heapless`, `smallvec`, `ecow`, and `arcstr`
support Borsh serialization through their borrowed contents, but their owned
containers do not supply Borsh deserialization.

Schemars forwards to the borrowed type, without requiring schema traits on the
policy or owned storage. It does not add a capacity limit to the schema, since
borrowed values may exceed that capacity. Deserializers still enforce limits.

Serde forwards to all enabled storage integrations. Generator support is
forwarded where upstream implements it: QuickCheck for `compact_str` and
`smol-bytes`; proptest for `compact_str`; arbitrary for `smallvec`, `compact_str`,
and `smol-bytes`. Upstream `compact_str` enables `std` with its Borsh and proptest
features. Arrayvec 0.7's Borsh implementation also needs its `std` prelude,
which this integration enables for that combination. These combinations are not
bare-metal configurations; base ArrayVec storage and Serde remain core-compatible.

The three generation integrations require their respective `Arbitrary` trait on
`T::Owned`. They do not require framework traits on the policy itself. QuickCheck
also requires cloneable backing and produces `Cow<'static, T, P>`; manually
constructed borrowed values have no shrink candidates. Proptest forwards the
owned type's `Parameters` and strategy. `heapless` 0.9 does not implement these
generator traits on its containers; use an explicit bounded strategy mapped to
`Cow::owned` for `Heapless<N>`.

### bytemuck and zerocopy

`Cow<'a, T, P>` implements `bytemuck::TransparentWrapper<T::Storage<'a>>`.
`wrap` / `peel`, reference conversions, and slice conversions preserve the exact
backing storage without cloning. With `std` or `alloc`, the upstream
`TransparentWrapperAlloc` blanket implementation additionally supports containers
such as `Box` and `Vec`, preserving their allocations. The wrapper adds no validity
invariants to its field, and reference conversions retain both the outer borrow
and the inner data lifetime.

```rust,ignore
use bytemuck::TransparentWrapper;
use cower::{Cow, CowRepr, StaticPolicy};

let storage = CowRepr::Borrowed("hello");
let view = Cow::<'_, str, StaticPolicy>::wrap_ref(&storage);
assert_eq!(view.as_ref(), "hello");
assert!(core::ptr::eq(Cow::peel_ref(view), &storage));
```

Zerocopy APIs operate on the **contents**, not on the Cow handle's pointers or
variant. `try_from_bytes`, `try_from_prefix`, and `try_from_suffix` check size,
alignment, and validity, then construct a borrowed Cow tied to the input's
lifetime. Their `_with_elems` counterparts accept an explicit trailing-element
count for slice DSTs, including zero-sized elements. Original zerocopy errors
are returned unchanged. Owned conversion is not required for borrowing.

For a generic raw content view, import `zerocopy::IntoBytes` and use
`value.as_ref().as_bytes()` when `T: IntoBytes + Immutable`. There is no separate
Cow-specific forwarding method. This replaces the generic `Cow::as_bytes`
provided by the zerocopy integration in 0.1.1; the inherent const accessors
for strings and byte slices remain available independently of zerocopy.
`to_mut_bytes` / `try_to_mut_bytes` first promote to owned storage and require
`T: IntoBytes + FromBytes` and `Owned: BorrowMut<T>`. This prevents unrestricted
byte edits to types such as `str` and `bool`. A failed fallible promotion preserves
the borrow. Mutable access may clone or allocate, including detaching shared
storage; exclusively owned storage is reused. The native layout and byte order
apply; these views are not a portable serialization format.

```rust,ignore
use cower::Cow;
use zerocopy::IntoBytes;

let original = [1u16, 2];
let mut value: Cow<'_, [u16]> = Cow::try_from_bytes(original.as_bytes()).unwrap();
assert!(value.is_borrowed());
assert_eq!(value.as_ref().as_bytes(), original.as_bytes());
value.to_mut_bytes()[..2].copy_from_slice(&42u16.to_ne_bytes());
assert_eq!(value.as_ref(), &[42, 2]);
assert_eq!(original, [1, 2]);
```

`Cow` and `CowRepr` derive `KnownLayout` and conditionally `Immutable` through
zerocopy's official macros, using actual field bounds rather than bounds on the
policy marker. `KnownLayout` does not promise a stable wire layout. The Cow
handle itself receives no `Pod`, `Zeroable`, `FromBytes`, or `IntoBytes`
implementation: content traits do not establish the validity of a handle's
pointers, discriminant, or unused bytes.

### rkyv archives

Sized values, strings, and slices archive into `ArchivedCow<T::Archived>`,
`ArchivedCow<ArchivedString>`, and `ArchivedCow<ArchivedVec<T::Archived>>`
respectively. The archived wrapper dereferences to its contents and implements
`CheckBytes`. The format is independent of the policy and borrowed/owned variant.

```rust,ignore
use cower::Cow;
use rkyv::rancor::Error;

let value: Cow<'_, str> = Cow::borrowed("hello");
let bytes = rkyv::to_bytes::<Error>(&value).unwrap();
let archived = rkyv::access::<rkyv::Archived<Cow<'_, str>>, Error>(&bytes).unwrap();
assert_eq!(archived.as_str(), "hello");
let restored: Cow<'static, str> = rkyv::from_bytes::<_, Error>(&bytes).unwrap();
assert!(restored.is_owned());
assert_eq!(restored, "hello");
```

Deserialization converts the decoded value into the policy's owned type with
`TryFrom`, returning errors such as insufficient `Heapless<N>` capacity. String
conversion accepts any `&str`, so it cannot manufacture static references.
With `std`/`alloc`, slice deserialization uses a temporary `Vec<T>`; without
either feature, `Heapless<N>` slices deserialize directly into inline storage.
Other policies deserialize when their owned types implement the required
`TryFrom`: strings need conversion from `&str`, slices from `Vec<T>` in the
heap tiers. For example, `ArrayVec` slices lack the latter conversion and support
archiving/reading but not owned rkyv deserialization. Without cower's `std` or
`alloc`, direct inline slice deserialization is provided only for `Heapless<N>`.
The `rkyv::api::low` API supports stack-backed output and scratch buffers.
Archived reads do not deserialize or allocate.

## Custom policies

`ToOwned<P>` selects both `Owned` and a lifetime-parameterized `Storage` type.
Custom policies normally choose the supplied `CowRepr` backing enum:

```rust,ignore
use cower::{Cow, CowRepr, ToOwned};

struct Inline<const N: usize>;

impl<const N: usize> ToOwned<Inline<N>> for str {
    type Owned = heapless::String<N>;
    type Storage<'a> = CowRepr<'a, Self, Inline<N>>;
}

impl<T, const N: usize> ToOwned<Inline<N>> for [T] {
    type Owned = heapless::Vec<T, N>;
    type Storage<'a> = CowRepr<'a, Self, Inline<N>> where T: 'a;
}

let mut text: Cow<'_, str, Inline<16>> = Cow::borrowed("hello");
text.try_to_mut().unwrap().push('!').unwrap();
assert_eq!(text, "hello!");

let mut too_long: Cow<'_, str, Inline<4>> = Cow::borrowed("hello");
assert!(too_long.try_to_mut().is_err());
assert!(too_long.is_borrowed());
```

For `heapless` storage, the built-in policy above avoids these custom implementations.
`CowStorage` describes the storage interface, and `OwnedStorage`
adds infallible conversion. Both are already implemented for the supplied
backings; custom policies only need to select `Storage`.

## Construction and pattern matching

Use `Cow::borrowed` and `Cow::owned` for any policy. The default policy also has
const `Cow::Borrowed` and `Cow::Owned` constructors. For const construction with a
custom policy, use `Cow::from_storage` with that policy's backing type.

`Cow` is a struct: `Cow::Borrowed` and `Cow::Owned` are constructor functions,
not patterns. Match the explicit enum returned by `into_repr`, inspect
`as_borrowed` / `as_owned`, or match `as_std()` for standard backing:

```rust
use cower::{Cow, CowRepr};
let text: Cow<'_, str> = Cow::Borrowed("hello");
match text.into_repr() {
    CowRepr::Borrowed(value) => assert_eq!(value, "hello"),
    CowRepr::Owned(_) => unreachable!(),
}
```

## Conversion and compatibility

The default heap backing delegates `to_mut`, `into_owned`, `Clone`, and
`clone_from` to standard `Cow`, including custom standard `ToOwned`
implementations whose owned type implements neither `Clone` nor `From<&T>`.
`CowRepr` uses the policy's `to_owned` and its lifetime-aware `From` bound.
`try_to_mut` and `try_into_owned` use `TryFrom`, preserving capacity errors.
Static storage only permits promotion of an already-static borrow.

The policy trait itself also provides `to_owned` and `clone_into` with
`Owned: From<&'a Self>`. The default heap implementation forwards those calls to
standard `ToOwned`, retaining `clone_into`'s capacity reuse. Use qualified trait
calls to distinguish it from the standard prelude trait:

```rust
use cower::ToOwned;
let mut text = <str as ToOwned>::to_owned("old");
<str as ToOwned>::clone_into("new", &mut text);
assert_eq!(text, "new");
```

Implemented stable interfaces include `Default`, content comparisons, `Hash`,
formatting, owned and borrowed string/slice/C-string conversions, array borrows,
string `+` / `+=`, iterator collection, and conversions to `Box`, `Rc`, and `Arc`
(where pointer atomics exist). The `std` feature adds `Path` and `OsStr` families.
Allocation-dependent operations are unavailable for static storage.

Switching an import is sufficient for many concrete string and slice uses, and
reference arguments can coerce to standard `Cow`. This is not complete type
identity: by-value arguments need `.into()`, generic trait bounds do not apply
implicit coercions, and `Deref::Target` is the backing type rather than `T`.
Use `as_ref()` for `&T`. Generic policy inference may require a type annotation.
The wrapper's `is_borrowed` / `is_owned` methods are runtime methods; standard
nightly-only APIs are not part of the Rust 1.85 compatibility target.

## Safety and features

No independent `repr(Rust)` enums are cast to each other. Reference access and
deref use ordinary field borrows. The internal unsafe move implements
const `into_storage` on Rust 1.85: it uses `ManuallyDrop` and reads the initialized
field exactly once, preserving its exact type and preventing double-drop. Its
safety argument does not depend on standard `Cow`'s enum layout. The optional
bytemuck integration adds one audited `unsafe impl` of `TransparentWrapper`,
justified by the single field and `repr(transparent)`. Zerocopy uses upstream
safe APIs and official derives; byte access always refers to the contents.

The default feature is `std`. `std` and `alloc` are independent; either enables
default heap backing. Disable both and any integrations requiring heap/std for
a core-only build. The base library MSRV is Rust 1.85, including const conversions
to and reference access to standard `Cow`. Optional integrations can require a
newer compiler:

| Features | Minimum Rust version |
| --- | --- |
| Core-only, `alloc`, `std`, and integrations not listed below | 1.85 |
| `heapless` / `heapless_0_9` | 1.87 |
| `smol-bytes` / `smol-bytes_0_2` | 1.95 |
| `--all-features` | 1.95 |

Feature combinations require the highest applicable version. CI checks the library
at these floors across core, alloc, and std configurations. Development dependencies
may require newer Rust; tests and benchmarks run on stable Rust.

Tests cover feature configurations, capacity errors, static lifetime failures, const
values/references, direct reference coercions, allocation identity, and drops.

Miri CI checks all features under Tree Borrows, plus the bare core build and
core-only `heapless,rkyv` integration. Stacked Borrows checks all features except
`rkyv` / `rkyv_0_8`, plus the bare core build: rkyv's relative pointers rely on
the aliasing behavior supported by Tree Borrows, as described in
[the upstream explanation](https://github.com/rkyv/rkyv/issues/436#issuecomment-1767450952).
Both lanes retain strict provenance and aliasing checks.

```sh
cargo test
cargo test --no-default-features
cargo test --no-default-features --features alloc
cargo test --no-default-features --features heapless
cargo test --no-default-features --features heapless,serde,rkyv
cargo test --no-default-features --features bytemuck,zerocopy
cargo test --all-features
cargo run --example inline --no-default-features --features heapless
```

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

[Github-url]: https://github.com/al8n/cower/
[CI-url]: https://github.com/al8n/cower/actions/workflows/ci.yml
[doc-url]: https://docs.rs/cower
[crates-url]: https://crates.io/crates/cower
[codecov-url]: https://app.codecov.io/gh/al8n/cower/
