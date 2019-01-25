// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod avg_fn;
mod count_fn;
mod max_fn;
mod median_fn;
mod min_fn;
mod sum_fn;
mod stddev_fn;

pub use self::avg_fn::*;
pub use self::count_fn::*;
pub use self::max_fn::*;
pub use self::median_fn::*;
pub use self::min_fn::*;
pub use self::sum_fn::*;
pub use self::stddev_fn::*;

#[derive(Clone, Debug)]
pub enum AggregateFn {
    Avg(AvgFn),
    Count(CountFn),
    Max(MaxFn),
    Median(MedianFn),
    Min(MinFn),
    Sum(SumFn),
    Stddev(StddevFn)
}

#[derive(Clone, Debug)]
pub enum AggregateType {
    All,
    Distinct,
    Unique,
}
