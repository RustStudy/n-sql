// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
