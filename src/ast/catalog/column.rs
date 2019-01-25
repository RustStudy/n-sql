// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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
