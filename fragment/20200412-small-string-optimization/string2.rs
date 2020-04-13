//! String type.

use std::{
    borrow::Cow,
    convert::Infallible,
    fmt,
    iter::FromIterator,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
    rc::Rc,
    string::FromUtf8Error,
    sync::Arc,
};

/// Backend string type.
type BackendString = String;

/// Owned string type with optional small string optimization.
///
/// In some use cases, strings are usually short.
/// `id` attribute of HTML or keys of JSON objects are one of such cases.
///
/// This type is intended for use in such situation: when strings are expected to be very short.
/// However, this type can also contain a long string.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SmallString {
    /// Inner string.
    inner: BackendString,
}

// Some methods in `std::string::String` are not implemented, here is why:
//
//  * `from_utf8_lossy
//      + `Cow<str>` is tied to `String`.
//  * `from_utf16`, `from_utf16_lossy`
//      + Low priority.
//  * `from_raw_parts`
//      + `SmallString` type does not guarantee that the backend type can be
//        constructible from a raw pointer.
//  * `reserve_exact`
//      + `SmallString` type does not guarantee that the capacity of the backend
//        string type can be fully controlled.
//  * `as_mut_vec`
//      + `SmallString` does not guarantee that the internal representation is `Vec<u8>`.
//  * `drain`
//      + Is complex and maybe hard to implement correctly.
impl SmallString {
    /// Creates a new empty `SmallString`.
    // This is not `const` because I am unsure it can be guaranteed for other backends.
    #[inline]
    pub fn new() -> SmallString {
        Default::default()
    }

    /// Creates a new `SmallString` from the given backend value.
    #[inline(always)]
    fn from_inner(inner: BackendString) -> Self {
        Self { inner }
    }

    /// Creates a new empty `SmallString` with a particular capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_inner(BackendString::with_capacity(capacity))
    }

    /// Converts a vector of bytes to a `SmallString`.
    #[inline]
    pub fn from_utf8(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
        BackendString::from_utf8(vec).map(Self::from_inner)
    }

    /// Converts a vector of bytes to a `SmallString` without checking that the
    /// string contains valid UTF-8.
    ///
    /// # Safety
    ///
    /// `bytes` argument should be valid UTF-8 sequence.
    /// See the documentation for `std::string::String::from_utf8_unchecked()`.
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        Self::from_inner(BackendString::from_utf8_unchecked(bytes))
    }

    /// Converts a `SmallString` into a byte vector.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.inner.into_bytes()
    }

    /// Extracts a string slice containing the entire `SmallString`.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Converts a `SmallString` into a mutable string slice.
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.inner
    }

    /// Appends a given string slice onto the end of this `SmallString`.
    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.inner.push_str(string)
    }

    /// Returns this `SmallString`'s capacity, in bytes.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Ensures that this `SmallString`'s capacity is at least `additional` bytes
    /// larger than its length.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Shrinks the capacity of this `SmallString` to match its length.
    ///
    /// Note that this function does not guarantee the new capacity is exactly same as the length,
    /// because it can be impossible to control the internal buffer capacity.
    /// For example, inlined string has non-zero minimum capacity.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Appends the given `char` to the end of this `SmallString`.
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch)
    }

    /// Returns a byte slice of this `SmallString`'s contents.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Shortens this `SmallString` to the specified length.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len)
    }

    /// Removes the last character from the string buffer and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.inner.pop()
    }

    /// Removes a `char` from this `SmallString` at a byte position and returns it.
    #[inline]
    pub fn remove(&mut self, idx: usize) -> char {
        self.inner.remove(idx)
    }

    /// Retains only the characters specified by the predicate.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(char) -> bool,
    {
        self.inner.retain(f)
    }

    /// Inserts a character into this `SmallString` at a byte position.
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.inner.insert(idx, ch)
    }

    /// Inserts a string slice into this `SmallString` at a byte position.
    #[inline]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.inner.insert_str(idx, string)
    }

    /// Returns the length of this `SmallString`, in bytes, not `char`s or graphemes.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the `SmallString` has a length of zero, and `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Splits the string into two at the given index.
    #[inline]
    pub fn split_off(&mut self, at: usize) -> Self {
        Self::from_inner(self.inner.split_off(at))
    }

    /// Truncates this `SmallString`, removing all contents.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Removes the specified range in the string, and replaces it with the given string.
    ///
    /// The given string doesn't needo to be the same length as the range.
    #[inline]
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.inner.replace_range(range, replace_with)
    }

    /// Converts this `SmallString` into a `Box<str>`.
    #[inline]
    pub fn into_boxed_str(self) -> Box<str> {
        self.inner.into_boxed_str()
    }
}

impl std::ops::Add<&'_ str> for SmallString {
    type Output = Self;

    #[inline]
    fn add(self, other: &str) -> Self::Output {
        Self::from_inner(self.inner + other)
    }
}

impl std::ops::AddAssign<&'_ str> for SmallString {
    #[inline]
    fn add_assign(&mut self, other: &str) {
        self.inner += other;
    }
}

impl AsRef<[u8]> for SmallString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsRef<str> for SmallString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl std::borrow::Borrow<str> for SmallString {
    #[inline]
    fn borrow(&self) -> &str {
        self.inner.borrow()
    }
}

impl std::borrow::BorrowMut<str> for SmallString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.inner.borrow_mut()
    }
}

impl std::ops::Deref for SmallString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl std::ops::DerefMut for SmallString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl std::fmt::Display for SmallString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a> std::iter::Extend<&'a char> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a char>,
    {
        self.inner.extend(iter)
    }
}

impl<'a> std::iter::Extend<&'a str> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.inner.extend(iter)
    }
}

impl<'a> std::iter::Extend<Cow<'a, str>> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Cow<'a, str>>,
    {
        self.inner.extend(iter)
    }
}

impl std::iter::Extend<String> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.inner.extend(iter)
    }
}

impl std::iter::Extend<SmallString> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = SmallString>,
    {
        self.inner.extend(iter.into_iter().map(|s| s.inner))
    }
}

impl std::iter::Extend<char> for SmallString {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.inner.extend(iter)
    }
}

impl From<String> for SmallString {
    #[inline]
    fn from(s: String) -> Self {
        Self::from_inner(s)
    }
}

impl From<SmallString> for String {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner
    }
}

impl From<&'_ String> for SmallString {
    #[inline]
    fn from(s: &String) -> Self {
        Self::from_inner(s.into())
    }
}

impl From<&'_ str> for SmallString {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_inner(s.into())
    }
}

impl From<Box<str>> for SmallString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self::from_inner(s.into())
    }
}

impl From<SmallString> for Box<str> {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner.into()
    }
}

impl From<SmallString> for Arc<str> {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner.into()
    }
}

impl From<SmallString> for Rc<str> {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner.into()
    }
}

impl From<SmallString> for Cow<'_, str> {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner.into()
    }
}

impl From<SmallString> for Vec<u8> {
    #[inline]
    fn from(s: SmallString) -> Self {
        s.inner.into()
    }
}

impl<'a> FromIterator<&'a char> for SmallString {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a char>,
    {
        Self::from_inner(BackendString::from_iter(iter))
    }
}

impl<'a> FromIterator<Cow<'a, str>> for SmallString {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Cow<'a, str>>,
    {
        Self::from_inner(BackendString::from_iter(iter))
    }
}

impl FromIterator<String> for SmallString {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self::from_inner(BackendString::from_iter(iter))
    }
}

impl FromIterator<SmallString> for SmallString {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = SmallString>,
    {
        Self::from_inner(BackendString::from_iter(iter.into_iter().map(|s| s.inner)))
    }
}

impl FromIterator<SmallString> for String {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = SmallString>,
    {
        String::from_iter(iter.into_iter().map(|s| s.inner))
    }
}

impl FromIterator<SmallString> for Cow<'_, str> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = SmallString>,
    {
        Cow::from_iter(iter.into_iter().map(|s| s.inner))
    }
}

impl FromIterator<char> for SmallString {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        Self::from_inner(BackendString::from_iter(iter))
    }
}

impl std::str::FromStr for SmallString {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BackendString::from_str(s).map(Self::from_inner)
    }
}

macro_rules! impl_index_range {
    ($range:ty) => {
        impl Index<$range> for SmallString {
            type Output = str;

            #[inline]
            fn index(&self, index: $range) -> &Self::Output {
                self.inner.index(index)
            }
        }

        impl IndexMut<$range> for SmallString {
            #[inline]
            fn index_mut(&mut self, index: $range) -> &mut Self::Output {
                self.inner.index_mut(index)
            }
        }
    };
}

impl_index_range!(Range<usize>);
impl_index_range!(RangeFrom<usize>);
impl_index_range!(RangeFull);
impl_index_range!(RangeInclusive<usize>);
impl_index_range!(RangeTo<usize>);
impl_index_range!(RangeToInclusive<usize>);

macro_rules! impl_cmp {
    ($other:ty) => {
        impl PartialEq<$other> for SmallString {
            #[inline]
            fn eq(&self, other: &$other) -> bool {
                PartialEq::eq(&self.inner, other)
            }
        }

        impl PartialEq<SmallString> for $other {
            #[inline]
            fn eq(&self, other: &SmallString) -> bool {
                PartialEq::eq(self, &other.inner)
            }
        }
    };
}

impl_cmp!(str);
impl_cmp!(String);
impl_cmp!(Cow<'_, str>);
