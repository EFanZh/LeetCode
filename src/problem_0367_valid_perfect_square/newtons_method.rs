pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as u64;
        let mut x = num;

        loop {
            match (x * x).cmp(&num) {
                Ordering::Less => return false,
                Ordering::Equal => return true,
                Ordering::Greater => x = (x + num / x) / 2,
            }
        }
    }
}

impl super::Solution for Solution {
    fn is_perfect_square(num: i32) -> bool {
        Self::is_perfect_square(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
