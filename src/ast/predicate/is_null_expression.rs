// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::super::Expression;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Debug)]
pub struct IsNullExpression {
    pub expr: Box<Expression>,
}

impl IsNullExpression {
    pub fn new(expr: Box<Expression>) -> IsNullExpression {
        IsNullExpression { expr }
    }
}
