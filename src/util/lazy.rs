pub type Lazy<T> = once_cell::sync::Lazy<T>;
pub type AsyncLazyInner<T> = async_oncecell::Lazy<T>;
pub type AsyncLazy<T> = Lazy<AsyncLazyInner<T>>;

#[macro_export]
macro_rules! AsyncLazyNew {
    ($e:expr) => {
        crate::util::lazy::AsyncLazy::new(|| {
            crate::util::lazy::AsyncLazyInner::new(async { $e.await })
        })
    };
}

#[macro_export]
macro_rules! LazyNew {
    ($e:expr) => {
        crate::util::lazy::Lazy::new(|| $e)
    };
}
