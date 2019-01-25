// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
