// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct SignFn {
    pub expr: Box<Expression>
}

impl SignFn {
    pub fn new(expr: Box<Expression>) -> SignFn {
        SignFn{expr}
    }
}