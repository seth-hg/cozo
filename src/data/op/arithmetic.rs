use crate::data::eval::EvalError;
use crate::data::op::{extract_two_args, Op};
use crate::data::value::Value;
use std::result;

type Result<T> = result::Result<T, EvalError>;

pub(crate) struct OpAdd;

impl OpAdd {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => (l + r).into(),
            (Value::Float(l), Value::Int(r)) => (l + (r as f64)).into(),
            (Value::Int(l), Value::Float(r)) => ((l as f64) + r.into_inner()).into(),
            (Value::Float(l), Value::Float(r)) => (l.into_inner() + r.into_inner()).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_ADD: &str = "+";

impl Op for OpAdd {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_ADD
    }

    fn non_null_args(&self) -> bool {
        true
    }
    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpSub;

impl OpSub {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => (l - r).into(),
            (Value::Float(l), Value::Int(r)) => (l - (r as f64)).into(),
            (Value::Int(l), Value::Float(r)) => ((l as f64) - r.into_inner()).into(),
            (Value::Float(l), Value::Float(r)) => (l.into_inner() - r.into_inner()).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_SUB: &str = "-";

impl Op for OpSub {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_SUB
    }

    fn non_null_args(&self) -> bool {
        true
    }
    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpMul;

impl OpMul {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => (l * r).into(),
            (Value::Float(l), Value::Int(r)) => (l * (r as f64)).into(),
            (Value::Int(l), Value::Float(r)) => ((l as f64) * r.into_inner()).into(),
            (Value::Float(l), Value::Float(r)) => (l.into_inner() * r.into_inner()).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_MUL: &str = "*";

impl Op for OpMul {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_MUL
    }

    fn non_null_args(&self) -> bool {
        true
    }

    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpDiv;

impl OpDiv {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => (l as f64 / r as f64).into(),
            (Value::Float(l), Value::Int(r)) => (l / (r as f64)).into(),
            (Value::Int(l), Value::Float(r)) => ((l as f64) / r.into_inner()).into(),
            (Value::Float(l), Value::Float(r)) => (l.into_inner() / r.into_inner()).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_DIV: &str = "/";

impl Op for OpDiv {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_DIV
    }

    fn non_null_args(&self) -> bool {
        true
    }

    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpMod;

impl OpMod {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => (l % r).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_MOD: &str = "%";

impl Op for OpMod {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_MOD
    }

    fn non_null_args(&self) -> bool {
        true
    }

    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpPow;

impl OpPow {
    pub(crate) fn eval_two_non_null<'a>(
        &self,
        left: Value<'a>,
        right: Value<'a>,
    ) -> Result<Value<'a>> {
        let res: Value = match (left, right) {
            (Value::Int(l), Value::Int(r)) => ((l as f64).powf(r as f64)).into(),
            (Value::Float(l), Value::Int(r)) => ((l.into_inner()).powf(r as f64)).into(),
            (Value::Int(l), Value::Float(r)) => ((l as f64).powf(r.into_inner())).into(),
            (Value::Float(l), Value::Float(r)) => ((l.into_inner()).powf(r.into_inner())).into(),
            (l, r) => {
                return Err(EvalError::OpTypeMismatch(
                    self.name().to_string(),
                    vec![l.to_static(), r.to_static()],
                ));
            }
        };
        Ok(res)
    }
}

pub(crate) const NAME_OP_POW: &str = "**";

impl Op for OpPow {
    fn arity(&self) -> Option<usize> {
        Some(2)
    }

    fn has_side_effect(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        NAME_OP_POW
    }

    fn non_null_args(&self) -> bool {
        true
    }
    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        let (left, right) = extract_two_args(args);
        self.eval_two_non_null(left, right)
    }
}

pub(crate) struct OpMinus;

impl OpMinus {
    pub(crate) fn eval_one_non_null<'a>(&self, arg: Value<'a>) -> Result<Value<'a>> {
        match arg {
            Value::Int(i) => Ok((-i).into()),
            Value::Float(i) => Ok((-i).into()),
            v => Err(EvalError::OpTypeMismatch(
                self.name().to_string(),
                vec![v.to_static()],
            )),
        }
    }
}

pub(crate) const NAME_OP_MINUS: &str = "--";

impl Op for OpMinus {
    fn arity(&self) -> Option<usize> {
        Some(1)
    }
    fn has_side_effect(&self) -> bool {
        false
    }
    fn name(&self) -> &str {
        NAME_OP_MINUS
    }
    fn non_null_args(&self) -> bool {
        true
    }
    fn eval<'a>(&self, args: Vec<Value<'a>>) -> crate::data::op::Result<Value<'a>> {
        self.eval_one_non_null(args.into_iter().next().unwrap())
    }
}
