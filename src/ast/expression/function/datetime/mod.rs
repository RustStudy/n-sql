// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod day_add_fn;
mod day_sub_fn;
mod hour_add_fn;
mod hour_sub_fn;
mod minute_add_fn;
mod minute_sub_fn;
mod month_add_fn;
mod month_sub_fn;
mod second_add_fn;
mod second_sub_fn;
mod year_add_fn;
mod year_sub_fn;
mod extract_fn;
mod datetime_type;
mod datetime_diff_fn;

pub use self::day_add_fn::*;
pub use self::day_sub_fn::*;
pub use self::hour_add_fn::*;
pub use self::hour_sub_fn::*;
pub use self::minute_add_fn::*;
pub use self::minute_sub_fn::*;
pub use self::month_add_fn::*;
pub use self::month_sub_fn::*;
pub use self::second_add_fn::*;
pub use self::second_sub_fn::*;
pub use self::year_add_fn::*;
pub use self::year_sub_fn::*;
pub use self::extract_fn::*;
pub use self::datetime_type::*;
pub use self::datetime_diff_fn::*;




#[derive(Clone, Debug)]
pub enum DatetimeFn {
    Extract(ExtractFn),
    Diff(DatetimeDiffFn),
    DayAdd(DayAddFn),
    DaySub(DaySubFn),
    HourAdd(HourAddFn),
    HourSub(HourSubFn),
    MinuteAdd(MinuteAddFn),
    MinuteSub(MinuteSubFn),
    MonthAdd(MonthAddFn),
    MonthSub(MonthSubFn),
    SecondAdd(SecondAddFn),
    SecondSub(SecondSubFn),
    YearAdd(YearAddFn),
    YearSub(YearSubFn)
}

