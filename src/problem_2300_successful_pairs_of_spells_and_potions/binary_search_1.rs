pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut spells = spells;
        let mut potions = potions;
        let success = success as u64;

        potions.sort_unstable_by_key(|&x| Reverse(x as u32));

        for spell in &mut spells {
            let spell_strength = u64::from(*spell as u32);

            *spell = potions.partition_point(|&x| spell_strength * u64::from(x as u32) >= success) as _;
        }

        spells
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        Self::successful_pairs(spells, potions, success)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
