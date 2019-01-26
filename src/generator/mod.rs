// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


mod visitor;

use ast::{Expression, PredicateExpression,Statement};
use self::visitor::Visitor;
use std::result;
use std::fmt::Error;

pub trait Generator<T> {
    fn to_sql(&self) -> result::Result<String, Error>;
}

impl Visitor for Expression {
}

impl Visitor for PredicateExpression {
}

impl Visitor for Statement {
}


impl Generator<Expression> for Expression {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        self.visit_expression(self, &mut s)?;
        Ok(s)
    }
}


impl Generator<PredicateExpression> for PredicateExpression {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        self.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl Generator<Statement> for Statement {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        self.visit_statement(self, &mut s)?;
        Ok(s)
    }
}