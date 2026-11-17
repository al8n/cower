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
The library has no required dependencies; the optional `heapless` feature adds
fixed-capacity inline storage.

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
Rust 1.87. Reference access borrows the actual field; it does not reinterpret a
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
cower = { version = "0.0.0", default-features = false, features = ["heapless"] }
```

```rust
# #[cfg(feature = "heapless")]
# {
use cower::{Cow, Heapless};

let mut text: Cow<'_, str, Heapless<16>> = Cow::borrowed("hello");
text.try_to_mut().unwrap().push('!').unwrap();
assert_eq!(text, "hello!");

let source = [1, 2, 3];
let mut bytes: Cow<'_, [u8], Heapless<4>> = Cow::from(&source);
bytes.try_to_mut().unwrap().push(4).unwrap();
assert_eq!(bytes.try_into_owned().unwrap().as_slice(), &[1, 2, 3, 4]);
# }
```

`N` counts UTF-8 bytes for strings and elements for slices. Borrowed values may
exceed that capacity; conversion to owned storage returns `heapless::CapacityError`
if they do. Failed `try_to_mut` calls preserve the original borrow. Slice promotion
requires `T: Clone`, while borrowing and moving existing owned vectors do not.
Use `.into()` to move a `heapless::String<N>` or `heapless::Vec<T, N>` into its
corresponding `Cow`, or `Cow::from(&container)` to borrow one.

## Custom policies

`ToOwned<P>` selects both `Owned` and a lifetime-parameterized `Storage` type.
Custom policies normally choose the supplied `CowRepr` backing enum:

```rust
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
nightly-only APIs are not part of the Rust 1.87 compatibility target.

## Safety and features

No independent `repr(Rust)` enums are cast to each other. Reference access and
deref use ordinary field borrows. The one internal unsafe operation implements
const `into_storage` on Rust 1.85: it uses `ManuallyDrop` and reads the initialized
field exactly once, preserving its exact type and preventing double-drop. Its
safety argument does not depend on standard `Cow`'s enum layout.

The default feature is `std`. `std` and `alloc` are independent; either enables
default heap backing. Disable both for a core-only build. The library MSRV is
Rust 1.87, including the optional `heapless` 0.9.3 dependency. Tests cover feature
configurations, capacity errors, static lifetime failures, const
values/references, direct reference coercions, allocation identity, and drops.

```sh
cargo test
cargo test --no-default-features
cargo test --no-default-features --features alloc
cargo test --no-default-features --features heapless
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
