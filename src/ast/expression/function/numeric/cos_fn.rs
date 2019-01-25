// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct CosFn {
    pub expr: Box<Expression>
}

impl CosFn {
    pub fn new(expr: Box<Expression>) -> CosFn {
        CosFn{expr}
    }
}