// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression;

#[derive(Clone, Debug)]
pub struct YearAddFn {
    pub expr: Box<Expression>,
    pub offset: Box<Expression>,
}

impl YearAddFn {
    pub fn new(expr: Box<Expression>, offset: Box<Expression>) -> YearAddFn {
        YearAddFn { expr, offset }
    }
}