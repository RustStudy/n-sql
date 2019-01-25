// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use std::ops::{
    Add, Mul, Sub, Div
};

#[derive(Copy, Clone, Debug)]
pub enum NumericValue {
    Integer(i64),
    Float(f64),
}

impl From<i64> for NumericValue {
    fn from(v: i64) -> Self {
        NumericValue::Integer(v)
    }
}

impl From<i32> for NumericValue {
    fn from(v: i32) -> Self {
        NumericValue::Integer(v as i64)
    }
}

impl From<f64> for NumericValue {
    fn from(v: f64) -> Self {
        NumericValue::Float(v)
    }
}

impl From<f32> for NumericValue {
    fn from(v: f32) -> Self {
        NumericValue::Float(v as f64)
    }
}

impl Add for NumericValue {
    type Output = NumericValue;

    fn add(self, other: NumericValue) -> Self::Output {
        use NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li + ri).into(),
                Float(rf) => ((li as f64) + rf).into()
            },
            Float(lf) => match other {
                Integer(ri) => (lf + (ri as f64)).into(),
                Float(rf) => (lf + rf).into()
            },
        }
    }
}

impl Sub for NumericValue {
    type Output = NumericValue;

    fn sub(self, other: NumericValue) -> Self::Output {
        use NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li - ri).into(),
                Float(rf) => ((li as f64) - rf).into()
            },
            Float(lf) => match other {
                Integer(ri) => (lf - (ri as f64)).into(),
                Float(rf) => (lf - rf).into()
            },
        }
    }
}

impl Div for NumericValue {
    type Output = NumericValue;

    fn div(self, other: NumericValue) -> Self::Output {
        use NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li / ri).into(),
                Float(rf) => ((li as f64) / rf).into()
            },
            Float(lf) => match other {
                Integer(ri) => (lf / (ri as f64)).into(),
                Float(rf) => (lf / rf).into()
            },
        }
    }
}

impl Mul for NumericValue {
    type Output = NumericValue;

    fn mul(self, other: NumericValue) -> Self::Output {
        use NumericValue::*;
        match self {
            Integer(li) => match other {
                Integer(ri) => (li * ri).into(),
                Float(rf) => ((li as f64) * rf).into()
            },
            Float(lf) => match other {
                Integer(ri) => (lf * (ri as f64)).into(),
                Float(rf) => (lf * rf).into()
            },
        }
    }
}

