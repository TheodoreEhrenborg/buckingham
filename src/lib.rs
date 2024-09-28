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
// How to combine two units
// OK, I think I want each one to be a HashMap(str -> int)
// How do I combine the hashmaps?
// OK, get a set of the keys from each
// Then take the union of that
// Then iterate over the keys, taking the values (or 0) from each of the dictionaries,
// and adding them
// Finally, iterate over the hashmap and remove any with 0 value

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
