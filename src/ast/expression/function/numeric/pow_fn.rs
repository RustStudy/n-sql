// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression;


#[derive(Clone, Debug)]
pub struct PowFn {
    pub x: Box<Expression>,
    pub y: Box<Expression>
}

impl PowFn {
    pub fn new(x: Box<Expression>, y: Box<Expression>) -> PowFn {
        PowFn{x, y}
    }
}
