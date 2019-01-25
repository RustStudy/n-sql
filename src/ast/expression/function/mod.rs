// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod cast_fn;
mod numeric;
mod nvl_fn;
mod string;
mod datetime;
mod coalesce_fn;
mod format_fn;


pub use self::cast_fn::*;
pub use self::nvl_fn::*;
pub use self::string::*;
pub use self::datetime::*;
pub use self::numeric::*;
pub use self::coalesce_fn::*;
pub use self::format_fn::*;

#[derive(Clone, Debug)]
pub enum Function {
    Aggregate(AggregateFn),
    Numeric(NumericFn),
    String(StringFn),
    Cast(Box<CastFn>),
    Nvl(Box<NvlFn>),
    Datetime(DatetimeFn),
    Coalesce(CoalesceFn),
    Now
}
