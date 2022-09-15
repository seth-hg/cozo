use std::collections::BTreeMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use miette::{bail, Diagnostic, ensure, Result};
use pest::prec_climber::{Operator, PrecClimber};
use smartstring::{LazyCompact, SmartString};
use thiserror::Error;

use crate::data::expr::{Expr, get_op};
use crate::data::functions::{
    OP_ADD, OP_AND, OP_CONCAT, OP_DIV, OP_EQ, OP_GE, OP_GT, OP_LE, OP_LIST, OP_LT, OP_MINUS,
    OP_MOD, OP_MUL, OP_NEGATE, OP_NEQ, OP_OR, OP_POW, OP_SUB,
};
use crate::data::symb::Symbol;
use crate::data::value::DataValue;
use crate::parse::{ExtractSpan, Pair, Rule, SourceSpan};

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::Assoc::*;

        PrecClimber::new(vec![
            Operator::new(Rule::op_or, Left),
            Operator::new(Rule::op_and, Left),
            Operator::new(Rule::op_gt, Left)
                | Operator::new(Rule::op_lt, Left)
                | Operator::new(Rule::op_ge, Left)
                | Operator::new(Rule::op_le, Left),
            Operator::new(Rule::op_mod, Left),
            Operator::new(Rule::op_eq, Left) | Operator::new(Rule::op_ne, Left),
            Operator::new(Rule::op_add, Left)
                | Operator::new(Rule::op_sub, Left)
                | Operator::new(Rule::op_concat, Left),
            Operator::new(Rule::op_mul, Left) | Operator::new(Rule::op_div, Left),
            Operator::new(Rule::op_pow, Right),
        ])
    };
}

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid expression encountered")]
#[diagnostic(code(parser::invalid_expression))]
pub(crate) struct InvalidExpression(#[label] pub(crate) SourceSpan);

pub(crate) fn build_expr(pair: Pair<'_>, param_pool: &BTreeMap<String, DataValue>) -> Result<Expr> {
    ensure!(pair.as_rule() == Rule::expr, InvalidExpression(pair.extract_span()));

    PREC_CLIMBER.climb(
        pair.into_inner(),
        |v| build_unary(v, param_pool),
        build_expr_infix,
    )
}

fn build_expr_infix(lhs: Result<Expr>, op: Pair<'_>, rhs: Result<Expr>) -> Result<Expr> {
    let args = vec![lhs?, rhs?];
    let op = match op.as_rule() {
        Rule::op_add => &OP_ADD,
        Rule::op_sub => &OP_SUB,
        Rule::op_mul => &OP_MUL,
        Rule::op_div => &OP_DIV,
        Rule::op_mod => &OP_MOD,
        Rule::op_pow => &OP_POW,
        Rule::op_eq => &OP_EQ,
        Rule::op_ne => &OP_NEQ,
        Rule::op_gt => &OP_GT,
        Rule::op_ge => &OP_GE,
        Rule::op_lt => &OP_LT,
        Rule::op_le => &OP_LE,
        Rule::op_concat => &OP_CONCAT,
        Rule::op_or => &OP_OR,
        Rule::op_and => &OP_AND,
        _ => unreachable!(),
    };
    let start = args[0].span().0;
    let end = args[1].span().0 + args[1].span().1;
    let length = end - start;
    Ok(Expr::Apply {
        op,
        args: args.into(),
        span: SourceSpan(start, length),
    })
}

fn build_unary(pair: Pair<'_>, param_pool: &BTreeMap<String, DataValue>) -> Result<Expr> {
    let span = pair.extract_span();
    let s = pair.as_str();
    let mut inner = pair.into_inner();
    let p = inner.next().unwrap();
    let op = p.as_rule();
    Ok(match op {
        Rule::term => build_unary(p, param_pool)?,
        Rule::var => Expr::Binding {
            var: Symbol::new(s, p.extract_span()),
            tuple_pos: None,
        },
        Rule::param => {
            #[derive(Error, Diagnostic, Debug)]
            #[error("Required parameter {0} not found")]
            #[diagnostic(code(parser::param_not_found))]
            struct ParamNotFoundError(String, #[label] SourceSpan);

            let param_str = s.strip_prefix('$').unwrap();
            Expr::Const {
                val: param_pool
                    .get(param_str)
                    .ok_or_else(|| ParamNotFoundError(param_str.to_string(), span))?
                    .clone(),
                span,
            }
        }
        Rule::minus => {
            let inner = build_unary(inner.next().unwrap(), param_pool)?;
            Expr::Apply {
                op: &OP_MINUS,
                args: [inner].into(),
                span,
            }
        }
        Rule::negate => {
            let inner = build_unary(inner.next().unwrap(), param_pool)?;
            Expr::Apply {
                op: &OP_NEGATE,
                args: [inner].into(),
                span,
            }
        }
        Rule::pos_int => {
            #[derive(Error, Diagnostic, Debug)]
            #[error("Cannot parse integer")]
            #[diagnostic(code(parser::bad_pos_int))]
            struct BadIntError(#[label] SourceSpan);

            let i = s
                .replace('_', "")
                .parse::<i64>()
                .map_err(|_| BadIntError(span))?;
            Expr::Const {
                val: DataValue::from(i),
                span,
            }
        }
        Rule::hex_pos_int => {
            let i = parse_int(s, 16);
            Expr::Const {
                val: DataValue::from(i),
                span,
            }
        }
        Rule::octo_pos_int => {
            let i = parse_int(s, 8);
            Expr::Const {
                val: DataValue::from(i),
                span,
            }
        }
        Rule::bin_pos_int => {
            let i = parse_int(s, 2);
            Expr::Const {
                val: DataValue::from(i),
                span,
            }
        }
        Rule::dot_float | Rule::sci_float => {
            #[derive(Error, Diagnostic, Debug)]
            #[error("Cannot parse float")]
            #[diagnostic(code(parser::bad_float))]
            struct BadFloatError(#[label] SourceSpan);

            let f = s
                .replace('_', "")
                .parse::<f64>()
                .map_err(|_| BadFloatError(span))?;
            Expr::Const {
                val: DataValue::from(f),
                span,
            }
        }
        Rule::null => Expr::Const {
            val: DataValue::Null,
            span,
        },
        Rule::boolean => Expr::Const {
            val: DataValue::Bool(s == "true"),
            span,
        },
        Rule::quoted_string | Rule::s_quoted_string | Rule::raw_string => {
            let s = parse_string(p)?;
            Expr::Const {
                val: DataValue::Str(s),
                span,
            }
        }
        Rule::list | Rule::tx_list => {
            let mut collected = vec![];
            for p in p.into_inner() {
                collected.push(build_expr(p, param_pool)?)
            }
            Expr::Apply {
                op: &OP_LIST,
                args: collected.into(),
                span,
            }
        }
        Rule::apply => {
            let mut p = p.into_inner();
            let ident_p = p.next().unwrap();
            let ident = ident_p.as_str();
            let mut args: Box<_> = p
                .next()
                .unwrap()
                .into_inner()
                .map(|v| build_expr(v, param_pool))
                .try_collect()?;
            #[derive(Error, Diagnostic, Debug)]
            #[error("Named function '{0}' not found")]
            #[diagnostic(code(parser::func_not_function))]
            struct FuncNotFoundError(String, #[label] SourceSpan);

            let op = get_op(ident)
                .ok_or_else(|| FuncNotFoundError(ident.to_string(), ident_p.extract_span()))?;
            op.post_process_args(&mut args);

            #[derive(Error, Diagnostic, Debug)]
            #[error("Wrong number of arguments for function '{0}'")]
            #[diagnostic(code(parser::func_wrong_num_args))]
            struct WrongNumArgsError(String, #[label] SourceSpan, #[help] String);

            if op.vararg {
                ensure!(
                    op.min_arity <= args.len(),
                    WrongNumArgsError(
                        ident.to_string(),
                        span,
                        format!("Need at least {} argument(s)", op.min_arity)
                    )
                );
            } else {
                ensure!(
                    op.min_arity == args.len(),
                    WrongNumArgsError(
                        ident.to_string(),
                        span,
                        format!("Need exactly {} argument(s)", op.min_arity)
                    )
                );
            }
            Expr::Apply {
                op,
                args: args.into(),
                span,
            }
        }
        Rule::grouping => build_expr(p.into_inner().next().unwrap(), param_pool)?,
        r => unreachable!("Encountered unknown op {:?}", r),
    })
}

pub(crate) fn parse_int(s: &str, radix: u32) -> i64 {
    i64::from_str_radix(&s[2..].replace('_', ""), radix).unwrap()
}

pub(crate) fn parse_string(pair: Pair<'_>) -> Result<SmartString<LazyCompact>> {
    match pair.as_rule() {
        Rule::quoted_string => Ok(parse_quoted_string(pair)?),
        Rule::s_quoted_string => Ok(parse_s_quoted_string(pair)?),
        Rule::raw_string => Ok(parse_raw_string(pair)?),
        Rule::ident => Ok(SmartString::from(pair.as_str())),
        t => unreachable!("{:?}", t),
    }
}

#[derive(Error, Diagnostic, Debug)]
#[error("invalid UTF8 code {0}")]
#[diagnostic(code(parser::invalid_utf8_code))]
struct InvalidUtf8Error(u32, #[label] SourceSpan);

#[derive(Error, Diagnostic, Debug)]
#[error("invalid escape sequence {0}")]
#[diagnostic(code(parser::invalid_escape_seq))]
struct InvalidEscapeSeqError(String, #[label] SourceSpan);

fn parse_quoted_string(pair: Pair<'_>) -> Result<SmartString<LazyCompact>> {
    let pairs = pair.into_inner().next().unwrap().into_inner();
    let mut ret = SmartString::new();
    for pair in pairs {
        let s = pair.as_str();
        match s {
            r#"\""# => ret.push('"'),
            r"\\" => ret.push('\\'),
            r"\/" => ret.push('/'),
            r"\b" => ret.push('\x08'),
            r"\f" => ret.push('\x0c'),
            r"\n" => ret.push('\n'),
            r"\r" => ret.push('\r'),
            r"\t" => ret.push('\t'),
            s if s.starts_with(r"\u") => {
                let code = parse_int(s, 16) as u32;
                let ch = char::from_u32(code)
                    .ok_or_else(|| InvalidUtf8Error(code, pair.extract_span()))?;
                ret.push(ch);
            }
            s if s.starts_with('\\') => {
                bail!(InvalidEscapeSeqError(s.to_string(), pair.extract_span()))
            }
            s => ret.push_str(s),
        }
    }
    Ok(ret)
}

fn parse_s_quoted_string(pair: Pair<'_>) -> Result<SmartString<LazyCompact>> {
    let pairs = pair.into_inner().next().unwrap().into_inner();
    let mut ret = SmartString::new();
    for pair in pairs {
        let s = pair.as_str();
        match s {
            r#"\'"# => ret.push('\''),
            r"\\" => ret.push('\\'),
            r"\/" => ret.push('/'),
            r"\b" => ret.push('\x08'),
            r"\f" => ret.push('\x0c'),
            r"\n" => ret.push('\n'),
            r"\r" => ret.push('\r'),
            r"\t" => ret.push('\t'),
            s if s.starts_with(r"\u") => {
                let code = parse_int(s, 16) as u32;
                let ch = char::from_u32(code)
                    .ok_or_else(|| InvalidUtf8Error(code, pair.extract_span()))?;
                ret.push(ch);
            }
            s if s.starts_with('\\') => {
                bail!(InvalidEscapeSeqError(s.to_string(), pair.extract_span()))
            }
            s => ret.push_str(s),
        }
    }
    Ok(ret)
}

fn parse_raw_string(pair: Pair<'_>) -> Result<SmartString<LazyCompact>> {
    Ok(SmartString::from(
        pair.into_inner().into_iter().next().unwrap().as_str(),
    ))
}
