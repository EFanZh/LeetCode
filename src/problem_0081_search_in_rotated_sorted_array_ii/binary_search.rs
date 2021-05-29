pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if let Some((first, mut rest)) = nums.split_first() {
            if *first == target {
                true
            } else {
                while let Some((last, new_rest)) = rest.split_last() {
                    if last == first {
                        rest = new_rest;
                    } else if *last == target {
                        return true;
                    } else {
                        rest = new_rest;

                        break;
                    }
                }

                if target < *first {
                    rest.binary_search_by(|value| {
                        if value < first {
                            value.cmp(&target)
                        } else {
                            Ordering::Less
                        }
                    })
                } else {
                    rest.binary_search_by(|value| {
                        if value < first {
                            Ordering::Greater
                        } else {
                            value.cmp(&target)
                        }
                    })
                }
                .is_ok()
            }
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::search(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
