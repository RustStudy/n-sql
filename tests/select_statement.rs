// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

mod common;

use common::*;

#[test]
fn test_select_all(){
    test_statement("select * FROM student", "select * from student");
}


#[test]
fn test_select_one_element(){
    test_statement("select name FROM student", "select name from student");
}

#[test]
fn test_select_one_element1(){
    test_statement("select nvl(name,     'No Name') FROM student", "select nvl(name, 'No Name') from student");
}

//#[test]
//fn test_select_one_element2(){
//    let statement = StatementParser::new()
//        .parse("select 2name FROM student")
//        .unwrap();
//    assert_eq!("select 2name from student", statement.to_sql().unwrap());
//}

#[test]
fn test_select_one_element_with_alias(){
    test_statement("select nvl(name,     'No Name') as x FROM student", "select nvl(name, 'No Name') as x from student");
}

#[test]
fn test_select_one_element_with_alias1(){
    test_statement("select nvl(name,     'No Name')       x FROM student", "select nvl(name, 'No Name') as x from student");
}


#[test]
fn test_select_two_element(){
    test_statement("select name, age FROM student", "select name, age from student");
}

#[test]
fn test_select_two_element1(){
    test_statement("select t.name, t.age FROM student t", "select t.name, t.age from student as t");
}


#[test]
fn test_select_with_where(){
    test_statement("select name FROM student where age > 3", "select name from student where age > 3");
}

#[test]
fn test_select_with_group(){
    test_statement("select count(id), gender FROM student group by gender", "select count(id), gender from student group by gender");
}

#[test]
fn test_select_with_group_having(){
    test_statement("select count(id), gender FROM student group by gender having count(id) > 3", "select count(id), gender from student group by gender having count(id) > 3");
}

#[test]
fn test_select_with_order(){
    test_statement("select count(id) FROM student order by name", "select count(id) from student order by name");
}

#[test]
fn test_select_with_skip(){
    test_statement("select count(id) FROM student skip 3", "select count(id) from student skip 3");
}

#[test]
fn test_select_with_limit(){
    test_statement("select count(id) FROM student limit 3", "select count(id) from student limit 3");
}

#[test]
fn test_select_with_limit1(){
    test_statement("select count(id) FROM student limit 3   + 2", "select count(id) from student limit 3 + 2");
}

#[test]
fn test_select_full(){
    test_statement("select count(id), name FROM student where a>3 group by name having count(id) > 1 order by age skip 2 limit 3",
                   "select count(id), name from student where a > 3 group by name having count(id) > 1 order by age skip 2 limit 3");
}


#[test]
fn test_select_from_dual() {
    test_statement("select now()", "select now()");
}

#[test]
fn test_select_from_dual1() {
    test_statement("select now() from dual", "select now()");
}

#[test]
fn test_select_from_sub_query() {
    test_statement("SELECT NOW() from (select * from student) t", "select now() from (select * from student) as t");
}

#[test]
fn test_select_case_when() {
    test_statement("select case when score >= 85 then 'A' when score<85 and score >=60 then 'B' else 'C' end level from student",
                   "select case when score >= 85 then 'A' when score < 85 and score >= 60 then 'B' else 'C' end as level from student" );
}

#[test]
fn test_select_join() {
    test_statement("SELECT name from (select * from tree) t join tree t2 on t2.parent_id = t.id",
                   "select name from (select * from tree) as t join tree as t2 on (t2.parent_id = t.id)");
}


