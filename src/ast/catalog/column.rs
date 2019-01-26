// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::Identifier;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Debug)]
pub struct Column {
    pub schema: Option<Identifier>,
    pub table: Option<Identifier>,
    pub name: Identifier,
}

impl Column {
    pub fn new(
        name: Identifier,
        table: Option<Identifier>,
        schema: Option<Identifier>,
    ) -> Column {
        Column {
            name,
            table,
            schema,
        }
    }
}
