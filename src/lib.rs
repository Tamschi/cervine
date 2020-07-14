use std::fmt::Display;
use {
    core::{borrow::Borrow, ops::Deref},
    smartstring::alias::String,
    std::borrow::ToOwned,
};

#[derive(Debug, PartialEq)]
pub enum Woc<'a, T, R: ?Sized> {
    Owned(T),
    Borrowed(&'a R),
}

impl<'a, T: Borrow<R>, R: ToOwned<Owned = T>> Woc<'a, T, R> {
    fn into_owned(self) -> T {
        match self {
            Woc::Owned(t) => t,
            Woc::Borrowed(r) => r.to_owned(),
        }
    }
}

impl<'a, T: AsRef<R>, R: ?Sized> AsRef<R> for Woc<'a, T, R> {
    fn as_ref(&self) -> &R {
        match self {
            Woc::Owned(t) => t.as_ref(),
            Woc::Borrowed(r) => r,
        }
    }
}

impl<'a, T: Deref<Target = R>, R: ?Sized> Deref for Woc<'a, T, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        match self {
            Woc::Owned(t) => t,
            Woc::Borrowed(r) => r,
        }
    }
}

impl<'a, T: Borrow<R>, R: ?Sized> Borrow<R> for Woc<'a, T, R> {
    fn borrow(&self) -> &R {
        match self {
            Woc::Owned(t) => t.borrow(),
            Woc::Borrowed(r) => r,
        }
    }
}

impl<'a, T: Display, R: ?Sized + Display> Display for Woc<'a, T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Woc::Owned(t) => t.fmt(f),
            Woc::Borrowed(r) => r.fmt(f),
        }
    }
}

impl<'a, T: Clone, R: ?Sized> Clone for Woc<'a, T, R> {
    fn clone(&self) -> Self {
        match self {
            Woc::Owned(t) => Self::Owned(t.clone()),
            Woc::Borrowed(r) => Self::Borrowed(r),
        }
    }
}