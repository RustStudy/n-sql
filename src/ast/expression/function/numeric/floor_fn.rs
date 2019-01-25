// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;


#[derive(Clone, Debug)]
pub struct FloorFn {
    pub expr: Box<Expression>
}

impl FloorFn {
    pub fn new(expr: Box<Expression>) -> FloorFn {
        FloorFn { expr }
    }
}