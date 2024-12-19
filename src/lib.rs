// Plan
// Need to parse "3 m/s" to (3.0, ['m'], ["s"])
// Start with floats, then maybe polymorphic
// Then need routines for +-/*
use nom::branch::alt;
use nom::character::complete::alpha1;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::Mul;

use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::number::complete::float;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    x: f32,
    units: HashMap<String, i32>,
}

impl Add for Unit {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        if self.units == other.units {
            Ok(Self {
                x: self.x + other.x,
                units: self.units,
            })
        } else {
            Err("Units don't match".to_string())
        }
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let new_x = self.x * rhs.x;
        let new_units = combine(self.units, rhs.units);
        Self {
            x: new_x,
            units: new_units,
        }
    }
}

fn combine(units1: HashMap<String, i32>, units2: HashMap<String, i32>) -> HashMap<String, i32> {
    let keys1 = units1.keys().collect::<HashSet<&String>>();
    let keys2 = units2.keys().collect::<HashSet<&String>>();
    let all_keys = keys1.union(&keys2);
    let mut result = HashMap::new();
    for key in all_keys {
        let new_value =
            units1.get(&key.to_string()).unwrap_or(&0) + units2.get(&key.to_string()).unwrap_or(&0);
        if new_value != 0 {
            result.insert(key.to_string(), new_value);
        }
    }
    result
}

// How to combine two units
// OK, I think I want each one to be a HashMap(str -> int)
// How do I combine the hashmaps?
// OK, get a set of the keys from each
// Then take the union of that
// Then iterate over the keys, taking the values (or 0) from each of the dictionaries,
// and adding them
// Finally, iterate over the hashmap and remove any with 0 value

// How to parse:
// Format:
// Float space unit(^optional_i32) unit(^optional_i32) / unit(^optional_i32)
// And the division part is optional
// 4.5 m^2 kg / s GBP

// TODO What if the user specifies the same unit twice in the unparsed string?

fn parse_unit_and_exp(input: &str) -> IResult<&str, (&str, i32)> {
    separated_pair(alpha1, tag("^"), i32)(input)
}

fn parse_unit_and_default_exp(input: &str) -> IResult<&str, (&str, i32)> {
    let (remaining, unit_name) = alpha1(input)?;
    Ok((remaining, (unit_name, 1)))
}

fn parse_unit_and_maybe_exp(input: &str) -> IResult<&str, (&str, i32)> {
    alt((parse_unit_and_exp, parse_unit_and_default_exp))(input)
}

fn parse_full_expression(input: &str) -> IResult<&str, (f32, Vec<(&str, i32)>)> {
    all_consuming(tuple((
        float,
        many0(preceded(multispace1, parse_unit_and_maybe_exp)),
    )))(input)
}

fn build_unit(input: (f32, Vec<(&str, i32)>)) -> Unit {
    Unit {
        x: input.0,
        units: input
            .1
            .into_iter()
            .map(|(x, y)| (String::from(x), y))
            .collect::<HashMap<_, _>>(),
    }
}

pub fn u(input: &str) -> Result<Unit, Box<dyn std::error::Error + '_>> {
    let (_, unpacked) = parse_full_expression(input)?;
    Ok(build_unit(unpacked))
}

// TODO Support division?

// TODO Test that default exponents work in the middle of a string
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add() {
        assert_eq!(
            Unit {
                x: 1.,
                units: HashMap::from([("m".to_string(), 1)])
            } + Unit {
                x: 2.,
                units: HashMap::from([("m".to_string(), 1)])
            },
            Ok(Unit {
                x: 3.,
                units: HashMap::from([("m".to_string(), 1)])
            })
        );
    }

    #[test]
    fn combine_works() {
        assert_eq!(
            combine(
                HashMap::from([("m".to_string(), 2), ("s".to_string(), -2),]),
                HashMap::from([("m".to_string(), 1), ("s".to_string(), 1),])
            ),
            HashMap::from([("m".to_string(), 3), ("s".to_string(), -1),])
        )
    }
    #[test]
    fn unit_and_exp_works() {
        let (remaining, parsed) = parse_unit_and_exp("meters^2").unwrap();
        assert_eq!(parsed, ("meters", 2));
        assert_eq!(remaining, "");
    }

    #[test]
    fn unit_and_exp1_works() {
        let (remaining, parsed) = parse_unit_and_default_exp("meters").unwrap();
        assert_eq!(parsed, ("meters", 1));
        assert_eq!(remaining, "");
    }

    #[test]
    fn unit_and_exp_maybe_works() {
        let (remaining, parsed) = parse_unit_and_maybe_exp("meters").unwrap();
        assert_eq!(parsed, ("meters", 1));
        assert_eq!(remaining, "");
        let (remaining, parsed) = parse_unit_and_maybe_exp("meters^-4").unwrap();
        assert_eq!(parsed, ("meters", -4));
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_full_expression_works() {
        let (remaining, parsed) = parse_full_expression("5 meters^2 seconds").unwrap();
        assert_eq!(parsed, (5.0, vec![("meters", 2), ("seconds", 1)]));
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_and_build() {
        let (remaining, parsed) = parse_full_expression("5 meters^2 seconds^-1").unwrap();
        assert_eq!(remaining, "");
        let u = build_unit(parsed);
        assert_eq!(
            u,
            Unit {
                x: 5.,
                units: HashMap::from([("meters".to_string(), 2), ("seconds".to_string(), -1)])
            }
        );
    }
    #[test]
    fn no_remaining() {
        let result = parse_full_expression("5 meters^2 seconds^-1 ");
        assert!(result.is_err());
    }
    #[test]
    fn parse_and_convert() {
        let u = u("5 meters^2 seconds^-1").unwrap();
        assert_eq!(
            u,
            Unit {
                x: 5.,
                units: HashMap::from([("meters".to_string(), 2), ("seconds".to_string(), -1)])
            }
        );
    }
    #[test]
    fn basic_mul() {
        let u1 = u("5 meters^2 seconds^-1").unwrap();
        let u2 = u("3 meters^-1 kg").unwrap();
        let u3 = u("15 kg^1 meters seconds^-1").unwrap();
        assert_eq!(u1 * u2, u3);
    }
    #[test]
    fn cancel() {
        let u1 = u("-5 meters^2 seconds^-1").unwrap();
        let u2 = u("4.1 meters^-2").unwrap();
        let u3 = u("-20.5 seconds^-1").unwrap();
        assert_eq!(u1 * u2, u3);
    }
}
