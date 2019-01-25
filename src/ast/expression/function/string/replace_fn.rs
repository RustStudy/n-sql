// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct ReplaceFn {
    pub text: Expression,
    pub replace_text: Expression,
}

impl ReplaceFn {
    pub fn new(text: Expression, replace_text: Expression) -> ReplaceFn {
        ReplaceFn{text, replace_text}
    }
}
