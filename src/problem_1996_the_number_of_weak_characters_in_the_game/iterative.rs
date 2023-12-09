pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties
            .into_iter()
            .map(|property| {
                let [attack, defense]: [_; 2] = property.try_into().ok().unwrap();

                (attack as u32, defense as u32)
            })
            .collect::<Box<_>>();

        properties.sort_unstable_by_key(|&(attack, defense)| (Reverse(attack), defense));

        let mut result = 0;
        let mut max_defense = 0;

        for &(_, defense) in &*properties {
            if defense < max_defense {
                result += 1;
            }

            max_defense = max_defense.max(defense);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        Self::number_of_weak_characters(properties)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
