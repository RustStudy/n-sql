// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

extern crate n_sql;

use n_sql::StatementParser;
use n_sql::Lexer;

mod common;

use common::*;

#[test]
fn test_union(){
    test_statement("select name from student where gender ='男' union select name from student where gender ='女'",
                   "select name from student where gender = '男' union select name from student where gender = '女'");
}

#[test]
fn test_union1(){
    test_statement("select name from student where gender ='男' union select name from student where gender ='女' skip 1 limit 2",
                   "select name from student where gender = '男' union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union2(){
    let statement = StatementParser::new()
        .parse(Lexer::new("select name from student where gender ='男' skip 2 union select name from student where gender ='女' skip 1 limit 2").tokenizer());
    assert_eq!(true, statement.is_err());
}

#[test]
fn test_union3(){
    test_statement("(select name from student where gender ='男' skip 2) union select name from student where gender ='女' skip 1 limit 2",
                   "(select name from student where gender = '男' skip 2) union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union_all(){
    test_statement("select name from student where gender ='男' union all select name from student where gender ='女'",
                   "select name from student where gender = '男' union all select name from student where gender = '女'");
}

