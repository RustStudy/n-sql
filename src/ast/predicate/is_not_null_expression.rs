// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use super::super::Expression;

#[derive(Clone, Debug)]
pub struct IsNotNullExpression {
    pub expr: Box<Expression>,
}

impl IsNotNullExpression {
    pub fn new(expr: Box<Expression>) -> IsNotNullExpression {
        IsNotNullExpression { expr }
    }
}
