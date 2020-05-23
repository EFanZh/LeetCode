pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut start = 1;
        let mut count = x;

        while count != 0 {
            let half = count / 2;
            let middle = start + half;

            match middle.saturating_mul(middle).cmp(&x) {
                Ordering::Less => {
                    start = middle + 1;
                    count -= half + 1;
                }
                Ordering::Equal => return middle as _,
                Ordering::Greater => {
                    count = half;
                }
            }
        }

        (start - 1) as _
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
