#![warn(clippy::pedantic)]

use core::{borrow::Borrow, ops::Deref};
use std::{fmt::Display, hash::Hash};

#[derive(Debug)]
pub enum Woc<'a, T, R: ?Sized> {
	Owned(T),
	Borrowed(&'a R),
}

impl<'a, T: From<&'a R>, R: ?Sized> Woc<'a, T, R> {
	pub fn into_owned(self) -> T {
		match self {
			Woc::Owned(t) => t,
			Woc::Borrowed(r) => r.into(),
		}
	}

	pub fn is_borrowed(&self) -> bool {
		match self {
			Woc::Owned(_) => false,
			Woc::Borrowed(_) => true,
		}
	}

	pub fn is_owned(&self) -> bool {
		match self {
			Woc::Owned(_) => true,
			Woc::Borrowed(_) => false,
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

impl<'a, T: AsRef<R>, R: ?Sized> Deref for Woc<'a, T, R> {
	type Target = R;
	fn deref(&self) -> &Self::Target {
		match self {
			Woc::Owned(t) => t.as_ref(),
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

impl<'a, T: Default, R: ?Sized> Default for Woc<'a, T, R> {
	fn default() -> Self {
		Woc::Owned(T::default())
	}
}

impl<'a, T: AsRef<R>, R: PartialEq + ?Sized> PartialEq<R> for Woc<'a, T, R> {
	fn eq(&self, other: &R) -> bool {
		self.as_ref() == other
	}
}

impl<'a, T: AsRef<R>, R: PartialEq + ?Sized> PartialEq for Woc<'a, T, R> {
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: AsRef<R>, R: Eq + ?Sized> Eq for Woc<'a, T, R> {}

impl<'a, T: Hash, R: Hash + ?Sized> Hash for Woc<'a, T, R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Woc::Owned(t) => t.hash(state),
			Woc::Borrowed(r) => r.hash(state),
		}
	}
}
