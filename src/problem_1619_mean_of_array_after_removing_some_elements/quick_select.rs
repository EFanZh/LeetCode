pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        let split = arr.len() / 20 - 1;

        let remaining = arr
            .select_nth_unstable(split)
            .2
            .select_nth_unstable_by_key(split, |&value| Reverse(value))
            .2;

        #[expect(clippy::cast_precision_loss, reason = "optimal")]
        let result = f64::from(remaining.iter().sum::<i32>()) / remaining.len() as f64;

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn trim_mean(arr: Vec<i32>) -> f64 {
        Self::trim_mean(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
