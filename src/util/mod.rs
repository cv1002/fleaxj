pub mod inspect;
pub mod lazy;

pub fn try_do<R>(f: impl FnOnce() -> R) -> R {
    f()
}
