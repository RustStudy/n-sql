// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;

use common::*;

#[test]
fn test(){
    test_statement("select name from student where gender ='男' intersect select name from student where gender ='女'",
                   "select name from student where gender = '男' intersect select name from student where gender = '女'");
}