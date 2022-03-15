pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut source_iter = nums.into_iter().map(|x| x * x);
        let mut target_iter = result.iter_mut().rev();

        'outer: while let Some(mut left) = source_iter.next() {
            'left: loop {
                if let Some(right) = source_iter.next_back() {
                    loop {
                        match left.cmp(&right) {
                            Ordering::Less => {
                                *target_iter.next().unwrap() = right;

                                continue 'left;
                            }
                            Ordering::Equal => {
                                *target_iter.next().unwrap() = left;
                                *target_iter.next().unwrap() = left;

                                continue 'outer;
                            }
                            Ordering::Greater => {
                                *target_iter.next().unwrap() = left;

                                if let Some(next_left) = source_iter.next() {
                                    left = next_left;
                                } else {
                                    *target_iter.next().unwrap() = right;

                                    break 'outer;
                                }
                            }
                        }
                    }
                } else {
                    *target_iter.next().unwrap() = left;

                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        Self::sorted_squares(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
