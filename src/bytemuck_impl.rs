use crate::{Cow, ToOwned};
use bytemuck_1::TransparentWrapper;

/// Reinterprets Cow and its exact backing storage, preserving ownership and lifetimes.
///
/// This also supports slices of wrappers. With cower's `std` or `alloc`,
/// bytemuck's `TransparentWrapperAlloc` blanket implementation additionally
/// supports allocated containers. No byte-casting traits such as `Pod` or
/// `Zeroable` are supplied: the borrowed contents being POD does not make
/// their containing storage POD.
///
/// ```compile_fail
/// fn needs_pod<T: bytemuck_1::Pod>() {}
/// needs_pod::<cower::Cow<'static, [u8]>>();
/// ```
///
/// Wrapper references remain tied to the borrow of the storage:
///
/// ```compile_fail
/// use bytemuck_1::TransparentWrapper;
/// use cower::{Cow, CowRepr, StaticPolicy};
/// fn extend(storage: &CowRepr<'static, str, StaticPolicy>)
///     -> &'static Cow<'static, str, StaticPolicy>
/// {
///     Cow::wrap_ref(storage)
/// }
/// ```
///
/// Moving a wrapper also preserves the lifetime of any contained borrow:
///
/// ```compile_fail
/// use bytemuck_1::TransparentWrapper;
/// use cower::{Cow, CowRepr, StaticPolicy};
/// fn extend(storage: CowRepr<'_, str, StaticPolicy>) -> Cow<'static, str, StaticPolicy> {
///     Cow::wrap(storage)
/// }
/// ```
// SAFETY: Cow is repr(transparent) over T::Storage<'a>. The only other field is
// a zero-sized, alignment-one PhantomData marker. It adds no validity restrictions:
// from_storage accepts every valid Storage. The wrapper has no custom Drop.
// The trait defaults preserve the exact storage type and its lifetime parameter;
// no independently defined enums are reinterpreted. No methods are overridden.
#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck_1")))]
unsafe impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> TransparentWrapper<T::Storage<'a>>
  for Cow<'a, T, P>
{
}
