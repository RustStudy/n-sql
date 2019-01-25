// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::{Expression, DatetimeType};

#[derive(Clone, Debug)]
pub struct ExtractFn {
    pub expr: Box<Expression>,
    pub extract_type: DatetimeType,
}


impl ExtractFn {
    pub fn new(extract_type: DatetimeType, expr: Box<Expression>) -> ExtractFn {
        ExtractFn { expr, extract_type }
    }
}
