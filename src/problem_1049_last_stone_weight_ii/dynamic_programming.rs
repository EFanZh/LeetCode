pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut visited = HashSet::new();
        let mut values = vec![0];
        let mut sum = 0;

        for stone in stones {
            let stone = stone as u16;

            sum += stone;

            for i in 0..values.len() {
                let candidate = values[i] + stone;

                if visited.insert(candidate) {
                    values.push(candidate);
                }
            }
        }

        i32::from(
            values
                .into_iter()
                .map(|value| value.abs_diff(sum - value))
                .min()
                .unwrap(),
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        Self::last_stone_weight_ii(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
