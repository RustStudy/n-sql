// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Identifier;

#[derive(Clone, Debug)]
pub struct Table {
    pub database: Option<Identifier>,
    pub schema: Option<Identifier>,
    pub name: Identifier
}

impl Table {
    pub fn new(
        name: Identifier,
        schema: Option<Identifier>,
        database: Option<Identifier>)
        -> Table {
        Table { name, schema, database }
    }
}
