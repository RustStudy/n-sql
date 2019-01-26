// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


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
