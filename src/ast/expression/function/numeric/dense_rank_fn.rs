// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


use ast::{Expression, SortingDirection};


#[derive(Clone, Debug)]
pub struct DenseRankFn {
    pub expr: Box<Expression>,
    pub order: Option<SortingDirection>
}

impl DenseRankFn {
    pub fn new(expr: Box<Expression>, order: Option<SortingDirection>) -> DenseRankFn {
        DenseRankFn { expr, order }
    }
}
