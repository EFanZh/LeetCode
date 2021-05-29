pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        nums.split_first().map_or(false, |(&first, rest)| {
            let mut min = first;
            let mut stack = Vec::with_capacity(nums.len() / 2);

            for &num in rest {
                loop {
                    if let Some((top_low, top_high)) = stack.last() {
                        match num.cmp(top_low) {
                            Ordering::Less => {
                                match num.cmp(&min) {
                                    Ordering::Less => min = num,
                                    Ordering::Equal => {}
                                    Ordering::Greater => {
                                        if num > min + 1 {
                                            stack.push((min, num));
                                        }
                                    }
                                }

                                break;
                            }
                            Ordering::Equal => break,
                            Ordering::Greater => {
                                if num < *top_high {
                                    return true;
                                }

                                stack.pop();
                            }
                        }
                    } else {
                        match num.cmp(&min) {
                            Ordering::Less => min = num,
                            Ordering::Equal => {}
                            Ordering::Greater => {
                                if num > min + 1 {
                                    stack.push((min, num));
                                }
                            }
                        }

                        break;
                    }
                }
            }

            false
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find132pattern(nums: Vec<i32>) -> bool {
        Self::find132pattern(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
