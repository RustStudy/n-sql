// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;
use ast::Identifier;
use ast::PredicateExpression;
use ast::TableView;
use ast::SortingDirection;

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub select_type: Option<SelectType>,
    pub elements: Vec<SelectElement>,
    pub tables: Option<Vec<Box<TableView>>>,
    pub predicate: Option<Box<PredicateExpression>>,
    pub group: Option<GroupElements>,
    pub sorts: Option<Vec<SortingElement>>,
}

impl SelectStatement {
    pub fn new(
        select_type: Option<SelectType>,
        elements: Vec<SelectElement>,
        tables: Option<Vec<Box<TableView>>>,
        predicate: Option<Box<PredicateExpression>>,
        group: Option<GroupElements>,
        sorts: Option<Vec<SortingElement>>,
    ) -> SelectStatement {
        SelectStatement {
            select_type,
            elements,
            tables,
            predicate,
            group,
            sorts
        }
    }
}

#[derive(Clone, Debug)]
pub enum  SelectElement {
    Expression(Box<Expression>, Option<Identifier>),
    Asterisk(Option<Identifier>),
}

#[derive(Clone, Debug)]
pub struct GroupElements {
    pub elements: Vec<Box<Expression>>,
    pub having: Option<Box<PredicateExpression>>,
}

impl GroupElements {
    pub fn new(elements: Vec<Box<Expression>>, having: Option<Box<PredicateExpression>>) -> GroupElements {
        GroupElements { elements, having }
    }
}
#[derive(Clone, Debug)]
pub struct SortingElement {
    pub expr: Box<Expression>,
    pub direction: Option<SortingDirection>,
}

impl SortingElement {
    pub fn new(expr: Box<Expression>, direction: Option<SortingDirection>) -> SortingElement {
        SortingElement { expr, direction }
    }
}
#[derive(Clone, Debug, Copy)]
pub enum SelectType {
    All,
    Distinct,
    Unique
}

mod select_builder {
    struct Builder {
    }
    pub trait IHasFrom {
        fn from() -> Box<IKeySelect>;
    }
    pub trait IKeySelect {
    }
    pub trait IkeyFrom{
    }
    fn select() -> Box<IKeySelect> {
        unimplemented!()
    }
}
