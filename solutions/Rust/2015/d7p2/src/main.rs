use std::collections::HashMap;

use aoc_util::hook_solution;
use nom::ascii;
use nom::ascii::{dec_uint, space1};
use nom::combinator::{self, preceded, terminated};
use nom::{prelude::*, seq};
use winnow as nom;

type Reg = u16;

trait Evaluate<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg;
}

fn value_parser<'input>() -> impl Parser<&'input str, Value<'input>, ()> {
    let ident_parser = ascii::alpha1.map(Value::Ident);
    combinator::alt((dec_uint.map(Value::Constant), ident_parser))
}

use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Value<'input> {
    Constant(Reg),
    Ident(&'input str),
}

impl<'a> Evaluate<'a> for Value<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg {
        use Value::*;
        match self {
            Constant(r) => r,
            Ident(id) => {
                let res = state.0[id].eval(state);
                state.0.insert(id, EvalExpr::Computed(res));
                res
            }
        }
    }
}

fn bop_parser<'input>() -> impl Parser<&'input str, BinaryOperation<'input>, ()> {
    let mut kind_parser = ascii::alpha1.parse_to::<BinaryOperationKind>();
    combinator::seq!(
        value_parser(),
        preceded(space1, kind_parser.by_ref()),
        preceded(space1, value_parser()),
    )
    .map(|(lhs, kind, rhs)| BinaryOperation { lhs, kind, rhs })
}

#[derive(Debug, Clone, Copy)]
struct BinaryOperation<'input> {
    lhs: Value<'input>,
    kind: BinaryOperationKind,
    rhs: Value<'input>,
}

impl<'a> Evaluate<'a> for BinaryOperation<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg {
        let Self { lhs, kind, rhs } = self;
        let lhs = lhs.eval(state);
        let rhs = rhs.eval(state);
        use BinaryOperationKind::*;
        match kind {
            And => lhs & rhs,
            Or => lhs | rhs,
            LShift => lhs << rhs,
            RShift => lhs >> rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
enum BinaryOperationKind {
    #[strum(ascii_case_insensitive)]
    And,
    #[strum(ascii_case_insensitive)]
    Or,
    #[strum(ascii_case_insensitive)]
    LShift,
    #[strum(ascii_case_insensitive)]
    RShift,
}

fn uop_parser<'input>() -> impl Parser<&'input str, UnaryOperation<'input>, ()> {
    let kind_parser = ascii::alpha1.parse_to::<UnaryOperationKind>();
    combinator::separated_pair(kind_parser, space1, value_parser())
        .map(|(kind, operand)| UnaryOperation { kind, operand })
}

#[derive(Debug, Clone, Copy)]
struct UnaryOperation<'input> {
    operand: Value<'input>,
    kind: UnaryOperationKind,
}

impl<'a> Evaluate<'a> for UnaryOperation<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg {
        let Self { operand, kind } = self;
        let operand = operand.eval(state);
        use UnaryOperationKind::*;
        match kind {
            Not => !operand,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumString)]
enum UnaryOperationKind {
    #[strum(ascii_case_insensitive)]
    Not,
}

fn expr_parser<'input>() -> impl Parser<&'input str, Expr<'input>, ()> {
    let uop = uop_parser().map(Expr::UOp);
    let bop = bop_parser().map(Expr::BOp);
    let value = value_parser().map(Expr::Value);
    combinator::alt((uop, bop, value))
}

impl<'a> Evaluate<'a> for Expr<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg {
        use Expr::*;
        match self {
            BOp(op) => op.eval(state),
            UOp(op) => op.eval(state),
            Value(v) => v.eval(state),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Expr<'input> {
    BOp(BinaryOperation<'input>),
    UOp(UnaryOperation<'input>),
    Value(Value<'input>),
}

fn statement_parser<'input>() -> impl Parser<&'input str, Statement<'input>, ()> {
    let mut expr = terminated(expr_parser(), combinator::delimited(space1, "->", space1));
    seq!(expr.by_ref(), ascii::alpha1).map(|(expr, output)| Statement { expr, output })
}

#[derive(Debug, Clone, Copy)]
struct Statement<'input> {
    expr: Expr<'input>,
    output: &'input str,
}

#[derive(Debug, Clone, Copy)]
enum EvalExpr<'input> {
    Unknown(Expr<'input>),
    Computed(Reg),
}

impl<'a> Evaluate<'a> for EvalExpr<'a> {
    fn eval(self, state: &mut State<'a>) -> Reg {
        use EvalExpr::*;
        match self {
            Computed(r) => r,
            Unknown(expr) => expr.eval(state),
        }
    }
}

fn state_parser<'input>() -> impl Parser<&'input str, State<'input>, ()> {
    let line = terminated(statement_parser(), combinator::opt(ascii::line_ending));
    nom::combinator::repeat(.., line).fold(
        || State(HashMap::new()),
        |mut state, Statement { expr, output }| {
            if let Some(conflict) = state.0.insert(output, EvalExpr::Unknown(expr)) {
                panic!("Conflicting value for output {output}: would overwrite {conflict:?}")
            }
            state
        },
    )
}

#[derive(Debug, Clone)]
struct State<'input>(HashMap<&'input str, EvalExpr<'input>>);

fn solution(input: &str) -> u16 {
    let mut state = state_parser().parse(input).unwrap();
    let original = state.clone();
    let a = state.0["a"];
    let part1 = a.eval(&mut state);
    state = original;
    state.0.insert("b", EvalExpr::Computed(part1));
    let a = state.0["a"];
    a.eval(&mut state)
}

fn main() {
    hook_solution(solution)
}
