// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct MinuteSubFn {
    pub expr: Box<Expression>,
    pub offset: Box<Expression>,
}

impl MinuteSubFn {
    pub fn new(expr: Box<Expression>, offset: Box<Expression>) -> MinuteSubFn {
        MinuteSubFn { expr, offset }
    }
}