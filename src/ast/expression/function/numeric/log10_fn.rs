// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;


#[derive(Clone, Debug)]
pub struct Log10Fn {
    pub expr: Box<Expression>
}

impl Log10Fn {
    pub fn new(expr: Box<Expression>) -> Log10Fn {
        Log10Fn { expr }
    }
}