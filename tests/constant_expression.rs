// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

extern crate n_sql;

use n_sql::{ExpressionParser};
use n_sql::Generator;
use n_sql::NumericValue;
use n_sql::ConstantValue;
use n_sql::Expression;
use n_sql::Optimizer;
use n_sql::Lexer;

mod common;
use common::*;

#[test]
fn test(){
    let left: NumericValue = 1i32.into();
    let right: NumericValue = 2.2.into();
    let result: Box<Expression> = Expression::Constant(ConstantValue::Numeric(left + right)).into();
    assert_eq!("3.2", result.to_sql().unwrap());
}

#[test]
fn test_integer(){
    test_expression("3", "3");
}


#[test]
fn test_integer_minus(){
    test_expression("-3", "-3");
}

#[test]
fn test_string(){
    test_expression(" 'abc' ", "'abc'");
}

#[test]
fn test_float(){
    test_expression("3.2", "3.2");
}
#[test]
fn test_float1(){
    test_expression(".2", "0.2")
}

#[test]
fn test_float_minus(){
    test_expression("-.2", "-0.2");
}

#[test]
fn test_optimizer() {
    let expr = ExpressionParser::new().parse(Lexer::new("2+4.7").tokenizer()).unwrap();
    assert_eq!("6.7", Box::new(expr.optimize()).to_sql().unwrap());
}

#[test]
fn test_optimizer1() {
    let expr = ExpressionParser::new().parse(Lexer::new("2+4.7*10").tokenizer()).unwrap();
    assert_eq!("49", Box::new(expr.optimize()).to_sql().unwrap())
}
#[test]
fn test_optimizer2() {
    let expr = ExpressionParser::new().parse(Lexer::new("(2+4.7)*10").tokenizer()).unwrap();
    assert_eq!("67", Box::new(expr.optimize()).to_sql().unwrap())
}