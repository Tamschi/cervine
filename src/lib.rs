#![no_std]
#![warn(clippy::pedantic)]
#![doc(html_root_url = "https://docs.rs/cervine/0.0.2")]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

use core::{
	borrow::Borrow,
	cmp::Ordering,
	convert::{TryFrom, TryInto as _},
	fmt::{self, Display, Formatter},
	hash::{Hash, Hasher},
	ops::Deref,
};

#[cfg(feature = "serde")]
use serde::{de, ser};

#[derive(Debug, Copy)]
pub enum Cow<'a, T, R: ?Sized> {
	Owned(T),
	Borrowed(&'a R),
}

impl<'a, T, R: ?Sized> Cow<'a, T, R> {
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

impl<'a, T: From<&'a R>, R: ?Sized> Cow<'a, T, R> {
	pub fn into_owned(self) -> T {
		match self {
			Cow::Owned(t) => t,
			Cow::Borrowed(r) => r.into(),
		}
	}

	pub fn make_mut(&mut self) -> &mut T {
		match self {
			Cow::Owned(t) => t,
			Cow::Borrowed(r) => {
				*self = Cow::Owned((*r).into());
				match self {
					Cow::Owned(t) => t,
					Cow::Borrowed(_) => unreachable!(),
				}
			}
		}
	}
}

impl<'a, T: TryFrom<&'a R>, R: ?Sized> Cow<'a, T, R> {
	pub fn try_into_owned(self) -> Result<T, T::Error> {
		match self {
			Cow::Owned(t) => Ok(t),
			Cow::Borrowed(r) => r.try_into(),
		}
	}

	pub fn try_make_mut(&mut self) -> Result<&mut T, T::Error> {
		match self {
			Cow::Owned(t) => Ok(t),
			Cow::Borrowed(r) => {
				*self = Cow::Owned((*r).try_into()?);
				match self {
					Cow::Owned(t) => Ok(t),
					Cow::Borrowed(_) => unreachable!(),
				}
			}
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

impl<'a, T: AsRef<R>, R: Display + ?Sized> Display for Cow<'a, T, R> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.as_ref().fmt(f)
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

impl<'a, T: AsRef<R>, R: PartialOrd + ?Sized> PartialOrd<R> for Cow<'a, T, R> {
	fn partial_cmp(&self, other: &R) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
	fn lt(&self, other: &R) -> bool {
		self.as_ref().lt(other)
	}
	fn le(&self, other: &R) -> bool {
		self.as_ref().le(other)
	}
	fn gt(&self, other: &R) -> bool {
		self.as_ref().gt(other)
	}
	fn ge(&self, other: &R) -> bool {
		self.as_ref().ge(other)
	}
}

impl<'a, T: AsRef<R>, R: PartialOrd + ?Sized> PartialOrd for Cow<'a, T, R> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.as_ref().partial_cmp(other.as_ref())
	}
	fn lt(&self, other: &Self) -> bool {
		self.as_ref().lt(other.as_ref())
	}
	fn le(&self, other: &Self) -> bool {
		self.as_ref().le(other.as_ref())
	}
	fn gt(&self, other: &Self) -> bool {
		self.as_ref().gt(other.as_ref())
	}
	fn ge(&self, other: &Self) -> bool {
		self.as_ref().ge(other.as_ref())
	}
}

impl<'a, T: AsRef<R>, R: Ord + ?Sized> Ord for Cow<'a, T, R> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_ref().cmp(other.as_ref())
	}

	// min, max and clamp handled by default implementation, since they can't be forwarded directly.
}

impl<'a, T: AsRef<R>, R: Hash + ?Sized> Hash for Cow<'a, T, R> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_ref().hash(state)
	}
}

#[cfg(feature = "serde")]
impl<'a, T: ser::Serialize, R: ser::Serialize + ?Sized> ser::Serialize for Cow<'a, T, R> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: ser::Serializer,
	{
		match self {
			Cow::Owned(t) => t.serialize(serializer),
			Cow::Borrowed(r) => r.serialize(serializer),
		}
	}
}

#[cfg(feature = "serde")]
impl<'de, T: de::Deserialize<'de>, R: ?Sized> de::Deserialize<'de> for Cow<'de, T, R> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		T::deserialize(deserializer).map(Cow::Owned)
	}

	fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		match place {
			Cow::Owned(t) => T::deserialize_in_place(deserializer, t),
			Cow::Borrowed(_) => serde::Deserialize::deserialize(deserializer).map(|de| *place = de),
		}
	}
}
