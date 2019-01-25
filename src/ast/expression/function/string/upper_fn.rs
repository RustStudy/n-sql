// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;


#[derive(Clone, Debug)]
pub struct UpperFn {
    pub text: Expression,
}

impl UpperFn {
    pub fn new(text: Expression) -> UpperFn {
        UpperFn { text }
    }
}