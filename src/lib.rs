// Plan
// Need to parse "3 m/s" to (3.0, ['m'], ["s"])
// Start with floats, then maybe polymorphic
// Then need routines for +-/*
use std::ops::Add;
#[derive(Debug, Clone, PartialEq)]
struct Unit {
    x: f64,
    upper: Vec<String>,
    lower: Vec<String>
}

impl Add for Unit {
    type Output = Result<Self,String>;

    fn add(self, other: Self) -> Result<Self,String> {
        if self.upper == other.upper && self.lower == other.lower {
            Ok(Self {
                x: self.x + other.x,
                upper: self.upper,
                lower: self.lower,
            })
        }
        else {
            Err("Units don't match".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Unit { x: 1., upper: vec![], lower: vec![] } + Unit { x: 2., upper: vec![], lower: vec![] },
            Ok(Unit { x: 3., upper: vec![] , lower: vec![]})
        );
    }
}
