// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::super::{Expression, NumericValue, StringValue};
use super::InElements;

#[derive(Clone, Debug)]
pub struct NotInExpression {
    pub left: Box<Expression>,
    pub values: InElements,
}

impl NotInExpression {
    pub fn new(left: Box<Expression>, values: InElements) -> NotInExpression {
        NotInExpression { left, values }
    }
}
