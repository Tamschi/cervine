#![warn(clippy::pedantic)]

use core::{borrow::Borrow, ops::Deref};
use std::{fmt::Display, hash::Hash};

#[derive(Debug)]
pub enum Cow<'a, T, R: ?Sized> {
	Owned(T),
	Borrowed(&'a R),
}

impl<'a, T: From<&'a R>, R: ?Sized> Cow<'a, T, R> {
	pub fn into_owned(self) -> T {
		match self {
			Cow::Owned(t) => t,
			Cow::Borrowed(r) => r.into(),
		}
	}

	pub fn is_borrowed(&self) -> bool {
		match self {
			Cow::Owned(_) => false,
			Cow::Borrowed(_) => true,
		}
	}

	pub fn is_owned(&self) -> bool {
		match self {
			Cow::Owned(_) => true,
			Cow::Borrowed(_) => false,
		}
	}
}

impl<'a, T: AsRef<R>, R: ?Sized> AsRef<R> for Cow<'a, T, R> {
	fn as_ref(&self) -> &R {
		match self {
			Cow::Owned(t) => t.as_ref(),
			Cow::Borrowed(r) => r,
		}
	}
}

impl<'a, T: AsRef<R>, R: ?Sized> Deref for Cow<'a, T, R> {
	type Target = R;
	fn deref(&self) -> &Self::Target {
		match self {
			Cow::Owned(t) => t.as_ref(),
			Cow::Borrowed(r) => r,
		}
	}
}

impl<'a, T: Borrow<R>, R: ?Sized> Borrow<R> for Cow<'a, T, R> {
	fn borrow(&self) -> &R {
		match self {
			Cow::Owned(t) => t.borrow(),
			Cow::Borrowed(r) => r,
		}
	}
}

impl<'a, T: Display, R: ?Sized + Display> Display for Cow<'a, T, R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Cow::Owned(t) => t.fmt(f),
			Cow::Borrowed(r) => r.fmt(f),
		}
	}
}

impl<'a, T: Clone, R: ?Sized> Clone for Cow<'a, T, R> {
	fn clone(&self) -> Self {
		match self {
			Cow::Owned(t) => Self::Owned(t.clone()),
			Cow::Borrowed(r) => Self::Borrowed(r),
		}
	}
}

impl<'a, T: Default, R: ?Sized> Default for Cow<'a, T, R> {
	fn default() -> Self {
		Cow::Owned(T::default())
	}
}

impl<'a, T: AsRef<R>, R: PartialEq + ?Sized> PartialEq<R> for Cow<'a, T, R> {
	fn eq(&self, other: &R) -> bool {
		self.as_ref() == other
	}
}

impl<'a, T: AsRef<R>, R: PartialEq + ?Sized> PartialEq for Cow<'a, T, R> {
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: AsRef<R>, R: Eq + ?Sized> Eq for Cow<'a, T, R> {}

impl<'a, T: Hash, R: Hash + ?Sized> Hash for Cow<'a, T, R> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Cow::Owned(t) => t.hash(state),
			Cow::Borrowed(r) => r.hash(state),
		}
	}
}
