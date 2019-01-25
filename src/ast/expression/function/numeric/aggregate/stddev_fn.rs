// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::{Expression, AggregateType};

#[derive(Clone, Debug)]
pub struct StddevFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl StddevFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> StddevFn {
        StddevFn{expr, aggregate_type}
    }
}
