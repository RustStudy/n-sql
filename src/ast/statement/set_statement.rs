// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
