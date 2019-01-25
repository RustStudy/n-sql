// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
mod aggregate;
mod abs_fn;
mod ceil_fn;
mod cos_fn;
mod dense_rank_fn;
mod floor_fn;
mod log10_fn;
mod log_fn;
mod pow_fn;
mod rank_fn;
mod round_fn;
mod sign_fn;
mod sin_fn;
mod sqrt_fn;
mod tan_fn;


pub use self::aggregate::*;
pub use self::abs_fn::*;
pub use self::ceil_fn::*;
pub use self::cos_fn::*;
pub use self::dense_rank_fn::*;
pub use self::floor_fn::*;
pub use self::log10_fn::*;
pub use self::log_fn::*;
pub use self::pow_fn::*;
pub use self::rank_fn::*;
pub use self::round_fn::*;
pub use self::sign_fn::*;
pub use self::sin_fn::*;
pub use self::sqrt_fn::*;
pub use self::tan_fn::*;

#[derive(Clone, Debug)]
pub enum NumericFn {
    Abs(AbsFn),
    Ceil(CeilFn),
    Cos(CosFn),
    DenseRank(DenseRankFn),
    Floor(FloorFn),
    Log10(Log10Fn),
    Log(LogFn),
    Pow(PowFn),
    Rank(RankFn),
    Round(RoundFn),
    Sin(SinFn),
    Sign(SignFn),
    Sqrt(SqrtFn),
    Tan(TanFn)
}
