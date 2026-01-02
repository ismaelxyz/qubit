use pest::Parser;
use pest_derive::Parser;

use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::*;

use crate::convert_chart::{UnitType, convert};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(modulus, Left))
            .op(Op::infix(power, Right))
            .op(Op::infix(percentOf, Left) | Op::infix(percentOn, Left))
            .op(Op::infix(rightShift, Right) | Op::infix(leftShift, Right))
    };
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PRATT_PARSER
        .map_primary(|pair: Pair<Rule>| match pair.as_rule() {
            Rule::convert => {
                let mut i = pair.into_inner();
                let value = i.next().unwrap().as_str().parse::<f64>().unwrap();
                // Try to figure out rule name for the conversion between units
                // weight = kilo to gram
                // length = kilometer to meter
                let si_unit_type = i
                    .clone()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let from = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let to = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();

                if let (Ok(from), Ok(to)) = (
                    format!("{:?}::{:?}", si_unit_type, from).parse::<UnitType>(),
                    format!("{:?}::{:?}", si_unit_type, to).parse::<UnitType>(),
                ) {
                    convert(value, from, to)
                } else {
                    f64::NAN
                }
            }
            Rule::function => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                let value = eval(i);
                apply_fun(name, value)
            }
            Rule::pi => std::f64::consts::PI,
            Rule::e => std::f64::consts::E,
            Rule::tau => std::f64::consts::TAU,
            Rule::num => pair.as_str().trim().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => f64::NAN,
        })
        .map_infix(|lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            Rule::percentOf => percent_of(lhs, rhs),
            Rule::percentOn => percent_on(lhs, rhs),
            Rule::rightShift => (lhs as i64 >> rhs as i64) as f64,
            Rule::leftShift => ((lhs as i64) << rhs as i64) as f64,
            Rule::modulus => lhs % rhs,
            _ => f64::NAN,
        })
        .parse(expression)
}

fn percent_on(a: f64, b: f64) -> f64 {
    a / 100_f64 * b + b
}

fn percent_of(a: f64, b: f64) -> f64 {
    a / 100_f64 * b
}

fn apply_fun(name: &str, arg: f64) -> f64 {
    match name {
        "sin" => arg.to_radians().sin(),
        "cos" => arg.to_radians().cos(),
        "tan" => arg.to_radians().tan(),
        "asin" => arg.asin(),
        "acos" => arg.acos(),
        "atan" => arg.atan(),
        "sinh" => arg.sinh(),
        "cosh" => arg.cosh(),
        "tanh" => arg.tanh(),
        "asinh" => arg.asinh(),
        "acosh" => arg.acosh(),
        "atanh" => arg.atanh(),
        "log" => arg.log10(),
        "sqrt" => arg.sqrt(),
        "cbrt" => arg.cbrt(),
        "round" => arg.round(),
        "ceil" => arg.ceil(),
        "floor" => arg.floor(),
        _ => f64::NAN,
    }
}

pub fn parse(input: &str) -> f64 {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(mut pairs) => {
            let Some(pair) = pairs.next() else {
                return f64::NAN;
            };

            match pair.as_rule() {
                // Feed the Pratt parser the actual expression token stream.
                Rule::calculation | Rule::stmt => eval(pair.into_inner()),
                Rule::expr => eval(pair.into_inner()),
                Rule::assign => {
                    let mut inner = pair.into_inner();
                    let _ident = inner.next();
                    // expr
                    match inner.next() {
                        Some(expr_pair) => eval(expr_pair.into_inner()),
                        None => f64::NAN,
                    }
                }
                _ => eval(pair.into_inner()),
            }
        }
        Err(_) => f64::NAN,
    }
}
