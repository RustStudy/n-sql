// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::SelectStatement;
use ast::PaginationStatement;


#[derive(Clone, Debug)]
pub enum SetStatement {
    Select(SelectStatement),
    Pagination(Box<PaginationStatement>),
    Intersect(Box<SetStatement>, Box<SetStatement>),
    Minus(Box<SetStatement>, Box<SetStatement>),
    Union(Box<SetStatement>, Box<SetStatement>),
    UnionAll(Box<SetStatement>, Box<SetStatement>),
}
