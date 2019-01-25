// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct PadRightFn {
    pub text: Expression,
    pub length: Expression,
    pub pad_text: Option<Expression>,
}

impl PadRightFn {
    pub fn new(text: Expression, length: Expression, pad_text: Option<Expression>) -> PadRightFn {
        PadRightFn{text, length, pad_text}
    }
}
