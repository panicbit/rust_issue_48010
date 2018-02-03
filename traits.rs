// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use prelude::*;
use os::raw::c_char;
use io;
use traits;
use core::hash::Hash;
use fmt::Debug;

pub trait Std: Sized + Debug + Send + Sync {
    type c_char: Copy + Hash + 'static;
    type c_double: Copy + Hash + 'static;
    type c_float: Copy + Hash + 'static;
    type c_int: Copy + Hash + 'static;
    type c_long: Copy + Hash + 'static;
    type c_longlong: Copy + Hash + 'static;
    type c_schar: Copy + Hash + 'static;
    type c_short: Copy + Hash + 'static;
    type c_uchar: Copy + Hash + 'static;
    type c_uint: Copy + Hash + 'static;
    type c_ulong: Copy + Hash + 'static;
    type c_ulonglong: Copy + Hash + 'static;
    type c_ushort: Copy + Hash + 'static;

    type Mutex: traits::Mutex;

    /// Usually defined as `&[0]`
    fn empty_cstr() -> &'static [c_char<Self>];

    fn last_os_error() -> i32;
    fn error_string(code: i32) -> String;

    fn init();
    unsafe fn abort_internal() -> !;
    unsafe fn strlen(cs: *const c_char<Self>) -> usize;
    fn decode_error_kind(errno: i32) -> io::ErrorKind;
    unsafe fn thread_guard_init() -> Option<usize>;

    /// A safe interface to `memchr`.
    ///
    /// Returns the index corresponding to the first occurrence of `needle` in
    /// `haystack`, or `None` if one is not found.
    ///
    /// memchr reduces to super-optimized machine code at around an order of
    /// magnitude faster than `haystack.iter().position(|&b| b == needle)`.
    /// (See benchmarks.)
    ///
    /// # Examples
    ///
    /// This shows how to find the first position of a byte in a byte string.
    ///
    /// ```ignore (cannot-doctest-private-modules)
    /// use memchr::memchr;
    ///
    /// let haystack = b"the quick brown fox";
    /// assert_eq!(memchr(b'k', haystack), Some(8));
    /// ```
    fn memchr(needle: u8, haystack: &[u8]) -> Option<usize>;

    /// A safe interface to `memrchr`.
    ///
    /// Returns the index corresponding to the last occurrence of `needle` in
    /// `haystack`, or `None` if one is not found.
    ///
    /// # Examples
    ///
    /// This shows how to find the last position of a byte in a byte string.
    ///
    /// ```ignore (cannot-doctest-private-modules)
    /// use memchr::memrchr;
    ///
    /// let haystack = b"the quick brown fox";
    /// assert_eq!(memrchr(b'o', haystack), Some(17));
    /// ```
    fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize>;

    // rand
    // fn hashmap_random_keys() -> (u64, u64);

    /// One-time global initialization of command line arguments.
    unsafe fn args_init(argc: isize, argv: *const *const u8);
}

pub trait Mutex: Sync {
    /// Creates a new mutex for use.
    ///
    /// Behavior is undefined if the mutex is moved after it is
    /// first used with any of the functions below.
    const NEW: Self;

    /// Prepare the mutex for use.
    ///
    /// This should be called once the mutex is at a stable memory address.
    #[inline]
    unsafe fn init(&mut self);

    /// Locks the mutex blocking the current thread until it is available.
    ///
    /// Behavior is undefined if the mutex has been moved between this and any
    /// previous function call.
    #[inline]
    unsafe fn lock(&self);

    /// Attempts to lock the mutex without blocking, returning whether it was
    /// successfully acquired or not.
    ///
    /// Behavior is undefined if the mutex has been moved between this and any
    /// previous function call.
    #[inline]
    unsafe fn try_lock(&self) -> bool;

    /// Unlocks the mutex.
    ///
    /// Behavior is undefined if the current thread does not actually hold the
    /// mutex.
    #[inline]
    unsafe fn unlock(&self);

    /// Deallocates all resources associated with this mutex.
    ///
    /// Behavior is undefined if there are current or will be future users of
    /// this mutex.
    #[inline]
    unsafe fn destroy(&self);
}
