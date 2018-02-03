// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use prelude::*;
use traits::{Std, OsString as OsStringT, OsStr as OsStrT};

use borrow::{Borrow, Cow};
use fmt;
use ops;
use cmp;
use hash::{Hash, Hasher};
use rc::Rc;
use alloc::arc::Arc;

use sys_common::{AsInner, IntoInner, FromInner};

/// A type that can represent owned, mutable platform-native strings, but is
/// cheaply inter-convertible with Rust strings.
///
/// The need for this type arises from the fact that:
///
/// * On Unix systems, strings are often arbitrary sequences of non-zero
///   bytes, in many cases interpreted as UTF-8.
///
/// * On Windows, strings are often arbitrary sequences of non-zero 16-bit
///   values, interpreted as UTF-16 when it is valid to do so.
///
/// * In Rust, strings are always valid UTF-8, which may contain zeros.
///
/// `OsString` and [`OsStr`] bridge this gap by simultaneously representing Rust
/// and platform-native string values, and in particular allowing a Rust string
/// to be converted into an "OS" string with no cost if possible.
///
/// `OsString` is to [`&OsStr`] as [`String`] is to [`&str`]: the former
/// in each pair are owned strings; the latter are borrowed
/// references.
///
/// # Creating an `OsString`
///
/// **From a Rust string**: `OsString` implements
/// [`From`]`<`[`String`]`>`, so you can use `my_string.from` to
/// create an `OsString` from a normal Rust string.
///
/// **From slices:** Just like you can start with an empty Rust
/// [`String`] and then [`push_str`][String.push_str] `&str`
/// sub-string slices into it, you can create an empty `OsString` with
/// the [`new`] method and then push string slices into it with the
/// [`push`] method.
///
/// # Extracting a borrowed reference to the whole OS string
///
/// You can use the [`as_os_str`] method to get an `&`[`OsStr`] from
/// an `OsString`; this is effectively a borrowed reference to the
/// whole string.
///
/// # Conversions
///
/// See the [module's toplevel documentation about conversions][conversions] for a discussion on
/// the traits which `OsString` implements for conversions from/to native representations.
///
/// [`OsStr`]: struct.OsStr.html
/// [`&OsStr`]: struct.OsStr.html
/// [`From`]: ../convert/trait.From.html
/// [`String`]: ../string/struct.String.html
/// [`&str`]: ../primitive.str.html
/// [`u8`]: ../primitive.u8.html
/// [`u16`]: ../primitive.u16.html
/// [String.push_str]: ../string/struct.String.html#method.push_str
/// [`new`]: #method.new
/// [`push`]: #method.push
/// [`as_os_str`]: #method.as_os_str
#[derive(Clone)]
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct OsString<STD: Std> {
    inner: STD::OsString,
}

/// Borrowed reference to an OS string (see [`OsString`]).
///
/// This type represents a borrowed reference to a string in the operating system's preferred
/// representation.
///
/// `&OsStr` is to [`OsString`] as [`&str`] is to [`String`]: the former in each pair are borrowed
/// references; the latter are owned strings.
///
/// See the [module's toplevel documentation about conversions][conversions] for a discussion on
/// the traits which `OsStr` implements for conversions from/to native representations.
///
/// [`OsString`]: struct.OsString.html
/// [`&str`]: ../primitive.str.html
/// [`String`]: ../string/struct.String.html
/// [conversions]: index.html#conversions
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct OsStr<STD: Std> {
    inner: STD::OsStr,
}

impl<STD: Std> OsString<STD> {
    /// Constructs a new empty `OsString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let os_string = OsString::new();
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new() -> OsString<STD> {
        OsString { inner: STD::OsString::from_string(String::new()) }
    }

    /// Converts to an [`OsStr`] slice.
    ///
    /// [`OsStr`]: struct.OsStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::{OsString, OsStr};
    ///
    /// let os_string = OsString::from("foo");
    /// let os_str = OsStr::new("foo");
    /// assert_eq!(os_string.as_os_str(), os_str);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn as_os_str(&self) -> &OsStr<STD> {
        self
    }

    /// Converts the `OsString` into a [`String`] if it contains valid Unicode data.
    ///
    /// On failure, ownership of the original `OsString` is returned.
    ///
    /// [`String`]: ../../std/string/struct.String.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let os_string = OsString::from("foo");
    /// let string = os_string.into_string();
    /// assert_eq!(string, Ok(String::from("foo")));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn into_string(self) -> Result<String, OsString<STD>> {
        self.inner.into_string().map_err(|buf| OsString { inner: buf} )
    }

    /// Extends the string with the given [`&OsStr`] slice.
    ///
    /// [`&OsStr`]: struct.OsStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut os_string = OsString::from("foo");
    /// os_string.push("bar");
    /// assert_eq!(&os_string, "foobar");
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn push<T: AsRef<OsStr<STD>>>(&mut self, s: T) {
        self.inner.push_slice(&s.as_ref().inner)
    }

    /// Creates a new `OsString` with the given capacity.
    ///
    /// The string will be able to hold exactly `capacity` length units of other
    /// OS strings without reallocating. If `capacity` is 0, the string will not
    /// allocate.
    ///
    /// See main `OsString` documentation information about encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut os_string = OsString::with_capacity(10);
    /// let capacity = os_string.capacity();
    ///
    /// // This push is done without reallocating
    /// os_string.push("foo");
    ///
    /// assert_eq!(capacity, os_string.capacity());
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn with_capacity(capacity: usize) -> OsString<STD> {
        OsString {
            inner: STD::OsString::with_capacity(capacity)
        }
    }

    /// Truncates the `OsString` to zero length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut os_string = OsString::from("foo");
    /// assert_eq!(&os_string, "foo");
    ///
    /// os_string.clear();
    /// assert_eq!(&os_string, "");
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the capacity this `OsString` can hold without reallocating.
    ///
    /// See `OsString` introduction for information about encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut os_string = OsString::with_capacity(10);
    /// assert!(os_string.capacity() >= 10);
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least `additional` more capacity to be inserted
    /// in the given `OsString`.
    ///
    /// The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut s = OsString::new();
    /// s.reserve(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more capacity to
    /// be inserted in the given `OsString`. Does nothing if the capacity is
    /// already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore capacity can not be relied upon to be precisely
    /// minimal. Prefer reserve if future insertions are expected.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut s = OsString::new();
    /// s.reserve_exact(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    /// Shrinks the capacity of the `OsString` to match its length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsString;
    ///
    /// let mut s = OsString::from("foo");
    ///
    /// s.reserve(100);
    /// assert!(s.capacity() >= 100);
    ///
    /// s.shrink_to_fit();
    /// assert_eq!(3, s.capacity());
    /// ```
    // #[stable(feature = "osstring_shrink_to_fit", since = "1.19.0")]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Converts this `OsString` into a boxed [`OsStr`].
    ///
    /// [`OsStr`]: struct.OsStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::{OsString, OsStr};
    ///
    /// let s = OsString::from("hello");
    ///
    /// let b: Box<OsStr> = s.into_boxed_os_str();
    /// ```
    // #[stable(feature = "into_boxed_os_str", since = "1.20.0")]
    pub fn into_boxed_os_str(self) -> Box<OsStr<STD>> {
        let rw = Box::into_raw(self.inner.into_box()) as *mut OsStr<STD>;
        unsafe { Box::from_raw(rw) }
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> From<String> for OsString<STD> {
    fn from(s: String) -> OsString<STD> {
        OsString { inner: STD::OsString::from_string(s) }
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized + AsRef<OsStr<STD>>, STD: Std> From<&'a T> for OsString<STD> {
    fn from(s: &'a T) -> OsString<STD> {
        s.as_ref().to_os_string()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> ops::Index<ops::RangeFull> for OsString<STD> {
    type Output = OsStr<STD>;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &OsStr<STD> {
        OsStr::from_inner(self.inner.as_slice())
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> ops::Deref for OsString<STD> {
    type Target = OsStr<STD>;

    #[inline]
    fn deref(&self) -> &OsStr<STD> {
        &self[..]
    }
}

// #[stable(feature = "osstring_default", since = "1.9.0")]
impl<STD: Std> Default for OsString<STD> {
    /// Constructs an empty `OsString`.
    #[inline]
    fn default() -> OsString<STD> {
        OsString::new()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> fmt::Debug for OsString<STD> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, formatter)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq for OsString<STD> {
    fn eq(&self, other: &OsString<STD>) -> bool {
        &**self == &**other
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq<str> for OsString<STD> {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq<OsString<STD>> for str {
    fn eq(&self, other: &OsString<STD>) -> bool {
        &**other == self
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Eq for OsString<STD> {}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialOrd for OsString<STD> {
    #[inline]
    fn partial_cmp(&self, other: &OsString<STD>) -> Option<cmp::Ordering> {
        (&**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &OsString<STD>) -> bool { &**self < &**other }
    #[inline]
    fn le(&self, other: &OsString<STD>) -> bool { &**self <= &**other }
    #[inline]
    fn gt(&self, other: &OsString<STD>) -> bool { &**self > &**other }
    #[inline]
    fn ge(&self, other: &OsString<STD>) -> bool { &**self >= &**other }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialOrd<str> for OsString<STD> {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        (&**self).partial_cmp(other)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Ord for OsString<STD> {
    #[inline]
    fn cmp(&self, other: &OsString<STD>) -> cmp::Ordering {
        (&**self).cmp(&**other)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Hash for OsString<STD> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&**self).hash(state)
    }
}

impl<STD: Std> OsStr<STD> {
    /// Coerces into an `OsStr` slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// let os_str = OsStr::new("foo");
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new<S: AsRef<OsStr<STD>> + ?Sized>(s: &S) -> &OsStr<STD> {
        s.as_ref()
    }

    fn from_inner(inner: &STD::OsStr) -> &OsStr<STD> {
        unsafe { &*(inner as *const STD::OsStr as *const OsStr<STD>) }
    }

    /// Yields a [`&str`] slice if the `OsStr` is valid Unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    ///
    /// [`&str`]: ../../std/primitive.str.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// let os_str = OsStr::new("foo");
    /// assert_eq!(os_str.to_str(), Some("foo"));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn to_str(&self) -> Option<&str> {
        self.inner.to_str()
    }

    /// Converts an `OsStr` to a [`Cow`]`<`[`str`]`>`.
    ///
    /// Any non-Unicode sequences are replaced with U+FFFD REPLACEMENT CHARACTER.
    ///
    /// [`Cow`]: ../../std/borrow/enum.Cow.html
    /// [`str`]: ../../std/primitive.str.html
    ///
    /// # Examples
    ///
    /// Calling `to_string_lossy` on an `OsStr` with valid unicode:
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// let os_str = OsStr::new("foo");
    /// assert_eq!(os_str.to_string_lossy(), "foo");
    /// ```
    ///
    /// Had `os_str` contained invalid unicode, the `to_string_lossy` call might
    /// have returned `"fo�"`.
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.inner.to_string_lossy()
    }

    /// Copies the slice into an owned [`OsString`].
    ///
    /// [`OsString`]: struct.OsString.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::{OsStr, OsString};
    ///
    /// let os_str = OsStr::new("foo");
    /// let os_string = os_str.to_os_string();
    /// assert_eq!(os_string, OsString::from("foo"));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn to_os_string(&self) -> OsString<STD> {
        OsString { inner: self.inner.to_owned() }
    }

    /// Checks whether the `OsStr` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// let os_str = OsStr::new("");
    /// assert!(os_str.is_empty());
    ///
    /// let os_str = OsStr::new("foo");
    /// assert!(!os_str.is_empty());
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the length of this `OsStr`.
    ///
    /// Note that this does **not** return the number of bytes in this string
    /// as, for example, OS strings on Windows are encoded as a list of [`u16`]
    /// rather than a list of bytes. This number is simply useful for passing to
    /// other methods like [`OsString::with_capacity`] to avoid reallocations.
    ///
    /// See `OsStr` introduction for more information about encoding.
    ///
    /// [`u16`]: ../primitive.u16.html
    /// [`OsString::with_capacity`]: struct.OsString.html#method.with_capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// let os_str = OsStr::new("");
    /// assert_eq!(os_str.len(), 0);
    ///
    /// let os_str = OsStr::new("foo");
    /// assert_eq!(os_str.len(), 3);
    /// ```
    // #[stable(feature = "osstring_simple_functions", since = "1.9.0")]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Converts a [`Box`]`<OsStr>` into an [`OsString`] without copying or allocating.
    ///
    /// [`Box`]: ../boxed/struct.Box.html
    /// [`OsString`]: struct.OsString.html
    // #[stable(feature = "into_boxed_os_str", since = "1.20.0")]
    pub fn into_os_string(self: Box<OsStr<STD>>) -> OsString<STD> {
        let boxed = unsafe { Box::from_raw(Box::into_raw(self) as *mut STD::OsStr) };
        OsString { inner: STD::OsString::from_box(boxed) }
    }

    /// Gets the underlying byte representation.
    ///
    /// Note: it is *crucial* that this API is private, to avoid
    /// revealing the internal, platform-specific encodings.
    fn bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

// #[stable(feature = "box_from_os_str", since = "1.17.0")]
impl<'a, STD: Std> From<&'a OsStr<STD>> for Box<OsStr<STD>> {
    fn from(s: &'a OsStr<STD>) -> Box<OsStr<STD>> {
        let rw = Box::into_raw(s.inner.into_box()) as *mut OsStr<STD>;
        unsafe { Box::from_raw(rw) }
    }
}

// #[stable(feature = "os_string_from_box", since = "1.18.0")]
impl<STD: Std> From<Box<OsStr<STD>>> for OsString<STD> {
    fn from(boxed: Box<OsStr<STD>>) -> OsString<STD> {
        boxed.into_os_string()
    }
}

// #[stable(feature = "box_from_os_string", since = "1.20.0")]
impl<STD: Std> From<OsString<STD>> for Box<OsStr<STD>> {
    fn from(s: OsString<STD>) -> Box<OsStr<STD>> {
        s.into_boxed_os_str()
    }
}

// #[stable(feature = "shared_from_slice2", since = "1.24.0")]
impl<STD: Std> Into<Arc<OsStr<STD>>> for OsString<STD> {
    #[inline]
    fn into(self) -> Arc<OsStr<STD>> {
        let arc = self.inner.into_arc();
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const OsStr<STD>) }
    }
}

// #[stable(feature = "shared_from_slice2", since = "1.24.0")]
impl<'a, STD: Std> Into<Arc<OsStr<STD>>> for &'a OsStr<STD> {
    #[inline]
    fn into(self) -> Arc<OsStr<STD>> {
        let arc = self.inner.into_arc();
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const OsStr<STD>) }
    }
}

// #[stable(feature = "shared_from_slice2", since = "1.24.0")]
impl<STD: Std> Into<Rc<OsStr<STD>>> for OsString<STD> {
    #[inline]
    fn into(self) -> Rc<OsStr<STD>> {
        let rc = self.inner.into_rc();
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const OsStr<STD>) }
    }
}

// #[stable(feature = "shared_from_slice2", since = "1.24.0")]
impl<'a, STD: Std> Into<Rc<OsStr<STD>>> for &'a OsStr<STD> {
    #[inline]
    fn into(self) -> Rc<OsStr<STD>> {
        let rc = self.inner.into_rc();
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const OsStr<STD>) }
    }
}

// #[stable(feature = "box_default_extra", since = "1.17.0")]
impl<STD: Std> Default for Box<OsStr<STD>> {
    fn default() -> Box<OsStr<STD>> {
        let rw = Box::into_raw(STD::OsStr::empty_box()) as *mut OsStr<STD>;
        unsafe { Box::from_raw(rw) }
    }
}

// #[stable(feature = "osstring_default", since = "1.9.0")]
impl<'a, STD: Std> Default for &'a OsStr<STD> {
    /// Creates an empty `OsStr`.
    #[inline]
    fn default() -> &'a OsStr<STD> {
        OsStr::new("")
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq for OsStr<STD> {
    fn eq(&self, other: &OsStr<STD>) -> bool {
        self.bytes().eq(other.bytes())
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq<str> for OsStr<STD> {
    fn eq(&self, other: &str) -> bool {
        *self == *OsStr::new(other)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialEq<OsStr<STD>> for str {
    fn eq(&self, other: &OsStr<STD>) -> bool {
        *other == *OsStr::new(self)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Eq for OsStr<STD> {}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialOrd for OsStr<STD> {
    #[inline]
    fn partial_cmp(&self, other: &OsStr<STD>) -> Option<cmp::Ordering> {
        self.bytes().partial_cmp(other.bytes())
    }
    #[inline]
    fn lt(&self, other: &OsStr<STD>) -> bool { self.bytes().lt(other.bytes()) }
    #[inline]
    fn le(&self, other: &OsStr<STD>) -> bool { self.bytes().le(other.bytes()) }
    #[inline]
    fn gt(&self, other: &OsStr<STD>) -> bool { self.bytes().gt(other.bytes()) }
    #[inline]
    fn ge(&self, other: &OsStr<STD>) -> bool { self.bytes().ge(other.bytes()) }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> PartialOrd<str> for OsStr<STD> {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.partial_cmp(OsStr::new(other))
    }
}

// FIXME (#19470): cannot provide PartialOrd<OsStr> for str until we
// have more flexible coherence rules.

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Ord for OsStr<STD> {
    #[inline]
    fn cmp(&self, other: &OsStr<STD>) -> cmp::Ordering { self.bytes().cmp(other.bytes()) }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        // #[stable(feature = "cmp_os_str", since = "1.8.0")]
        impl<'a, 'b, STD: Std> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { <OsStr<STD> as PartialEq>::eq(self, other) }
        }

        // #[stable(feature = "cmp_os_str", since = "1.8.0")]
        impl<'a, 'b, STD: Std> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { <OsStr<STD> as PartialEq>::eq(self, other) }
        }

        // #[stable(feature = "cmp_os_str", since = "1.8.0")]
        impl<'a, 'b, STD: Std> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <OsStr<STD> as PartialOrd>::partial_cmp(self, other)
            }
        }

        // #[stable(feature = "cmp_os_str", since = "1.8.0")]
        impl<'a, 'b, STD: Std> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <OsStr<STD> as PartialOrd>::partial_cmp(self, other)
            }
        }
    }
}

impl_cmp!(OsString<STD>, OsStr<STD>);
impl_cmp!(OsString<STD>, &'a OsStr<STD>);
// impl_cmp!(Cow<'a, OsStr<STD>>, OsStr<STD>);
// impl_cmp!(Cow<'a, OsStr<STD>>, &'b OsStr<STD>);
// impl_cmp!(Cow<'a, OsStr<STD>>, OsString<STD>);

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Hash for OsStr<STD> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes().hash(state)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> fmt::Debug for OsStr<STD> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

impl<STD: Std> OsStr<STD> {
    pub(crate) fn display(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> Borrow<OsStr<STD>> for OsString<STD> {
    fn borrow(&self) -> &OsStr<STD> { &self[..] }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> ToOwned for OsStr<STD> {
    type Owned = OsString<STD>;
    fn to_owned(&self) -> OsString<STD> {
        self.to_os_string()
    }
    fn clone_into(&self, target: &mut OsString<STD>) {
        target.clear();
        target.push(self);
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> AsRef<OsStr<STD>> for OsStr<STD> {
    fn as_ref(&self) -> &OsStr<STD> {
        self
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> AsRef<OsStr<STD>> for OsString<STD> {
    fn as_ref(&self) -> &OsStr<STD> {
        self
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> AsRef<OsStr<STD>> for str {
    fn as_ref(&self) -> &OsStr<STD> {
        OsStr::from_inner(STD::OsStr::from_str(self))
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<STD: Std> AsRef<OsStr<STD>> for String {
    fn as_ref(&self) -> &OsStr<STD> {
        (&**self).as_ref()
    }
}

impl<STD: Std> FromInner<STD::OsString> for OsString<STD> {
    fn from_inner(buf: STD::OsString) -> OsString<STD> {
        OsString { inner: buf }
    }
}

impl<STD: Std> IntoInner<STD::OsString> for OsString<STD> {
    fn into_inner(self) -> STD::OsString {
        self.inner
    }
}

impl<STD: Std> AsInner<STD::OsStr> for OsStr<STD> {
    fn as_inner(&self) -> &STD::OsStr {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sys_common::{AsInner, IntoInner};

    use rc::Rc;
    use sync::Arc;

    #[test]
    fn test_os_string_with_capacity() {
        let os_string = OsString::with_capacity(0);
        assert_eq!(0, os_string.inner.into_inner().capacity());

        let os_string = OsString::with_capacity(10);
        assert_eq!(10, os_string.inner.into_inner().capacity());

        let mut os_string = OsString::with_capacity(0);
        os_string.push("abc");
        assert!(os_string.inner.into_inner().capacity() >= 3);
    }

    #[test]
    fn test_os_string_clear() {
        let mut os_string = OsString::from("abc");
        assert_eq!(3, os_string.inner.as_inner().len());

        os_string.clear();
        assert_eq!(&os_string, "");
        assert_eq!(0, os_string.inner.as_inner().len());
    }

    #[test]
    fn test_os_string_capacity() {
        let os_string = OsString::with_capacity(0);
        assert_eq!(0, os_string.capacity());

        let os_string = OsString::with_capacity(10);
        assert_eq!(10, os_string.capacity());

        let mut os_string = OsString::with_capacity(0);
        os_string.push("abc");
        assert!(os_string.capacity() >= 3);
    }

    #[test]
    fn test_os_string_reserve() {
        let mut os_string = OsString::new();
        assert_eq!(os_string.capacity(), 0);

        os_string.reserve(2);
        assert!(os_string.capacity() >= 2);

        for _ in 0..16 {
            os_string.push("a");
        }

        assert!(os_string.capacity() >= 16);
        os_string.reserve(16);
        assert!(os_string.capacity() >= 32);

        os_string.push("a");

        os_string.reserve(16);
        assert!(os_string.capacity() >= 33)
    }

    #[test]
    fn test_os_string_reserve_exact() {
        let mut os_string = OsString::new();
        assert_eq!(os_string.capacity(), 0);

        os_string.reserve_exact(2);
        assert!(os_string.capacity() >= 2);

        for _ in 0..16 {
            os_string.push("a");
        }

        assert!(os_string.capacity() >= 16);
        os_string.reserve_exact(16);
        assert!(os_string.capacity() >= 32);

        os_string.push("a");

        os_string.reserve_exact(16);
        assert!(os_string.capacity() >= 33)
    }

    #[test]
    fn test_os_string_default() {
        let os_string: OsString = Default::default();
        assert_eq!("", &os_string);
    }

    #[test]
    fn test_os_str_is_empty() {
        let mut os_string = OsString::new();
        assert!(os_string.is_empty());

        os_string.push("abc");
        assert!(!os_string.is_empty());

        os_string.clear();
        assert!(os_string.is_empty());
    }

    #[test]
    fn test_os_str_len() {
        let mut os_string = OsString::new();
        assert_eq!(0, os_string.len());

        os_string.push("abc");
        assert_eq!(3, os_string.len());

        os_string.clear();
        assert_eq!(0, os_string.len());
    }

    #[test]
    fn test_os_str_default() {
        let os_str: &OsStr = Default::default();
        assert_eq!("", os_str);
    }

    #[test]
    fn into_boxed() {
        let orig = "Hello, world!";
        let os_str = OsStr::new(orig);
        let boxed: Box<OsStr> = Box::from(os_str);
        let os_string = os_str.to_owned().into_boxed_os_str().into_os_string();
        assert_eq!(os_str, &*boxed);
        assert_eq!(&*boxed, &*os_string);
        assert_eq!(&*os_string, os_str);
    }

    #[test]
    fn boxed_default() {
        let boxed = <Box<OsStr>>::default();
        assert!(boxed.is_empty());
    }

    #[test]
    fn test_os_str_clone_into() {
        let mut os_string = OsString::with_capacity(123);
        os_string.push("hello");
        let os_str = OsStr::new("bonjour");
        os_str.clone_into(&mut os_string);
        assert_eq!(os_str, os_string);
        assert!(os_string.capacity() >= 123);
    }

    #[test]
    fn into_rc() {
        let orig = "Hello, world!";
        let os_str = OsStr::new(orig);
        let rc: Rc<OsStr> = Rc::from(os_str);
        let arc: Arc<OsStr> = Arc::from(os_str);

        assert_eq!(&*rc, os_str);
        assert_eq!(&*arc, os_str);

        let rc2: Rc<OsStr> = Rc::from(os_str.to_owned());
        let arc2: Arc<OsStr> = Arc::from(os_str.to_owned());

        assert_eq!(&*rc2, os_str);
        assert_eq!(&*arc2, os_str);
    }
}
