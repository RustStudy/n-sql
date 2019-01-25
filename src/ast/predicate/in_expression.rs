// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::super::{Expression, NumericValue, StringValue};
use ast::SetStatement;

#[derive(Clone, Debug)]
pub struct InExpression {
    pub left: Box<Expression>,
    pub values: InElements,
}

impl InExpression {
    pub fn new(left: Box<Expression>, values: InElements) -> InExpression {
        InExpression { left, values }
    }
}

#[derive(Clone, Debug)]
pub enum InElements {
    Values(Vec<Box<Expression>>),
    Set(Box<SetStatement>)
}
