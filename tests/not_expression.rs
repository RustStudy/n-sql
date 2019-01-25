// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;
use common::*;

#[test]
fn not_equal_comparison(){
    test_predicate("nOt(a = 'abc')", "not(a = 'abc')");
}

#[test]
fn not_logical(){
    test_predicate("nOt(a = 'abc' and age <3)", "not(a = 'abc' and age < 3)");
}