// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub enum UnaryExpression {
    Plus(Box<Expression>),
    Minus(Box<Expression>),
}
