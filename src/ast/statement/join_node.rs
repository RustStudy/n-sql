// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::TableView;
use ast::PredicateExpression;

#[derive(Clone, Debug)]
pub struct JoinNode {
    pub join_type: JoinType,
    pub base: Box<TableView>,
    pub join: Box<TableView>,
    pub condition: Box<PredicateExpression>,
}

impl JoinNode {
    pub fn new(
        base: Box<TableView>,
        join_type: JoinType,
        join: Box<TableView>,
        condition: Box<PredicateExpression>)
        -> JoinNode {
        JoinNode { base, join, join_type, condition }
    }
}


#[derive(Clone, Debug, Copy)]
pub enum JoinType {
    None = 0,
    Inner = 1,
    Left = 2,
    Right = 3,
    LeftOuter = 4,
    RightOuter = 5,
    Full = 6,
    Cross = 7,
}