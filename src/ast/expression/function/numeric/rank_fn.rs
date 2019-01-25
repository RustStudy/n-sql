// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::{Expression, SortingDirection};


#[derive(Clone, Debug)]
pub struct RankFn {
    pub expr: Box<Expression>,
    pub order: Option<SortingDirection>
}

impl RankFn {
    pub fn new(expr: Box<Expression>, order: Option<SortingDirection>) -> RankFn {
        RankFn { expr, order }
    }
}

