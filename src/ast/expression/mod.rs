// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Error, Formatter};

mod arithmetic_expression;
mod case_when_expression;
mod constant;
mod function;
mod unary_expression;
mod variable;


use ast::Column;
pub use self::arithmetic_expression::*;
pub use self::case_when_expression::*;
pub use self::constant::*;
pub use self::function::*;
pub use self::unary_expression::*;
pub use self::variable::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(ConstantValue),
    Column(Column),
    Arithmetic(ArithmeticExpression),
    Function(Function),
    CaseWhen(CaseWhenExpression),
    Unary(UnaryExpression),
    Variable(Variable)
}

impl From<ConstantValue> for Expression {
    fn from(v: ConstantValue) -> Self {
        Expression::Constant(v)
    }
}

impl From<CastFn> for Expression {
    fn from(v: CastFn) -> Self {
        Expression::Function(Function::Cast(v.into()).into())
    }
}


impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        ConstantValue::from(value).into()
    }
}

