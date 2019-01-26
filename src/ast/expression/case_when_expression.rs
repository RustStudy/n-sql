// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::Expression;
use ast::PredicateExpression;

#[derive(Clone, Debug)]
pub enum CaseWhenExpression {
    Simple(SimpleCaseWhenExpression),
    Searched(SearchedCaseWhenExpression),
}

#[derive(Clone, Debug)]
pub struct SimpleCaseWhenExpression {
    pub expr: Box<Expression>,
    pub matches: Vec<(Box<Expression>, Box<Expression>)>,
    pub default: Option<Box<Expression>>,
}

impl SimpleCaseWhenExpression {
    pub fn new(
        expr: Box<Expression>,
        matches: Vec<(Box<Expression>, Box<Expression>)>,
        default: Option<Box<Expression>>,
    ) -> SimpleCaseWhenExpression {
        SimpleCaseWhenExpression {
            expr,
            matches,
            default,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearchedCaseWhenExpression {
    pub matches: Vec<(Box<PredicateExpression>, Box<Expression>)>,
    pub default: Option<Box<Expression>>,
}

impl SearchedCaseWhenExpression {
    pub fn new(
        matches: Vec<(Box<PredicateExpression>, Box<Expression>)>,
        default: Option<Box<Expression>>,
    ) -> SearchedCaseWhenExpression {
        SearchedCaseWhenExpression { matches, default }
    }
}
