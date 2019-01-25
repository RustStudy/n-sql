// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;


#[derive(Clone, Debug)]
pub struct LowerFn {
    pub text: Expression,
}

impl LowerFn {
    pub fn new(text: Expression) -> LowerFn {
        LowerFn { text }
    }
}
