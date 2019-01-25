// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
use std::fmt::{Debug, Error, Formatter};

use super::PredicateExpression;

#[derive(Clone, Debug)]
pub struct NotExpression {
    pub expr: Box<PredicateExpression>,
}

impl NotExpression {
    pub fn new(expr: Box<PredicateExpression>) -> NotExpression {
        NotExpression { expr }
    }
}
