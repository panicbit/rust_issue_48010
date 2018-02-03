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
pub use core::marker::{Copy, Send, Sized, Sync};
pub use core::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
pub use core::mem::drop;

// Re-exported types and traits
pub use alloc::boxed::Box;
pub use alloc::borrow::ToOwned;
pub use core::clone::Clone;
pub use core::cmp::{PartialEq, PartialOrd, Eq, Ord};
pub use core::convert::{AsRef, AsMut, Into, From};
pub use core::default::Default;
pub use core::iter::{Iterator, Extend, IntoIterator};
pub use core::iter::{DoubleEndedIterator, ExactSizeIterator};
pub use core::option::Option::{self, Some, None};
pub use core::result::Result::{self, Ok, Err};
pub use alloc::string::{String, ToString};
pub use alloc::vec::Vec;
