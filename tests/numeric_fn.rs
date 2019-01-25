// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;

use common::*;

#[test]
fn test_abs(){
    test_expression("abs(-6)", "abs(-6)");
}

#[test]
fn test_ceil(){
    test_expression("ceil(-6.8)", "ceil(-6.8)");
}

#[test]
fn test_cos(){
    test_expression("cos(6.8)", "cos(6.8)");
}

#[test]
fn test_dense_rank(){
    test_expression("dense_rank(a)", "dense_rank(a)");
}
#[test]
fn test_dense_rank1(){
    test_expression("dense_rank(a  , asc)", "dense_rank(a, asc)");
}
#[test]
fn test_dense_rank2(){
    test_expression("dense_rank(a  , desc)", "dense_rank(a, desc)");
}

#[test]
fn test_floor(){
    test_expression("floor(-6.8)", "floor(-6.8)");
}

#[test]
fn test_log10(){
    test_expression("log10(6.8)", "log10(6.8)");
}

#[test]
fn test_log(){
    test_expression("log(6.8)", "log(6.8)");
}
#[test]
fn test_log1(){
    test_expression("log(6.8, 2)", "log(6.8, 2)");
}

#[test]
fn test_pow() {
    test_expression("pow(6.8, 2)", "pow(6.8, 2)");
}
#[test]
fn test_pow1(){
    test_expression("power(6.8, 2)", "pow(6.8, 2)");
}
#[test]
fn test_rank(){
    test_expression("rank(a)", "rank(a)");
}
#[test]
fn test_rank1(){
    test_expression("rank(a  , asc)", "rank(a, asc)");
}
#[test]
fn test_rank2(){
    test_expression("rank(a  , desc)", "rank(a, desc)");
}

#[test]
fn test_round(){
    test_expression("round(6.855, 2)", "round(6.855, 2)");
}

#[test]
fn test_sign(){
    test_expression("sign(-6.855)", "sign(-6.855)");
}

#[test]
fn test_sin(){
    test_expression("sin(6.855)", "sin(6.855)");
}

#[test]
fn test_sqrt(){
    test_expression("sqrt(6.855)", "sqrt(6.855)");
}

#[test]
fn test_tan(){
    test_expression("tan(6.855)", "tan(6.855)");
}