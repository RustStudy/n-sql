// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct CoalesceFn {
    pub items: Vec<Box<Expression>>
}

impl CoalesceFn {
    pub fn new(items: Vec<Box<Expression>>) -> CoalesceFn {
        CoalesceFn { items }
    }
}
