// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
