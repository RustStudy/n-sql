// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;
use ast::SetStatement;

#[derive(Clone, Debug)]
pub struct PaginationStatement {
    pub set: Box<SetStatement>,
    pub skip: Option<Box<Expression>>,
    pub limit: Option<Box<Expression>>,
}

impl PaginationStatement {
    pub fn new(
        set: Box<SetStatement>,
        skip: Option<Box<Expression>>,
        limit: Option<Box<Expression>>)
        -> PaginationStatement {
        PaginationStatement { set, skip, limit }
    }
}