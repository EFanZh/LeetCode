pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut iter = nums.iter().copied();

        iter.next().is_some_and(|first| {
            let Some(mut prev) = iter.next().filter(|&second| second > first) else {
                return false;
            };

            loop {
                let Some(value) = iter.next() else { return false };
                let cmp = value.cmp(&prev);

                if cmp == Ordering::Equal {
                    return false;
                }

                prev = value;

                if cmp == Ordering::Less {
                    break;
                }
            }

            loop {
                let Some(value) = iter.next() else { return false };
                let cmp = value.cmp(&prev);

                if cmp == Ordering::Equal {
                    return false;
                }

                prev = value;

                if cmp == Ordering::Greater {
                    break;
                }
            }

            iter.all(|value| {
                let result = value > prev;

                if result {
                    prev = value;
                }

                result
            })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_trionic(nums: Vec<i32>) -> bool {
        Self::is_trionic(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
