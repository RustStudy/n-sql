// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;

use common::*;

#[test]
fn test(){
    test_expression("cast(a as text)", "a::text");
}

#[test]
fn test1(){
    test_expression("a::text", "a::text");
}