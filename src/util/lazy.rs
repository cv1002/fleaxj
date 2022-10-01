/// Lazy initialized value, initialize when first used.
pub type Lazy<T> = once_cell::sync::Lazy<T>;
/// Lazy async initialized value, initialize when first used.
pub type AsyncLazyInner<T> = async_oncecell::Lazy<T>;
/// Lazy async initialized value, initialize when first used.
pub type AsyncLazy<T> = Lazy<AsyncLazyInner<T>>;

/// Construct an AsyncLazy<T>
/// # Examples
///
/// ```
/// use fleaxj::{util::lazy::AsyncLazy, AsyncLazyNew};
///
/// static temp: AsyncLazy<i32> = AsyncLazyNew!(async { 1 });
///
/// async fn test() {
///     assert!(temp.get().await.eq(&1));
/// }
/// test();
/// ```
#[macro_export]
macro_rules! AsyncLazyNew {
    ($e:expr) => {
        $crate::util::lazy::AsyncLazy::new(|| {
            $crate::util::lazy::AsyncLazyInner::new(async { $e.await })
        })
    };
}
/// Construct an Lazy<T>
/// # Examples
///
/// ```
/// use std::ops::Deref;
/// use fleaxj::{util::lazy::Lazy, LazyNew};
///
/// static temp: Lazy<i32> = LazyNew!(1);
///
/// fn test() {
///     assert!(temp.eq(&1));
/// }
/// test();
/// ```
#[macro_export]
macro_rules! LazyNew {
    ($e:expr) => {
        $crate::util::lazy::Lazy::new(|| $e)
    };
}
