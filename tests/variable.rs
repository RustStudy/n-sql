// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod common;

use common::*;

#[test]
fn test1(){
    test_predicate("a = :  m", "a = :m");
}


#[test]
fn test2(){
    test_statement("select * from student where a in (:a,:b,:c)", "select * from student where a in (:a, :b, :c)");
}