// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod common;

use common::*;

#[test]
fn test_is_null(){
    test_predicate("a is null", "a is null");
}

#[test]
fn test_is_not_null(){
    test_predicate("a is not null", "a is not null");
}