// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct NvlFn {
    pub expr: Box<Expression>,
    pub default: Box<Expression>,
}

impl NvlFn {
    pub fn new(expr: Box<Expression>, default: Box<Expression>) -> NvlFn {
        NvlFn { expr, default }
    }
}
