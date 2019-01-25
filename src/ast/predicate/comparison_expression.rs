// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::super::Expression;
use core::fmt::Write;
use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
}

#[derive(Clone, Debug)]
pub struct ComparisonExpression {
    pub left: Box<Expression>,
    pub op: ComparisonOperator,
    pub right: Box<Expression>,
}

impl ComparisonExpression {
    pub fn new(
        left: Box<Expression>,
        op: ComparisonOperator,
        right: Box<Expression>,
    ) -> ComparisonExpression {
        ComparisonExpression { left, op, right }
    }
}
