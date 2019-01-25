// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;


#[derive(Clone, Debug)]
pub struct SqrtFn {
    pub expr: Box<Expression>
}

impl SqrtFn {
    pub fn new(expr: Box<Expression>) -> SqrtFn {
        SqrtFn{expr}
    }
}