pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut rolls = rolls;
        let mean = mean as u32;
        let n = n as u32;
        let total = mean * (rolls.len() as u32 + n);
        let rest = total - rolls.iter().sum::<i32>() as u32;
        let value = rest / n;
        let count_2 = rest % n;

        rolls.clear();
        rolls.reserve_exact(n as usize);

        if value != 0 && value < 6 + u32::from(count_2 == 0) {
            let value = value as i32;

            rolls.extend(iter::repeat_n(value, (n - count_2) as _));
            rolls.extend(iter::repeat_n(value + 1, count_2 as _));
        }

        rolls
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        Self::missing_rolls(rolls, mean, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
