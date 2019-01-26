// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::PredicateExpression;

#[derive(Copy, Clone, Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone, Debug)]
pub struct LogicalExpression {
    pub left: Box<PredicateExpression>,
    pub op: LogicalOperator,
    pub right: Box<PredicateExpression>,
}

impl LogicalExpression {
    pub fn new(
        left: Box<PredicateExpression>,
        op: LogicalOperator,
        right: Box<PredicateExpression>,
    ) -> LogicalExpression {
        LogicalExpression::from_predicate(left, op, right)
    }

    pub fn from_predicate(
        left: Box<PredicateExpression>,
        op: LogicalOperator,
        right: Box<PredicateExpression>,
    ) -> LogicalExpression {
        LogicalExpression { left, op, right }
    }

    pub fn from_logical(
        left: Box<LogicalExpression>,
        op: LogicalOperator,
        right: Box<LogicalExpression>,
    ) -> LogicalExpression {
        LogicalExpression {
            left: left.into(),
            op,
            right: right.into(),
        }
    }
}
