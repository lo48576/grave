//! String type.

use std::{fmt, string::FromUtf8Error};

#[cfg(feature = "small-string")]
use inlinable_string::{InlinableString, StringExt, INLINE_STRING_CAPACITY};

/// Backend string type.
#[cfg(feature = "small-string")]
type BackendString = InlinableString;

/// Backend string type.
#[cfg(not(feature = "small-string"))]
type BackendString = String;

/// Owned string type with optional small string optimization.
///
/// This type is intended to be used to contain strings which is expected to be very short.
/// However, this type can contain long string.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SmallString {
    /// Inner string.
    inner: BackendString,
}

impl SmallString {
    /// Creates a new string from the given backend value.
    #[inline]
    fn from_inner(inner: BackendString) -> Self {
        Self { inner }
    }

    /// Creates a new empty string.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty string with a particular capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_inner(BackendString::with_capacity(capacity))
    }

    /// Converts a vector of byets to a string.
    #[inline]
    pub fn from_utf8(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
        BackendString::from_utf8(vec).map(Self::from_inner)
    }

    /// Converts a stringng into a byte vector.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.inner.into_bytes()
    }

    /// Extracts a string slice containing te entire stringngg.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Converts a string into a mutable string slice.
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.inner
    }

    /// Appends a given string slice onto the end of this string.
    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.inner.push_str(string)
    }

    /// Returns this string's capacity, in bytes.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Ensures that this string's capacity is at least `additional` bytes larger than its length.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Ensures that this string's capacity is `additional` bytes larger than its length.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    /// Appends the given `char` to the end of this string.
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch)
    }

    /// Returns a byte slice of this string's contents.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Shortens this string to the specified length.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len)
    }

    /// Removes the last character from the string buffer and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.inner.pop()
    }

    /// Removes a `char` from this string at a byte position and returns it.
    #[inline]
    pub fn remove(&mut self, idx: usize) -> char {
        self.inner.remove(idx)
    }

    /// Retains only the characters specified by the predicate.
    #[inline]
    #[cfg(not(feature = "small-string"))]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(char) -> bool,
    {
        self.inner.retain(f)
    }

    /// Retains only the characters specified by the predicate.
    #[inline]
    #[cfg(feature = "small-string")]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(char) -> bool,
    {
        retain(&mut self.inner, f)
    }

    /// Inserts a character into this string at a byte position.
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.inner.insert(idx, ch)
    }

    /// Inserts a string slice into this string at a byte position.
    #[inline]
    #[cfg(feature = "small-string")]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        insert_str(&mut self.inner, idx, string)
    }

    /// Inserts a string slice into this string at a byte position.
    #[inline]
    #[cfg(not(feature = "small-string"))]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.inner.insert_str(idx, string)
    }

    /// Returns the length of this string, in bytes, not `char`s or graphemes.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the string has a length of zero, and `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Splits the string into two at the given index.
    #[inline]
    #[cfg(not(feature = "small-string"))]
    pub fn split_off(&mut self, at: usize) -> Self {
        Self::from_inner(self.inner.split_off(at))
    }

    /// Splits the string into two at the given index.
    #[inline]
    #[cfg(feature = "small-string")]
    pub fn split_off(&mut self, at: usize) -> Self {
        Self::from_inner(split_off(&mut self.inner, at))
    }

    /// Truncates this string, removing all contents.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Converts this string into a `Box<str>`.
    #[inline]
    #[cfg(not(feature = "small-string"))]
    pub fn into_boxed_str(self) -> Box<str> {
        self.inner.into_boxed_str()
    }

    /// Converts this string into a `Box<str>`.
    #[inline]
    #[cfg(feature = "small-string")]
    pub fn into_boxed_str(self) -> Box<str> {
        match self.inner {
            InlinableString::Heap(s) => s.into_boxed_str(),
            InlinableString::Inline(s) => Box::from(AsRef::<str>::as_ref(&s)),
        }
    }
}

impl From<String> for SmallString {
    #[cfg_attr(not(feature = "small-string"), allow(clippy::identity_conversion))]
    fn from(s: String) -> Self {
        Self::from_inner(s.into())
    }
}

impl From<&'_ str> for SmallString {
    fn from(s: &str) -> Self {
        Self::from_inner(s.into())
    }
}

impl Into<String> for SmallString {
    #[cfg_attr(not(feature = "small-string"), allow(clippy::identity_conversion))]
    fn into(self) -> String {
        self.inner.into()
    }
}

impl fmt::Display for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// `String::retain()` for `InlinableString`.
// Inefficient but fully safe.
#[cfg(feature = "small-string")]
fn retain<F>(this: &mut InlinableString, mut f: F)
where
    F: FnMut(char) -> bool,
{
    // Position of a character to check.
    let mut idx = 0;

    while idx < this.len() {
        // Get the character to check.
        let ch = (&this[idx..])
            .chars()
            .next()
            .expect("Should never fail: `this` has one or more valid character");

        if f(ch) {
            // Advance the cursor.
            idx += ch.len_utf8();
        } else {
            // Remove the character.
            this.remove(idx);
        }
    }
}

/// `String::insert_str()` for `InlinableString`.
// Inefficient but fully safe.
#[cfg(feature = "small-string")]
fn insert_str(this: &mut InlinableString, idx: usize, string: &str) {
    match this {
        InlinableString::Heap(ref mut s) => s.insert_str(idx, string),
        InlinableString::Inline(ref mut s) => {
            let new_len = s.len() + string.len();
            if new_len > INLINE_STRING_CAPACITY {
                let mut s = String::with_capacity(new_len);
                s.push_str(&this[..idx]);
                s.push_str(string);
                s.push_str(&this[idx..]);
                *this = InlinableString::Heap(s);
                return;
            }
            let mut next_idx = idx;
            for ch in string.chars() {
                let ch_len = ch.len_utf8();
                s.insert(next_idx, ch).expect(
                    "Should never fail because `new_len` is \
                     not greater than `INLINE_STRING_CAPACITY`",
                );
                next_idx += ch_len;
            }
        }
    }
}

/// `String::split_off()` for `InlinableString`.
// Unoptimized but fully `safe`.
#[inline]
#[cfg(feature = "small-string")]
pub fn split_off(this: &mut InlinableString, at: usize) -> InlinableString {
    let len = this.len();

    assert!(at <= len, "`at` out of bounds");

    let other_len = len - at;
    let mut other = InlinableString::with_capacity(other_len);
    other.push_str(&this[at..]);
    this.truncate(at);

    assert_eq!(this.len(), at, "`this` has unexpected length");
    assert_eq!(other.len(), other_len, "`other` has unexpected length");

    other
}
