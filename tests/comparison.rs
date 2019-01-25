// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod common;

use common::*;

#[test]
fn test_equal(){
    test_predicate("a = c", "a = c");
}

#[test]
fn test_equal1(){
    test_predicate("a = 'abc'", "a = 'abc'");
}

#[test]
fn test_not_equal(){
    test_predicate("a != 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal1(){
    test_predicate("a ~= 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal2(){
    test_predicate("a ^= 'abc'", "a <> 'abc'");
}

#[test]
fn test_not_equal3(){
    test_predicate("a <> 'abc'", "a <> 'abc'");
}


#[test]
fn test_less(){
    test_predicate("a <3", "a < 3");
}

#[test]
fn test_less1(){
    test_predicate("a <-3", "a < -3");
}


#[test]
fn test_less2(){
    test_predicate("a < 36.266", "a < 36.266");
}


#[test]
fn test_greater(){
    test_predicate("a>3", "a > 3");
}

#[test]
fn test_greater_variable(){
    test_predicate("a>:abc", "a > :abc");
}

#[test]
fn test_greater1(){
    test_predicate("我的字段>3", "我的字段 > 3");
}

