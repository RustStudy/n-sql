// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


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