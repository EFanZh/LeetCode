pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn peak_valley_difference(nums: Vec<i32>) -> i32 {
        let mut diff = 0;

        nums.windows(3).for_each(|window| {
            let [left, middle, right] = window.try_into().unwrap();

            match left.cmp(&middle) {
                Ordering::Less => {
                    if right < middle {
                        diff += middle;
                    }
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    if middle < right {
                        diff -= middle;
                    }
                }
            }
        });

        diff
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn peak_valley_difference(nums: Vec<i32>) -> i32 {
        Self::peak_valley_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
