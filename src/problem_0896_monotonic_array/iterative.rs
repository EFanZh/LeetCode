pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.split_first().map_or(true, |(first, rest)| {
            let mut iter = rest.iter().copied();

            while let Some(num) = iter.next() {
                match num.cmp(first) {
                    Ordering::Less => {
                        let mut prev = num;

                        for num in iter {
                            if num > prev {
                                return false;
                            }

                            prev = num;
                        }

                        break;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        let mut prev = num;

                        for num in iter {
                            if num < prev {
                                return false;
                            }

                            prev = num;
                        }

                        break;
                    }
                }
            }

            true
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_monotonic(nums: Vec<i32>) -> bool {
        Self::is_monotonic(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
