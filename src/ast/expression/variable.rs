// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Identifier;

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Identifier
}

impl Variable {
    pub fn new(name: Identifier) -> Variable {
        Variable { name }
    }
}
