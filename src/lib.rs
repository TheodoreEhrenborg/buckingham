// Plan
// Need to parse "3 m/s" to (3.0, ['m'], ["s"])
// Start with floats, then maybe polymorphic
// Then need routines for +-/*
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
#[derive(Debug, Clone, PartialEq)]
struct Unit {
    x: f64,
    units: HashMap<String, i64>,
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

fn combine(units1: HashMap<String, i64>, units2: HashMap<String, i64>) -> HashMap<String, i64> {
    let keys1 = units1.keys().collect::<HashSet<&String>>();
    let keys2 = units2.keys().collect::<HashSet<&String>>();
    let all_keys = keys1.union(&keys2);
    let mut result = HashMap::new();
    for key in all_keys {
        let new_value =
            units1.get(&key.to_string()).unwrap_or(&0) + units2.get(&key.to_string()).unwrap_or(&0);
        result.insert(key.to_string(), new_value);
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
}
