// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](../index.html) for more.

// Re-exported core operators
pub(crate) use core::marker::{Copy, Send, Sized, Sync};
pub(crate) use core::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
pub(crate) use core::mem::drop;

// Re-exported types and traits
pub(crate) use alloc::boxed::Box;
pub(crate) use alloc::borrow::ToOwned;
pub(crate) use core::clone::Clone;
pub(crate) use core::cmp::{PartialEq, PartialOrd, Eq, Ord};
pub(crate) use core::convert::{AsRef, AsMut, Into, From};
pub(crate) use core::default::Default;
pub(crate) use core::iter::{Iterator, Extend, IntoIterator};
pub(crate) use core::iter::{DoubleEndedIterator, ExactSizeIterator};
pub(crate) use core::option::Option::{self, Some, None};
pub(crate) use core::result::Result::{self, Ok, Err};
pub(crate) use alloc::string::{String, ToString};
pub(crate) use alloc::vec::Vec;
