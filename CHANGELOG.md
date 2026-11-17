# Changelog

All notable changes to this crate are documented here. The project follows semantic versioning;
before 1.0, a minor bump (0.x to 0.y) signals a breaking change.

New entries go under `## Unreleased`. A release renames that section to `## X.Y.Z (YYYY-MM-DD)` and
opens a new, empty `## Unreleased` above it. `ci/changelog.sh` checks this structure on every pull
request, and `release.yml` publishes the released section as the GitHub Release notes.

## Unreleased

- Make built-in policies uninhabited type-level markers, removing their value
  constructors and `Default` implementations.
- Add the optional `heapless` feature and `Heapless<N>` policy for inline strings
  and vectors, with fallible capacity checks and owned/borrowed container conversions.
- Raise the MSRV to Rust 1.87 to support `heapless` 0.9.3 as a library dependency.
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
