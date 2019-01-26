// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod concat_fn;
mod left_fn;
mod length_fn;
mod lower_fn;
mod pad_left_fn;
mod pad_right_fn;
mod replace_fn;
mod reverse_fn;
mod right_fn;
mod substr_fn;
mod trim_fn;
mod upper_fn;

pub use self::concat_fn::*;
pub use self::left_fn::*;
pub use self::length_fn::*;
pub use self::lower_fn::*;
pub use self::pad_left_fn::*;
pub use self::pad_right_fn::*;
pub use self::replace_fn::*;
pub use self::reverse_fn::*;
pub use self::right_fn::*;
pub use self::substr_fn::*;
pub use self::trim_fn::*;
pub use self::upper_fn::*;

#[derive(Clone, Debug)]
pub enum StringFn {
    Concat(ConcatFn),
    Left(LeftFn),
    Length(LengthFn),
    Lower(LowerFn),
    PadLeft(PadLeftFn),
    PadRight(PadRightFn),
    Replace(ReplaceFn),
    Reverse(ReverseFn),
    Right(RightFn),
    Substr(SubstrFn),
    Trim(TrimFn),
    Upper(UpperFn),
}