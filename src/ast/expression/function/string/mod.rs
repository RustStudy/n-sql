// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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