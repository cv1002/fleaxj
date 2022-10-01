/* Trait declaration */
pub trait ResultInspect<F: FnOnce(&T), T: Sized> {
    /// Call function when ok.
    fn inspect_ok(self, f: F) -> Self;
}
pub trait ResultInspectRef<F: FnOnce(&T), T: Sized> {
    /// Call function when ok.
    fn inspect_ref(&self, f: F);
}
pub trait ResultInspectErr<F: FnOnce(&E), E: Sized> {
    /// Call function when error.
    fn inspect_error(self, f: F) -> Self;
}
pub trait ResultInspectErrRef<F: FnOnce(&E), E: Sized> {
    /// Call function when error, but doesn't move.
    fn inspect_err_ref(&self, f: F);
}
pub trait OptionInspect<F: FnOnce(&T), T: Sized> {
    /// Call function when some.
    fn inspect_some(self, f: F) -> Self;
}
pub trait OptionInspectRef<F: FnOnce(&T), T: Sized> {
    /// Call function when some, but doesn't move.
    fn inspect_ref(&self, f: F);
}
pub trait OptionInspectNone<F: FnOnce()> {
    /// Call function when none.
    fn inspect_none(self, f: F) -> Self;
}
pub trait OptionInspectNoneRef<F: FnOnce()> {
    /// Call function when none, but doesn't move.
    fn inspect_none_ref(&self, f: F);
}

/* Trait implement */
impl<F: FnOnce(&T), T: Sized, E> ResultInspect<F, T> for Result<T, E> {
    fn inspect_ok(self, f: F) -> Self {
        if let Ok(ref o) = self.as_ref() {
            f(&o);
        }

        self
    }
}

impl<F: FnOnce(&T), T: Sized, E> ResultInspectRef<F, T> for Result<T, E> {
    fn inspect_ref(&self, f: F) {
        if let Ok(ref o) = self {
            f(&o);
        }
    }
}


impl<F: FnOnce(&E), T, E: Sized> ResultInspectErr<F, E> for Result<T, E> {
    fn inspect_error(self, f: F) -> Self {
        if let Err(ref e) = self.as_ref() {
            f(&e);
        }

        self
    }
}

impl<F: FnOnce(&E), T, E: Sized> ResultInspectErrRef<F, E> for Result<T, E> {
    fn inspect_err_ref(&self, f: F) {
        if let Err(ref e) = self {
            f(&e);
        }
    }
}

impl<F: FnOnce(&T), T: Sized> OptionInspect<F, T> for Option<T> {
    fn inspect_some(self, f: F) -> Self {
        if let Some(ref o) = self.as_ref() {
            f(&o);
        }

        self
    }
}

impl<F: FnOnce(&T), T: Sized> OptionInspectRef<F, T> for Option<T> {
    fn inspect_ref(&self, f: F) {
        if let Some(ref o) = self {
            f(&o);
        }
    }
}

impl<F: FnOnce(), T> OptionInspectNone<F> for Option<T> {
    fn inspect_none(self, f: F) -> Self {
        if let None = self.as_ref() {
            f();
        }

        self
    }
}

impl<F: FnOnce(), T> OptionInspectNoneRef<F> for Option<T> {
    fn inspect_none_ref(&self, f: F) {
        if let None = self {
            f();
        }
    }
}
