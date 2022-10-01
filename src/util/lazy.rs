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
/// use fleaxj::AsyncLazyNew;
/// use fleaxj::util::lazy::AsyncLazyInner;
/// async fn test() {
///     let temp = AsyncLazyNew!(async { 1 });
///     assert_eq!(temp.get().await, &1)
/// }
/// test();
/// ```
#[macro_export]
macro_rules! AsyncLazyNew {
    ($e:expr) => {
        fleaxj::util::lazy::AsyncLazy::new(|| {
            fleaxj::util::lazy::AsyncLazyInner::new(async { $e.await })
        })
    };
}

#[macro_export]
macro_rules! LazyNew {
    ($e:expr) => {
        fleaxj::util::lazy::Lazy::new(|| $e)
    };
}
