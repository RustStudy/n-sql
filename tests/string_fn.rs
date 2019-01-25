// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;

use common::*;

#[test]
fn test_concat() {
    test_expression("concat(a, b ,c)", "concat(a, b, c)");
}

#[test]
fn test_left() {
    test_expression("left(a, 3)", "left(a, 3)");
}

#[test]
fn test_upper1(){
    test_expression("UPPER(a)", "upper(a)");
}

#[test]
fn test_upper2(){
    test_expression("Upper('abc')", "upper('abc')");
}

#[test]
fn test_lower1(){
    test_expression("lower(a)", "lower(a)");
}

#[test]
fn test_lower2(){
    test_expression("lower('abc')", "lower('abc')");
}

#[test]
fn test_length1(){
    test_expression("LENGTH(a)", "length(a)");
}

#[test]
fn test_length2(){
    test_expression("Length('abc')", "length('abc')");
}

#[test]
fn test_pad_left(){
    test_expression("pad_left('abc', 3)", "pad_left('abc', 3)");
}

#[test]
fn test_pad_left1(){
    test_expression("pad_left('abc', 3, 'a')", "pad_left('abc', 3, 'a')");
}

#[test]
fn test_pad_left2(){
    test_expression("lpad('abc', 3, 'a')", "pad_left('abc', 3, 'a')");
}

#[test]
fn test_pad_right(){
    test_expression("pad_right('abc', 3)", "pad_right('abc', 3)");
}

#[test]
fn test_pad_right1(){
    test_expression("pad_right('abc', 3, 'a')", "pad_right('abc', 3, 'a')");
}

#[test]
fn test_pad_right2(){
    test_expression("rpad('abc', 3, 'a')", "pad_right('abc', 3, 'a')");
}

#[test]
fn test_replace(){
    test_expression("replace('abc', '88')", "replace('abc', '88')");
}

#[test]
fn test_reverse(){
    test_expression("reverse('abc')", "reverse('abc')");
}

#[test]
fn test_substr(){
    test_expression("substr('abc', 2)", "substr('abc', 2)");
}
#[test]
fn test_substr1(){
    test_expression("substr('abc', 1,1)", "substr('abc', 1, 1)");
}

#[test]
fn test_trim_end(){
    test_expression("trim_end('abc ')", "trim_end('abc ')");
}
#[test]
fn test_trim_end1(){
    test_expression("trim_end('abc ', 'd')", "trim_end('abc ', 'd')");
}
#[test]
fn test_trim_end2(){
    test_expression("rtrim('abc', 'a')", "trim_end('abc', 'a')");
}

#[test]
fn test_trim_end3(){
    test_expression("trim(trailing  'a' from 'abc')", "trim_end('abc', 'a')");
}

#[test]
fn test_trim_end4(){
    test_expression("trim(trailing from 'abc ')", "trim_end('abc ')");
}

#[test]
fn test_trim(){
    test_expression("trim('abc ')", "trim('abc ')");
}

#[test]
fn test_trim1(){
    test_expression("trim('abc ', 'a')", "trim('abc ', 'a')");
}
#[test]
fn test_trim2(){
    test_expression("btrim('abc ')", "trim('abc ')");
}
#[test]
fn test_trim3(){
    test_expression("btrim('abc ', 'a')", "trim('abc ', 'a')");
}
#[test]
fn test_trim_start(){
    test_expression("trim_start(' abc ')", "trim_start(' abc ')");
}

#[test]
fn test_trim_start1(){
    test_expression("trim_start(' abc ', 'a')", "trim_start(' abc ', 'a')");
}

#[test]
fn test_trim_start2(){
    test_expression("ltrim(' abc ', 'a')", "trim_start(' abc ', 'a')");
}

#[test]
fn test_trim_start3(){
    test_expression("ltrim(' abc ')", "trim_start(' abc ')");
}
#[test]
fn test_trim_start4(){
    test_expression("trim(leading from ' abc ')", "trim_start(' abc ')");
}

#[test]
fn test_trim_start5(){
    test_expression("trim(leading 'a' from ' abc ')", "trim_start(' abc ', 'a')");
}