pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut iter = height.iter().map(|&x| x as u32);
        let mut result = 0;
        let mut left_max = 0;
        let mut right_max = 0;

        'outer: while let Some(left) = iter.next() {
            if left <= left_max {
                result += left_max - left;
            } else {
                left_max = left;

                while right_max < left_max {
                    loop {
                        if let Some(right) = iter.next_back() {
                            if right <= right_max {
                                result += right_max - right;
                            } else {
                                right_max = right;

                                break;
                            }
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn trap(height: Vec<i32>) -> i32 {
        Self::trap(height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
