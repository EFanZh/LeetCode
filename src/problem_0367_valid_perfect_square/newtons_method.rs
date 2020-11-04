pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            true
        } else {
            let mut left = 1;
            let mut right = num.min(46340);

            loop {
                let middle = (left + right) / 2;

                match (middle * middle).cmp(&num) {
                    Ordering::Less => left = middle + 1,
                    Ordering::Equal => return true,
                    Ordering::Greater => right = middle,
                }

                if left == right {
                    return false;
                }
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
