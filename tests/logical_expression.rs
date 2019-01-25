// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.


mod common;

use common::*;

#[test]
fn test_and(){
    test_predicate("a = 3 And c is null", "a = 3 and c is null");
}

#[test]
fn test_or(){
    test_predicate("a = 3 or c < 5", "a = 3 or c < 5");
}

#[test]
fn test_priority(){
    test_predicate("a =3   and (c < 5 or c= 7)", "a = 3 and (c < 5 or c = 7)");
}


#[test]
fn test_superfluous_brackets(){
    test_predicate("(a =3 )  and ((c < 5 or c= 7))", "a = 3 and (c < 5 or c = 7)");
}


#[test]
fn test_superfluous_brackets1(){
    test_predicate("(a =3 )  and (((c < 5 or c= 7)))", "a = 3 and (c < 5 or c = 7)");
}