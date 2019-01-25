// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;
use common::*;

#[test]
fn test_add(){
    test_expression("a     +3","a + 3" );
}


#[test]
fn test_sub(){
    test_expression("a  - 3", "a - 3");
}

#[test]
fn test_mul(){
    test_expression("a* 3", "a * 3");
}


#[test]
fn test_div(){
    test_expression("a/ 3", "a / 3");
}

#[test]
fn test_priority(){
    test_expression("a/ 3 + 3", "a / 3 + 3");
}


#[test]
fn test_priority1(){
    test_expression("a/ (3 + 3)", "a / (3 + 3)");
}

#[test]
fn test_superfluous_brackets(){
    test_expression("(a)/ ((3 + 3))", "a / (3 + 3)");
}

#[test]
fn test_superfluous_brackets1(){
    test_expression("(a)/ ((m + n))", "a / (m + n)");
}