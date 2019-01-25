// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::*;
use std::fmt::{Write, Error, Result};
use std::result;

type Formatter = String;


pub trait Visitor {
    fn visit_statement(&self, statement: &Statement, f: &mut Formatter) -> Result {
        match statement {
            Statement::Set(t) => self.visit_set_statement(t, f)
        }
    }
    fn visit_set_statement(&self, statement: &SetStatement, f: &mut Formatter) -> Result {
        use SetStatement::*;
        let is_set: fn(&Box<SetStatement>) -> bool = |s| match s.as_ref() { Select(_)=> false, _=> true};
        let is_pagination: fn(&Box<SetStatement>) -> bool = |s| match s.as_ref() { Pagination(_)=> true, _=> false};
        match statement {
            Select(t) => self.visit_select_statement(t, f),
            Union(left, right) => {
                if is_pagination(left) {
                    f.write_char('(')?;
                    self.visit_set_statement(left, f)?;
                    f.write_char(')')?;
                } else {
                    self.visit_set_statement(left, f)?;
                }
                f.write_str(" union ")?;
                if is_set(right){
                    f.write_char('(')?;
                    self.visit_set_statement(right, f)?;
                    f.write_char(')')
                } else {
                    self.visit_set_statement(right, f)
                }
            }
            UnionAll(left, right) => {
                if is_pagination(left) {
                    f.write_char('(')?;
                    self.visit_set_statement(left, f)?;
                    f.write_char(')')?;
                } else {
                    self.visit_set_statement(left, f)?;
                }
                f.write_str(" union all ")?;
                if is_set(right){
                    f.write_char('(')?;
                    self.visit_set_statement(right, f)?;
                    f.write_char(')')
                } else {
                    self.visit_set_statement(right, f)
                }
            }
            Intersect(left, right) => {
                if is_pagination(left) {
                    f.write_char('(')?;
                    self.visit_set_statement(left, f)?;
                    f.write_char(')')?;
                } else {
                    self.visit_set_statement(left, f)?;
                }
                f.write_str(" intersect ")?;
                if is_set(right){
                    f.write_char('(')?;
                    self.visit_set_statement(right, f)?;
                    f.write_char(')')
                } else {
                    self.visit_set_statement(right, f)
                }
            }
            Minus(left, right) => {
                if is_pagination(left) {
                    f.write_char('(')?;
                    self.visit_set_statement(left, f)?;
                    f.write_char(')')?;
                } else {
                    self.visit_set_statement(left, f)?;
                }
                f.write_str(" minus ")?;
                if is_set(right){
                    f.write_char('(')?;
                    self.visit_set_statement(right, f)?;
                    f.write_char(')')
                } else {
                    self.visit_set_statement(right, f)
                }
            },
            Pagination(p) => self.visit_pagination_statement(p, f)
        }
    }
    fn visit_pagination_statement(&self, pagination_statement: &Box<PaginationStatement>, f: &mut Formatter) -> Result {
        self.visit_set_statement(&pagination_statement.set, f)?;
        if let Some(ref skip) = pagination_statement.skip {
            f.write_char(' ')?;
            f.write_str("skip")?;
            f.write_char(' ')?;
            self.visit_expression(skip, f)?;
        }

        if let Some(ref limit) = pagination_statement.limit {
            f.write_char(' ')?;
            f.write_str("limit")?;
            f.write_char(' ')?;
            self.visit_expression(limit, f)?;
        }
        Ok(())
    }
    fn visit_select_statement(&self, statement: &SelectStatement, f: &mut Formatter) -> Result {
        f.write_str("select")?;
        if let Some(ref t) = statement.select_type {
            f.write_char(' ')?;
            use SelectType::*;
            match t {
                All => f.write_str("all")?,
                Distinct => f.write_str("distinct")?,
                Unique => f.write_str("unique")?
            }
        }

        f.write_char(' ')?;
        self.visit_select_elements(&statement.elements, f)?;

        if let Some(ref tables) = statement.tables {
            f.write_char(' ')?;
            f.write_str("from")?;
            f.write_char(' ')?;
            self.visit_tables(tables, f)?;

            if let Some(ref w) = statement.predicate {
                f.write_char(' ')?;
                f.write_str("where")?;
                f.write_char(' ')?;
                self.visit_predicate(w, f)?;
            }

            if let Some(ref g) = statement.group {
                f.write_char(' ')?;
                f.write_str("group")?;
                f.write_char(' ')?;
                f.write_str("by")?;
                f.write_char(' ')?;
                self.visit_expressions(&g.elements, f)?;

                if let Some(ref h) = g.having {
                    f.write_char(' ')?;
                    f.write_str("having")?;
                    f.write_char(' ')?;
                    self.visit_predicate(h, f)?;
                }
            }

            if let Some(ref s) = statement.sorts {
                f.write_char(' ')?;
                f.write_str("order")?;
                f.write_char(' ')?;
                f.write_str("by")?;
                f.write_char(' ')?;
                self.visit_sorting_elements(s, f)?;
            }
        }
        Ok(())
    }
    fn visit_select_elements(&self, select_elements: &Vec<SelectElement>, f: &mut Formatter) -> Result {
        for i in 0..select_elements.len(){
            self.visit_select_element(&select_elements[i], f)?;
            if i != select_elements.len() - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn visit_select_element(&self, select_element: &SelectElement, f: &mut Formatter) -> Result {
        use SelectElement::*;
        match select_element {
            Expression(expr, alias) => {
                self.visit_expression(&expr, f)?;
                if let Some(ref alias) = alias {
                    f.write_char(' ')?;
                    self.visit_alias(alias, f)?;
                }
                Ok(())
            },
            Asterisk(table_view_name) => {
                if let Some(table_view_name) = table_view_name {
                    self.visit_identifier(table_view_name, f)?;
                }
                f.write_char('*')
            }
        }
    }
    fn visit_sorting_elements(&self, sorting_elements: &Vec<SortingElement>, f: &mut Formatter) -> Result {
        for i in 0..sorting_elements.len(){
            self.visit_sorting_element(&sorting_elements[i], f)?;
            if i != sorting_elements.len() - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn visit_sorting_element(&self, sorting_element: &SortingElement, f: &mut Formatter) -> Result {
        self.visit_expression(&sorting_element.expr, f)?;
        if let Some(ref sorting) = sorting_element.direction {
            f.write_char(' ')?;
            use SortingDirection::*;
            match sorting {
                Ascending => f.write_str("asc"),
                Descending => f.write_str("desc")
            }
        } else { Ok(()) }
    }
    fn visit_tables(&self, tables: &Vec<Box<TableView>>, f: &mut Formatter) -> Result {
        for i in 0..tables.len(){
            self.visit_table_view(&tables[i], f)?;
            if i != tables.len() - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn visit_table_view(&self, table_view: &Box<TableView>, f: &mut Formatter) -> Result {
        use TableView::*;
        match table_view.as_ref() {
            Table(table, alias) => {
                self.visit_table(table, f)?;
                if let Some(ref alias) = alias {
                    f.write_char(' ')?;
                    self.visit_alias(alias, f)
                } else {
                    Ok(())
                }
            },
            Set(statement, alias) => {
                f.write_char('(')?;
                self.visit_set_statement(statement, f)?;
                f.write_char(')')?;
                f.write_char(' ')?;
                self.visit_alias(alias, f)
            },
            Join(j) => self.visit_join(j, f)

        }
    }
    fn visit_join(&self, join_node: &JoinNode, f: &mut Formatter) -> Result {
        self.visit_table_view(&join_node.base, f)?;
        f.write_char(' ')?;
        match join_node.join_type {
            JoinType::None => f.write_str("join")?,
            JoinType::Inner => f.write_str("inner join")?,
            JoinType::Left => f.write_str("left join")?,
            JoinType::Right => f.write_str("right join")?,
            JoinType::LeftOuter => f.write_str("left outer join")?,
            JoinType::RightOuter => f.write_str("right outer join")?,
            JoinType::Full => f.write_str("full join")?,
            JoinType::Cross => f.write_str("cross join")?,
        }

        f.write_char(' ')?;
        self.visit_table_view(&join_node.join, f)?;

        f.write_char(' ')?;
        f.write_str("on")?;
        f.write_char(' ')?;
        f.write_char('(')?;
        self.visit_predicate(&join_node.condition, f)?;
        f.write_char(')')
    }
    fn visit_alias(&self, alias: &Identifier, f: &mut Formatter) -> Result {
        f.write_str("as ")?;
        self.visit_identifier(alias, f)
    }
    fn visit_expressions(&self, expressions: &Vec<Box<Expression>>,  f: &mut Formatter) -> Result {
        for i in 0..expressions.len(){
            self.visit_expression(&expressions[i], f)?;
            if i != expressions.len() - 1 {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn visit_table(&self, table: &Table, f: &mut Formatter) -> Result {
        if let Some(ref database) = table.database {
            self.visit_identifier(database, f)?;
            f.write_char('.')?;
        }
        if let Some(ref schema) = table.schema {
            self.visit_identifier(schema, f)?;
            f.write_char('.')?;
        }

        self.visit_identifier(&table.name, f)
    }
    fn visit_expression(&self, expr: &Expression,  f: &mut Formatter) -> Result {
        match expr {
            Expression::Constant(t) => self.visit_constant(t, f),
            Expression::Column(t) => self.visit_column(t, f),
            Expression::Function(t) => self.visit_function(t, f),
            Expression::Arithmetic(t) => self.visit_arithmetic(t, f),
            Expression::CaseWhen(t) => self.visit_case_when(t, f),
            Expression::Unary(t) => self.visit_unary(t, f),
            Expression::Variable(t) => self.visit_variable(t, f)
        }
    }
    fn visit_variable(&self, expr: &Variable, f: &mut Formatter) -> Result {
        f.write_char(':')?;
        self.visit_identifier(&expr.name, f)
    }
    fn visit_predicate(&self, expr: &PredicateExpression, f: &mut Formatter) -> Result {
        match expr {
            PredicateExpression::Comparison(p)=> self.visit_comparison(p, f),
            PredicateExpression::Logical(p) => self.visit_logical(p, f),
            PredicateExpression::Not(p) => self.visit_not(p, f),
            PredicateExpression::IsNull(p) => self.visit_is_null(p, f),
            PredicateExpression::IsNotNull(p) => self.visit_is_not_null(p, f),
            PredicateExpression::In(p) => self.visit_in(p, f),
            PredicateExpression::NotIn(p) => self.visit_not_in(p, f)
        }
    }
    fn visit_function(&self, function: &Function, f: &mut Formatter) -> Result {
        match function {
            Function::Aggregate(t) => self.visit_aggregate_fn(t, f),
            Function::String(t) => self.visit_string_fn(t, f),
            Function::Cast(t) => self.visit_cast_fn(t, f),
            Function::Numeric(t) => self.visit_numeric_fn(t, f),
            Function::Now => f.write_str("now()"),
            Function::Nvl(t) => self.visit_nvl_fn(t, f),
            Function::Coalesce(t) => self.visit_coalesce_fn(t, f),
            Function::Datetime(t) => self.visit_datetime_fn(t, f)
        }
    }
    fn visit_coalesce_fn(&self, function: &CoalesceFn, f: &mut Formatter) -> Result {
        f.write_str("coalesce")?;
        f.write_char('(')?;
        let max_index = function.items.len()  - 1;
        for i in 0..function.items.len() {
            self.visit_expression(&function.items[i], f)?;
            if i < max_index {
                f.write_str(", ")?;
            }
        }
        f.write_char(')')
    }
    fn visit_numeric_fn(&self, function: &NumericFn, f: &mut Formatter) -> Result {
        match function {
            NumericFn::Abs(t) => self.visit_abs_fn(t, f),
            NumericFn::Ceil(t) => self.visit_ceil_fn(t, f),
            NumericFn::Cos(t) => self.visit_cos_fn(t, f),
            NumericFn::DenseRank(t) => self.visit_dense_rank_fn(t, f),
            NumericFn::Floor(t) => self.visit_floor_fn(t, f),
            NumericFn::Log10(t) => self.visit_log10_fn(t, f),
            NumericFn::Log(t) => self.visit_log_fn(t, f),
            NumericFn::Pow(t) => self.visit_pow_fn(t, f),
            NumericFn::Rank(t) => self.visit_rank_fn(t, f),
            NumericFn::Round(t) => self.visit_round_fn(t, f),
            NumericFn::Sign(t) => self.visit_sign_fn(t, f),
            NumericFn::Sin(t) => self.visit_sin_fn(t, f),
            NumericFn::Sqrt(t) => self.visit_sqrt_fn(t, f),
            NumericFn::Tan(t) => self.visit_tan_fn(t, f)
        }
    }
    fn visit_abs_fn(&self, function: &AbsFn, f: &mut Formatter) -> Result {
        f.write_str("abs")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_ceil_fn(&self, function: &CeilFn, f: &mut Formatter) -> Result {
        f.write_str("ceil")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_cos_fn(&self, function: &CosFn, f: &mut Formatter) -> Result {
        f.write_str("cos")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_dense_rank_fn(&self, function: &DenseRankFn, f: &mut Formatter) -> Result {
        f.write_str("dense_rank")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        if let Some(ref order) = function.order {
            f.write_str(", ")?;
            use SortingDirection::*;
            match order {
                Ascending => f.write_str("asc")?,
                Descending => f.write_str("desc")?
            }
        }
        f.write_char(')')
    }
    fn visit_rank_fn(&self, function: &RankFn, f: &mut Formatter) -> Result {
        f.write_str("rank")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        if let Some(ref order) = function.order {
            f.write_str(", ")?;
            use SortingDirection::*;
            match order {
                Ascending => f.write_str("asc")?,
                Descending => f.write_str("desc")?
            }
        }
        f.write_char(')')
    }
    fn visit_floor_fn(&self, function: &FloorFn, f: &mut Formatter) -> Result {
        f.write_str("floor")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_log10_fn(&self, function: &Log10Fn, f: &mut Formatter) -> Result {
        f.write_str("log10")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_log_fn(&self, function: &LogFn, f: &mut Formatter) -> Result {
        f.write_str("log")?;
        f.write_char('(')?;
        if let Some(ref t) = function.base {
            self.visit_expression(t, f)?;
            f.write_str(", ")?;
        }
        self.visit_expression(&function.number, f)?;
        f.write_char(')')
    }
    fn visit_pow_fn(&self, function: &PowFn, f: &mut Formatter) -> Result {
        f.write_str("pow")?;
        f.write_char('(')?;
        self.visit_expression(&function.x, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.y, f)?;
        f.write_char(')')
    }
    fn visit_round_fn(&self, function: &RoundFn, f: &mut Formatter) -> Result {
        f.write_str("round")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        if let Some(ref t) = function.precision {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }
    fn visit_sign_fn(&self, function: &SignFn, f: &mut Formatter) -> Result {
        f.write_str("sign")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_sin_fn(&self, function: &SinFn, f: &mut Formatter) -> Result {
        f.write_str("sin")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_sqrt_fn(&self, function: &SqrtFn, f: &mut Formatter) -> Result {
        f.write_str("sqrt")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_tan_fn(&self, function: &TanFn, f: &mut Formatter) -> Result {
        f.write_str("tan")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_datetime_fn(&self, function: &DatetimeFn, f: &mut Formatter) -> Result {
        use DatetimeFn::*;
        match function {
            DayAdd(t) => self.visit_day_add_fn(t, f),
            DaySub(t) => self.visit_day_sub_fn(t, f),
            HourAdd(t) => self.visit_hour_add_fn(t, f),
            HourSub(t) => self.visit_hour_sub_fn(t, f),
            MinuteAdd(t) => self.visit_minute_add_fn(t, f),
            MinuteSub(t) => self.visit_minute_sub_fn(t, f),
            MonthAdd(t) => self.visit_month_add_fn(t, f),
            MonthSub(t) => self.visit_month_sub_fn(t, f),
            SecondAdd(t) => self.visit_second_add_fn(t, f),
            SecondSub(t) => self.visit_second_sub_fn(t, f),
            YearAdd(t) => self.visit_year_add_fn(t, f),
            YearSub(t) => self.visit_year_sub_fn(t, f),
            Extract(t) => self.visit_extract_fn(t, f),
            Diff(t) => self.visit_diff_fn(t, f)
        }
    }
    fn visit_diff_fn(&self, function: &DatetimeDiffFn, f: &mut Formatter) -> Result {
        self.visit_datetime_type(&function.diff_type, f)?;
        f.write_char('_')?;
        f.write_str("diff")?;
        f.write_char('(')?;
        self.visit_expression(&function.start, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.end, f)?;
        f.write_char(')')
    }
    fn visit_extract_fn(&self, function: &ExtractFn, f: &mut Formatter) -> Result {
        self.visit_datetime_type(&function.extract_type, f)?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_datetime_type(&self, datetime_type: &DatetimeType, f: &mut Formatter) -> Result {
        use DatetimeType::*;
        match datetime_type {
            Year => f.write_str("year"),
            Month => f.write_str("month"),
            Day => f.write_str("day"),
            Hour => f.write_str("hour"),
            Minute => f.write_str("minute"),
            Second => f.write_str("second")
        }
    }
    fn visit_day_add_fn(&self, function: &DayAddFn, f: &mut Formatter) -> Result {
        f.write_str("day_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_day_sub_fn(&self, function: &DaySubFn, f: &mut Formatter) -> Result {
        f.write_str("day_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_hour_add_fn(&self, function: &HourAddFn, f: &mut Formatter) -> Result {
        f.write_str("hour_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_hour_sub_fn(&self, function: &HourSubFn, f: &mut Formatter) -> Result {
        f.write_str("hour_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_minute_add_fn(&self, function: &MinuteAddFn, f: &mut Formatter) -> Result {
        f.write_str("minute_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_minute_sub_fn(&self, function: &MinuteSubFn, f: &mut Formatter) -> Result {
        f.write_str("minute_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_month_add_fn(&self, function: &MonthAddFn, f: &mut Formatter) -> Result {
        f.write_str("month_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_month_sub_fn(&self, function: &MonthSubFn, f: &mut Formatter) -> Result {
        f.write_str("month_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_second_add_fn(&self, function: &SecondAddFn, f: &mut Formatter) -> Result {
        f.write_str("second_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_second_sub_fn(&self, function: &SecondSubFn, f: &mut Formatter) -> Result {
        f.write_str("second_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_year_add_fn(&self, function: &YearAddFn, f: &mut Formatter) -> Result {
        f.write_str("year_add")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_year_sub_fn(&self, function: &YearSubFn, f: &mut Formatter) -> Result {
        f.write_str("year_sub")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.offset, f)?;
        f.write_char(')')
    }
    fn visit_nvl_fn(&self, function: &Box<NvlFn>, f: &mut Formatter) -> Result {
        f.write_str("nvl")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.default, f)?;
        f.write_char(')')
    }
    fn visit_cast_fn(&self, function: &Box<CastFn>, f: &mut Formatter) -> Result {
        self.visit_expression(&function.expr, f)?;
        f.write_str("::")?;
        f.write_str(&function.data_type.data_type)
    }
    fn visit_arithmetic(&self, expr: &ArithmeticExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.left, f)?;

        let is_add_or_sub: fn(&Expression) -> bool = | e| {
            match e {
                Expression::Arithmetic(t)  => {
                    match t.op {
                        ArithmeticOperator::Add | ArithmeticOperator::Sub => true,
                        _ => false
                    }
                },
                _ => false
            }
        };

        match expr.op {
            ArithmeticOperator::Add => {
                f.write_str(" + ")?;
                self.visit_expression(&expr.right, f)
            },
            ArithmeticOperator::Sub => {
                f.write_str(" - ")?;
                self.visit_expression(&expr.right, f)
            },
            ArithmeticOperator::Mul => {
                f.write_str(" * ")?;
                self.visit_expression(&expr.right, f)
            },
            ArithmeticOperator::Div => {
                if is_add_or_sub(expr.right.as_ref()) {
                    f.write_str(" / ")?;
                    f.write_char('(')?;
                    self.visit_expression(&expr.right, f)?;
                    f.write_char(')')
                } else {
                    f.write_str(" / ")?;
                    self.visit_expression(&expr.right, f)
                }
            },
        }

    }
    fn visit_case_when(&self, expr: &CaseWhenExpression, f: &mut Formatter) -> Result {
        match expr {
            CaseWhenExpression::Simple(t) => self.visit_simple_case_when(t, f),
            CaseWhenExpression::Searched(t) => self.visit_searched_case_when(t, f),
        }
    }
    fn visit_unary(&self, expr: &UnaryExpression, f: &mut Formatter) -> Result {
        match expr {
            UnaryExpression::Plus(t) => {
                f.write_char('+')?;
                self.visit_expression(t, f)
            },
            UnaryExpression::Minus(t) => {
                f.write_char('-')?;
                self.visit_expression(t, f)
            }
        }
    }
    fn visit_simple_case_when(&self, expr: &SimpleCaseWhenExpression, f: &mut Formatter) -> Result {
        f.write_str("case")?;
        f.write_char(' ')?;
        self.visit_expression(&expr.expr, f)?;
        for (when ,then) in &expr.matches {
            f.write_char(' ')?;
            f.write_str("when")?;
            f.write_char(' ')?;
            self.visit_expression(when, f)?;

            f.write_char(' ')?;
            f.write_str("then")?;
            f.write_char(' ')?;
            self.visit_expression(then, f)?;
        }

        if let Some(ref default) = expr.default {
            f.write_char(' ')?;
            f.write_str("else")?;
            f.write_char(' ')?;
            self.visit_expression(default, f)?;
        }
        f.write_char(' ')?;
        f.write_str("end")
    }
    fn visit_searched_case_when(&self, expr: &SearchedCaseWhenExpression, f: &mut Formatter) -> Result {
        f.write_str("case")?;
        for (when ,then) in &expr.matches {
            f.write_char(' ')?;
            f.write_str("when")?;
            f.write_char(' ')?;
            self.visit_predicate(when, f)?;

            f.write_char(' ')?;
            f.write_str("then")?;
            f.write_char(' ')?;
            self.visit_expression(then, f)?;
        }

        if let Some(ref default) = expr.default {
            f.write_char(' ')?;
            f.write_str("else")?;
            f.write_char(' ')?;
            self.visit_expression(default, f)?;
        }

        f.write_char(' ')?;
        f.write_str("end")
    }
    fn visit_aggregate_fn(&self, function: &AggregateFn, f: &mut Formatter) -> Result {
        match function {
            AggregateFn::Avg(t) => self.visit_avg_fn(t, f),
            AggregateFn::Count(t) => self.visit_count_fn(t, f),
            AggregateFn::Max(t) => self.visit_max_fn(t, f),
            AggregateFn::Median(t) => self.visit_median_fn(t, f),
            AggregateFn::Min(t) => self.visit_min_fn(t, f),
            AggregateFn::Sum(t) => self.visit_sum_fn(t, f),
            AggregateFn::Stddev(t) => self.visit_stddev_fn(t, f)
        }
    }
    fn visit_aggregate_type(&self, aggregate_type: &AggregateType, f: &mut Formatter) -> Result {
        use AggregateType::*;
        match aggregate_type {
            Distinct => f.write_str("distinct"),
            All => f.write_str("all"),
            Unique => f.write_str("unique")
        }
    }
    fn visit_stddev_fn(&self, function: &StddevFn, f: &mut Formatter) -> Result {
        f.write_str("stddev")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_avg_fn(&self, function: &AvgFn, f: &mut Formatter) -> Result {
        f.write_str("avg")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_count_fn(&self, function: &CountFn, f: &mut Formatter) -> Result {
        f.write_str("count")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_max_fn(&self, function: &MaxFn, f: &mut Formatter) -> Result {
        f.write_str("max")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_median_fn(&self, function: &MedianFn, f: &mut Formatter) -> Result {
        f.write_str("median")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_min_fn(&self, function: &MinFn, f: &mut Formatter) -> Result {
        f.write_str("min")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_sum_fn(&self, function: &SumFn, f: &mut Formatter) -> Result {
        f.write_str("sum")?;
        f.write_char('(')?;
        if let Some(ref t) = function.aggregate_type {
            self.visit_aggregate_type(t, f)?;
            f.write_char(' ')?;
        }
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
    fn visit_string_fn(&self, function: &StringFn, f: &mut Formatter) -> Result {
        match function {
            StringFn::Concat(t) => self.visit_concat_fn(t, f),
            StringFn::Left(t) => self.visit_left_fn(t, f),
            StringFn::Length(t) => self.visit_length_fn(t, f),
            StringFn::Lower(t) => self.visit_lower_fn(t, f),
            StringFn::PadLeft(t) => self.visit_pad_left_fn(t, f),
            StringFn::PadRight(t) => self.visit_pad_right_fn(t, f),
            StringFn::Replace(t) => self.visit_replace_fn(t, f),
            StringFn::Reverse(t) => self.visit_reverse_fn(t, f),
            StringFn::Right(t) => self.visit_right_fn(t, f),
            StringFn::Substr(t) => self.visit_substr_fn(t, f),
            StringFn::Trim(t) => self.visit_trim_fn(t, f),
            StringFn::Upper(t) => self.visit_upper_fn(t, f),
        }
    }
    fn visit_concat_fn(&self, function: &ConcatFn, f: &mut Formatter) -> Result {
        f.write_str("concat")?;
        f.write_char('(')?;
        let max_index= function.items.len() - 1;
        for i in 0..function.items.len() {
            self.visit_expression(&function.items[i], f)?;
            if i < max_index {
                f.write_str(", ")?;
            }
        }
        f.write_char(')')
    }
    fn visit_left_fn(&self, function: &LeftFn, f: &mut Formatter) -> Result {
        f.write_str("left")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.length, f)?;
        f.write_char(')')
    }
    fn visit_length_fn(&self, function: &LengthFn, f: &mut Formatter) -> Result {
        f.write_str("length")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_char(')')
    }
    fn visit_lower_fn(&self, function: &LowerFn, f: &mut Formatter) -> Result {
        f.write_str("lower")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_char(')')
    }
    fn visit_pad_left_fn(&self, function: &PadLeftFn, f: &mut Formatter) -> Result {
        f.write_str("pad_left")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.length, f)?;
        if let Some(ref t) = function.pad_text {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }
    fn visit_pad_right_fn(&self, function: &PadRightFn, f: &mut Formatter) -> Result {
        f.write_str("pad_right")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.length, f)?;
        if let Some(ref t) = function.pad_text {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }
    fn visit_replace_fn(&self, function: &ReplaceFn, f: &mut Formatter) -> Result {
        f.write_str("replace")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.replace_text, f)?;
        f.write_char(')')
    }
    fn visit_reverse_fn(&self, function: &ReverseFn, f: &mut Formatter) -> Result {
        f.write_str("reverse")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_char(')')
    }
    fn visit_right_fn(&self, function: &RightFn, f: &mut Formatter) -> Result {
        f.write_str("right")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.length, f)?;
        f.write_char(')')
    }
    fn visit_substr_fn(&self, function: &SubstrFn, f: &mut Formatter) -> Result {
        f.write_str("substr")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.start, f)?;
        if let Some(ref t) = function.length {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }

    fn visit_trim_fn(&self, function: &TrimFn, f: &mut Formatter) -> Result {
        use TrimType::*;
        match function.trim_type {
            Both => f.write_str("trim")?,
            Leading => f.write_str("trim_start")?,
            Trailing => f.write_str("trim_end")?,
        }
        ;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        if let Some(ref t) = function.trim_text {
            f.write_str(", ")?;
            self.visit_expression(t, f)?;
        }
        f.write_char(')')
    }
    fn visit_upper_fn(&self, function: &UpperFn, f: &mut Formatter) -> Result {
        f.write_str("upper")?;
        f.write_char('(')?;
        self.visit_expression(&function.text, f)?;
        f.write_char(')')
    }
    fn visit_column(&self, expr: &Column, f: &mut Formatter) -> Result {
        if let Some(ref schema) = expr.schema {
            self.visit_identifier(schema, f)?;
            f.write_char('.')?;
        }

        if let Some(ref table) = expr.table {
            self.visit_identifier(table, f)?;
            f.write_char('.')?;
        }
        write!(f, "{}", expr.name)
    }
    fn visit_constant(&self, expr: &ConstantValue, f: &mut Formatter) -> Result {
        match expr {
            ConstantValue::String(t) => self.visit_string(t, f),
            ConstantValue::Numeric(t) => self.visit_numeric(t, f),
            ConstantValue::Null => self.visit_null(f),
        }
    }
    fn visit_comparison(&self, expr: &ComparisonExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.left, f)?;
        f.write_str(" ")?;
        self.visit_comparison_operator(&expr.op, f)?;
        f.write_str(" ")?;
        self.visit_expression(&expr.right, f)
    }
    fn visit_comparison_operator(&self, op: &ComparisonOperator, f: &mut Formatter) -> Result {
        use self::ComparisonOperator::*;
        match op {
            Equal => f.write_str("="),
            NotEqual => f.write_str("<>"),
            Less => f.write_str("<"),
            Greater => f.write_str(">"),
            LessOrEqual => f.write_str("<="),
            GreaterOrEqual => f.write_str(">="),
        }
    }
    fn visit_logical(&self, expr: &LogicalExpression, f: &mut Formatter) -> Result {
        self.visit_predicate(&expr.left, f)?;
        f.write_str(" ")?;
        self.visit_logical_operator(&expr.op, f)?;
        f.write_str(" ")?;

        if let PredicateExpression::Logical(ref p) = expr.right.as_ref() {
            if let LogicalOperator::Or = p.op {
                f.write_char('(')?;
                self.visit_logical(p, f)?;
                f.write_char(')')
            } else { self.visit_logical(p, f) }
        } else {
            self.visit_predicate(&expr.right, f)
        }
    }
    fn visit_logical_operator(&self, op: &LogicalOperator, f: &mut Formatter) -> Result {
        use LogicalOperator::*;
        match op {
            And => f.write_str("and"),
            Or => f.write_str("or")
        }
    }
    fn visit_not(&self, expr: &NotExpression, f: &mut Formatter) -> Result {
        f.write_str("not(")?;
        self.visit_predicate(&expr.expr, f)?;
        f.write_str(")")
    }
    fn visit_is_null(&self, expr: &IsNullExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.expr, f)?;
        write!(f, " is null")
    }
    fn visit_is_not_null(&self, expr: &IsNotNullExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.expr, f)?;
        write!(f, " is not null")
    }
    fn visit_in(&self, expr: &InExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.left, f)?;
        write!(f, " in ")?;
        f.write_char('(')?;
        self.visit_in_elements(&expr.values, f)?;
        f.write_char(')')
    }
    fn visit_not_in(&self, expr: &NotInExpression, f: &mut Formatter) -> Result {
        self.visit_expression(&expr.left, f)?;
        write!(f, " not in ")?;
        f.write_char('(')?;
        self.visit_in_elements(&expr.values, f)?;
        f.write_char(')')
    }
    fn visit_in_elements(&self, expr: &InElements, f: &mut Formatter) -> Result {
        match expr {
            InElements::Values(v) => {
                for i in 0..v.len() {
                    self.visit_expression(&v[i], f)?;
                    if i + 1 != v.len() {
                        f.write_str(", ")?;
                    }
                }
                Ok(())
            },
            InElements::Set(s) => {
                self.visit_set_statement(s, f)
            }
        }
    }
    fn visit_string(&self, v: &StringValue, f: &mut Formatter) -> Result {
        write!(f, "'{}'", v.value)
    }
    fn visit_numeric(&self, v: &NumericValue, f: &mut Formatter) -> Result{
        match v {
            NumericValue::Integer(t) => write!(f, "{}", t),
            NumericValue::Float(t) => write!(f, "{}", t),
        }
    }
    fn visit_null(&self, f: &mut Formatter) -> Result{
        write!(f, "null")
    }
    fn visit_identifier(&self, identifier: &Identifier, f: &mut Formatter) -> Result {
        write!(f, "{}", identifier)
    }
}
