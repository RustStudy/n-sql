// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
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

