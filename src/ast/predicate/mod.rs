// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod comparison_expression;
mod in_expression;
mod is_not_null_expression;
mod is_null_expression;
mod logical_expression;
mod not_expression;
mod not_in_expression;

pub use self::comparison_expression::*;
pub use self::in_expression::*;
pub use self::is_not_null_expression::*;
pub use self::is_null_expression::*;
pub use self::logical_expression::*;
pub use self::not_expression::*;
pub use self::not_in_expression::*;

use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Debug)]
pub enum PredicateExpression {
    Comparison(ComparisonExpression),
    Logical(LogicalExpression),
    Not(NotExpression),
    IsNull(IsNullExpression),
    IsNotNull(IsNotNullExpression),
    In(InExpression),
    NotIn(NotInExpression),
}

impl PredicateExpression {
    fn not(self) -> NotExpression {
        NotExpression::new(self.into())
    }
}

impl From<ComparisonExpression> for PredicateExpression {
    fn from(v: ComparisonExpression) -> Self {
        v.into()
    }
}

impl From<LogicalExpression> for PredicateExpression {
    fn from(v: LogicalExpression) -> Self {
        v.into()
    }
}

impl From<NotExpression> for PredicateExpression {
    fn from(v: NotExpression) -> Self {
        v.into()
    }
}

// region [box]
impl From<ComparisonExpression> for Box<PredicateExpression> {
    fn from(v: ComparisonExpression) -> Self {
        v.into()
    }
}

impl From<LogicalExpression> for Box<PredicateExpression> {
    fn from(v: LogicalExpression) -> Self {
        v.into()
    }
}

impl From<Box<LogicalExpression>> for Box<PredicateExpression> {
    fn from(v: Box<LogicalExpression>) -> Self {
        v.into()
    }
}
// endregion
