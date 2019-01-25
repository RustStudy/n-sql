// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct ConcatFn {
    pub items: Vec<Expression>
}

impl ConcatFn {
    pub fn new(items: Vec<Expression>) -> ConcatFn {
        ConcatFn{items}
    }
}

