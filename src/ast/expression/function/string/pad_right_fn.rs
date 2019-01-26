// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
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
