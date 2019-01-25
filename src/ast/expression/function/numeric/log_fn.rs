// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;


#[derive(Clone, Debug)]
pub struct LogFn {
    pub base: Option<Box<Expression>>,
    pub number: Box<Expression>,
}

impl LogFn {
    pub fn new(base: Option<Box<Expression>>, number: Box<Expression>) -> LogFn {
        LogFn { base, number }
    }
}
