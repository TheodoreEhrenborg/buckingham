// Plan
// Need to parse "3 m/s" to (3.0, ['m'], ["s"])
// Start with floats, then maybe polymorphic
// Then need routines for +-/*
use std::ops::Add;
#[derive(Debug, Clone, PartialEq)]
struct Unit {
    x: f64,
    upper: Vec<String>
}

impl Add for Unit {
    type Output = Result<Self,String>;

    fn add(self, other: Self) -> Result<Self,String> {
        Ok(Self {
            x: self.x + other.x,
            upper: vec![]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Unit { x: 1., upper: vec![] } + Unit { x: 2., upper: vec![] },
            Ok(Unit { x: 3., upper: vec![] })
        );
    }
}
