// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::{Expression, DatetimeType};

#[derive(Clone, Debug)]
pub struct DatetimeDiffFn {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub diff_type: DatetimeType
}

impl DatetimeDiffFn {
    pub fn new(start: Box<Expression>, end: Box<Expression>, diff_type: DatetimeType) -> DatetimeDiffFn {
        DatetimeDiffFn{start, end, diff_type}
    }
}