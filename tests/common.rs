// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
#![allow(dead_code)]

extern crate n_sql;
use self::n_sql::{ExpressionParser, StatementParser, PredicateParser, Lexer, Generator};


pub fn test_expression(left: &str, right:&str){
    let expr = ExpressionParser::new()
        .parse(Lexer::new(left).tokenizer())
    .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}

pub fn test_statement(left: &str, right:&str){
    let expr = StatementParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}
pub fn test_predicate(left: &str, right:&str){
    let expr = PredicateParser::new()
        .parse(Lexer::new(left).tokenizer())
        .unwrap();
    assert_eq!(right, expr.to_sql().unwrap());
}