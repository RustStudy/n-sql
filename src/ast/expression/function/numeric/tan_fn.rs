// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;


#[derive(Clone, Debug)]
pub struct TanFn {
    pub expr: Box<Expression>
}

impl TanFn {
    pub fn new(expr: Box<Expression>) -> TanFn {
        TanFn{expr}
    }
}