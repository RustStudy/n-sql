// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct SinFn {
    pub expr: Box<Expression>
}

impl SinFn {
    pub fn new(expr: Box<Expression>) -> SinFn {
        SinFn{expr}
    }
}