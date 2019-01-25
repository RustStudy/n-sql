// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod common;

use common::*;

#[test]
fn test_nvl(){
    test_expression("nvl(a, 'no value')", "nvl(a, 'no value')");
}

#[test]
fn test_nvl1(){
    test_expression("NVL(a, 'no value')", "nvl(a, 'no value')");
}

#[test]
fn test_nvl2(){
    test_expression("NVL(a,      'no value')", "nvl(a, 'no value')");
}


#[test]
fn test_nvl3(){
    test_expression("NVL(a,      123)", "nvl(a, 123)");
}


#[test]
fn test_nvl4(){
    test_expression("NVL(a,      -123)", "nvl(a, -123)");
}

#[test]
fn test_coalesce(){
    test_expression("COALESCE(a,b,c)", "coalesce(a, b, c)");
}