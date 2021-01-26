pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut left = 0;
        let mut right = x.min(46340) + 1;

        while right - left > 1 {
            let middle = (left + right) / 2;

            match (middle * middle).cmp(&x) {
                Ordering::Less => left = middle,
                Ordering::Equal => return middle as _,
                Ordering::Greater => right = middle,
            }
        }

        left as _
    }
}

impl super::Solution for Solution {
    fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
