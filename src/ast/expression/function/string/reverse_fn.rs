// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;


#[derive(Clone, Debug)]
pub struct ReverseFn {
    pub text: Expression,
}

impl ReverseFn {
    pub fn new(text: Expression) -> ReverseFn {
        ReverseFn{text}
    }
}