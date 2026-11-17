# Changelog

All notable changes to this crate are documented here. The project follows semantic versioning;
before 1.0, a minor bump (0.x to 0.y) signals a breaking change.

New entries go under `## Unreleased`. A release renames that section to `## X.Y.Z (YYYY-MM-DD)` and
opens a new, empty `## Unreleased` above it. `ci/changelog.sh` checks this structure on every pull
request, and `release.yml` publishes the released section as the GitHub Release notes.

## Unreleased

## 0.1.2 (2026-11-18)

- Add generic const `from_static` constructors for all policies selecting
  `CowRepr` or standard `Cow` storage, including custom and unsized policies.
  Borrowed storage needs no cloning, allocation, or inline-capacity checks.
- Keep `Cow<'a, T, P>` as the public type while selecting const implementations
  through a transparent wrapper with an explicit storage parameter. Document
  that backing type so the alias's method links remain publicly accessible.
- Add `as_str` / UTF-8 `as_bytes` for strings, `as_bytes` for `[u8]`, and
  `as_slice` for `[T]`. These support custom policies using standard storage or
  static-reference `CowRepr` storage. Heap access is const on Rust 1.87+;
  static-reference access is const on Rust 1.85, without a `Clone` bound.
- **Breaking (zerocopy):** remove the generic raw `Cow::as_bytes` forwarding
  method. Import `zerocopy::IntoBytes` and use `value.as_ref().as_bytes()` instead.
  Const string/byte accessors remain available independently of zerocopy.
- Check generic policy construction and string/slice access in downstream core,
  alloc, and std fixtures on each MSRV lane, alongside lifetime, layout, and
  allocation regression tests. The base MSRV remains 1.85.

## 0.1.1 (2026-11-17)

- Lower the base MSRV to Rust 1.85, including core-only, alloc, std, and const
  standard `Cow` conversions. Optional heapless 0.9 and smol-bytes 0.2 integrations
  require Rust 1.87 and 1.95 respectively.
- Check the base and optional integration MSRVs separately in CI across core,
  alloc, and std configurations, including combinations of compatible integrations.

## 0.1.0 (2026-11-17)

- Expand regression coverage for `CowRepr` ownership transitions and fallible conversions,
  symmetric comparisons, native OS strings, and third-party container conversions.
- Merge coverage from core, alloc, and std configurations, including doctests, and retain
  coverage reports as CI artifacts independently of Codecov uploads.
- Add optional bytemuck 1 transparent value, reference, slice, and allocated
  container conversions to the exact backing storage, with lifetime/drop checks.
- Add ecow 0.3 shared string/vector/byte policies and arcstr 1 shared string and
  substring policies, preserving native sharing and forwarding Serde support.
- Add zerocopy 0.8 checked content borrowing (including slice DST element counts),
  immutable byte views, and owned-only mutable byte views. Derive `KnownLayout`
  and field-bounded `Immutable` without marking Cow handles as raw byte data.

- Add optional Schemars 1 schema forwarding and Borsh 1 content serialization /
  policy-owned deserialization, including capacity validation.
- Add optional bytes, smallvec, arrayvec, compact_str, and smol-bytes 0.2 storage
  policies, with owned/borrowed conversions and upstream serialization/generation
  feature forwarding. Smol-bytes includes fixed, shared, compact, and mutable storage.
- Introduce versioned integration features and dependency aliases; retain short
  feature names as aliases, including `serde` → `serde_1` → `serde-core_1`.
- Raise the MSRV to Rust 1.95 for smol-bytes 0.2.

- Add optional `serde` support through `serde_core`, with content-only serialization
  and policy-owned deserialization, including `heapless` capacity checks.
- Add optional `quickcheck`, `proptest`, and `arbitrary` integrations delegating to
  owned generation, preserving shrinking, strategy parameters, and fuzz size hints.
- Add optional rkyv 0.8 archives with `ArchivedCow`, checked zero-copy reads,
  policy-independent formats, and fallible owned deserialization.
- Make built-in policies uninhabited type-level markers, removing their value
  constructors and `Default` implementations.
- Add the optional `heapless` feature and `Heapless<N>` policy for inline strings
  and vectors, with fallible capacity checks and owned/borrowed container conversions.
- Add transparent `Cow<'a, T, P>` with policy-selected owned and backing storage.
- Add `ToOwned<P>` and `DefaultPolicy`: standard owned types with `std` / `alloc`, and static
  references for only `str` and `[T]` without either feature.
- Add `StaticPolicy` with blanket static-reference storage regardless of enabled features.
- Support infallible and fallible conversion, copy-on-write mutation, and content-based traits.
- Add lifetime-checked `ToOwned::to_owned` and `clone_into`, forwarding the default heap policy
  to standard implementations and honoring overrides in `Cow`'s infallible conversions.
- Include downstream `heapless` policy examples and feature-matrix tests.
- Use standard `Cow` as the default heap backing, with safe shared/mutable reference coercions
  and const value/reference conversions. No casts between independent enum layouts are used.
- Expose `CowRepr` for pattern matching and custom backing; custom `ToOwned` implementations
  select `Storage<'a>`. Preserve const default constructors and add generic storage constructors.
- Add standard-compatible default values, string operators, iterator collection, comparisons,
  and string/slice/C-string/path/OS-string and shared-pointer conversions.
