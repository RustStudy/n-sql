// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct SubstrFn {
    pub text: Expression,
    pub start: Expression,
    pub length: Option<Expression>
}

impl SubstrFn {
    pub fn new(text: Expression, start: Expression, length: Option<Expression>) -> SubstrFn {
        SubstrFn {text, start, length}
    }
}

