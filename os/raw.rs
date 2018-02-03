// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

use traits::Std;

/// Type used to construct void pointers for use with C.
///
/// This type is only useful as a pointer target. Do not use it as a
/// return type for FFI functions which have the `void` return type in
/// C. Use the unit type `()` or omit the return type instead.
// NB: For LLVM to recognize the void pointer type and by extension
//     functions like malloc(), we need to have it represented as i8* in
//     LLVM bitcode. The enum used here ensures this and prevents misuse
//     of the "raw" type by only having private variants.. We need two
//     variants, because the compiler complains about the repr attribute
//     otherwise.
#[repr(u8)]
#[stable(feature = "raw_os", since = "1.1.0")]
pub enum c_void {
    #[unstable(feature = "c_void_variant", reason = "should not have to exist",
               issue = "0")]
    #[doc(hidden)] __variant1,
    #[unstable(feature = "c_void_variant", reason = "should not have to exist",
               issue = "0")]
    #[doc(hidden)] __variant2,
}

pub type c_char<STD> = <STD as Std>::c_char;
pub type c_double<STD> = <STD as Std>::c_double;
pub type c_float<STD> = <STD as Std>::c_float;
pub type c_int<STD> = <STD as Std>::c_int;
pub type c_long<STD> = <STD as Std>::c_long;
pub type c_longlong<STD> = <STD as Std>::c_longlong;
pub type c_schar<STD> = <STD as Std>::c_schar;
pub type c_short<STD> = <STD as Std>::c_short;
pub type c_uchar<STD> = <STD as Std>::c_uchar;
pub type c_uint<STD> = <STD as Std>::c_uint;
pub type c_ulong<STD> = <STD as Std>::c_ulong;
pub type c_ulonglong<STD> = <STD as Std>::c_ulonglong;
pub type c_ushort<STD> = <STD as Std>::c_ushort;
