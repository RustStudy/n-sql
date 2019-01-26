// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::*;

pub trait Optimizer<T> {
    fn optimize(&self) -> T;
}

enum Result<T, A> {
    Ok(T),
    Alternative(A)
}

trait ConstantOptimizer {
    fn visit_arithmetic(&self, expr: &ArithmeticExpression) -> Expression {
        let left = self.visit_expression(expr.left.as_ref());
        let right = self.visit_expression(expr.right.as_ref());
        if let Expression::Constant(lc) = left {
            if let ConstantValue::Numeric(ln) = lc.clone() {
                if let Expression::Constant(rc) = right {
                    if let ConstantValue::Numeric(rn) = rc.clone() {
                        return Expression::Constant(ConstantValue::Numeric(
                            match expr.op {
                                ArithmeticOperator::Div => ln / rn,
                                ArithmeticOperator::Mul => ln * rn,
                                ArithmeticOperator::Sub => ln - rn,
                                ArithmeticOperator::Add => ln + rn
                            }
                        ))
                    }
                }

            }
        }
        Expression::Arithmetic(expr.clone())
    }
    fn visit_expression(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Constant(_) => expr.clone(),
            Expression::Arithmetic(ref arithmetic) => self.visit_arithmetic(arithmetic),
            Expression::Function(t) => self.visit_function(t),
            _ => expr.clone()
        }
    }
    fn visit_function(&self, function: &Function) -> Expression {
        match function {
            Function::String(t) => self.visit_string_fn(t),
            _ => Expression::Function(function.clone())
        }
    }
    fn visit_string_fn(&self, function: &StringFn) -> Expression {
        match function {
            StringFn::Substr(t) => {

                unimplemented!()
            },
            _ => Expression::Function(Function::String(function.clone()))
        }
    }
    fn try_eval_string_value(&self, expr: &Expression) -> Result<StringValue, Expression> {
        if let Expression::Constant(t) = expr {
            if let ConstantValue::String(t) = t {
                return Result::Ok(t.clone())
            }
        }
        Result::Alternative(expr.clone())
    }
}


impl ConstantOptimizer for Expression {
}

impl Optimizer<Expression> for Expression {
    fn optimize(&self) -> Expression {
        self.visit_expression(self)
    }
}