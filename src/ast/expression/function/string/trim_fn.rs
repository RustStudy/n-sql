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
pub enum TrimType {
    Both,
    Leading,
    Trailing
}


#[derive(Clone, Debug)]
pub struct TrimFn {
    pub text: Expression,
    pub trim_text: Option<Expression>,
    pub trim_type: TrimType
}

impl TrimFn {
    pub fn new(text: Expression, trim_text: Option<Expression>, trim_type: TrimType) -> TrimFn {
        TrimFn {text, trim_text, trim_type}
    }
}

