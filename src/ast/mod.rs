// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Error, Formatter};

mod expression;
mod identifier;
mod predicate;
mod statement;
mod catalog;

pub use self::expression::*;
pub use self::identifier::*;
pub use self::predicate::*;
pub use self::statement::*;
pub use self::catalog::*;

pub enum Ast {
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    Predicate(Box<PredicateExpression>),
}
