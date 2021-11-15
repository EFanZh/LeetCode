pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut iter = arr.iter().zip(&arr[1..]).map(|(lhs, rhs)| rhs.cmp(lhs));
        let mut length = 1;
        let mut result = 0;

        loop {
            // Going up.

            loop {
                if let Some(ordering) = iter.next() {
                    match ordering {
                        Ordering::Less => {
                            if length > 1 {
                                length += 1;

                                break;
                            }
                        }
                        Ordering::Equal => length = 1,
                        Ordering::Greater => length += 1,
                    }
                } else {
                    return result;
                }
            }

            // Going down.

            loop {
                if let Some(ordering) = iter.next() {
                    match ordering {
                        Ordering::Less => length += 1,
                        Ordering::Equal => {
                            result = result.max(length);
                            length = 1;

                            break;
                        }
                        Ordering::Greater => {
                            result = result.max(length);
                            length = 2;

                            break;
                        }
                    }
                } else {
                    return result.max(length);
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_mountain(arr: Vec<i32>) -> i32 {
        Self::longest_mountain(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
