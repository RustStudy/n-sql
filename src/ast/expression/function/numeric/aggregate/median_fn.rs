// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::AggregateType;
use ast::Expression;

#[derive(Clone, Debug)]
pub struct MedianFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MedianFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> MedianFn {
        MedianFn {
            expr,
            aggregate_type,
        }
    }
}
