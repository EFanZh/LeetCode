pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut stone_value = stone_value;
        let mut prev_1 = 0;
        let mut prev_2 = 0;
        let mut cache_1 = 0;
        let mut cache_2 = 0;
        let mut cache_3 = 0;

        while let Some(value) = stone_value.pop() {
            let extra = value - cache_3.min(cache_2 - prev_2).min(cache_1 - (prev_1 + prev_2));

            prev_1 = prev_2;
            prev_2 = value;
            cache_1 = cache_2;
            cache_2 = cache_3;
            cache_3 = extra;
        }

        String::from(match cache_3.cmp(&0) {
            Ordering::Less => "Bob",
            Ordering::Equal => "Tie",
            Ordering::Greater => "Alice",
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_iii(stone_value: Vec<i32>) -> String {
        Self::stone_game_iii(stone_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
