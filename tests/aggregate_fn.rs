// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;
use common::*;

#[test]
fn test_avg1(){
    test_expression("avg(a)", "avg(a)");
}

#[test]
fn test_avg2(){
    test_predicate("avg(a) >9", "avg(a) > 9");
}

#[test]
fn test_avg3(){
    test_expression("avg(distinct a)", "avg(distinct a)");
}

#[test]
fn test_count(){
    test_expression("count(a)", "count(a)");
}

#[test]
fn test_count1(){
    test_expression("count(unique a)", "count(unique a)");
}


#[test]
fn test_sum(){
    test_expression("sum(all a)", "sum(all a)");
}

#[test]
fn test_sum1(){
    test_expression("sum(a)", "sum(a)");
}


#[test]
fn test_stddev(){
    test_expression("stddev(a)", "stddev(a)");
}


