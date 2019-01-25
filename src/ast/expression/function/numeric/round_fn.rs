// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct RoundFn {
    pub expr: Box<Expression>,
    pub precision : Option<Box<Expression>>,
}

impl RoundFn {
    pub fn new(expr: Box<Expression>, precision : Option<Box<Expression>>) -> RoundFn {
        RoundFn{expr, precision}
    }
}