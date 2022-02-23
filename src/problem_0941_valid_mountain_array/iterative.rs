pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if let [first, second, ref rest @ ..] = *arr {
            if first < second {
                let mut prev = second;
                let mut iter = rest.iter().copied();

                loop {
                    if let Some(num) = iter.next() {
                        match num.cmp(&prev) {
                            Ordering::Less => {
                                prev = num;

                                break;
                            }
                            Ordering::Equal => return false,
                            Ordering::Greater => prev = num,
                        }
                    } else {
                        return false;
                    }
                }

                for num in iter {
                    if num < prev {
                        prev = num;
                    } else {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_mountain_array(arr: Vec<i32>) -> bool {
        Self::valid_mountain_array(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
